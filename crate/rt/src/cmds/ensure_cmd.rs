use std::{fmt::Debug, marker::PhantomData};

use peace_cfg::{ItemSpecId, OpCtx};
use peace_cmd::{
    ctx::{CmdCtx, CmdCtxView},
    scopes::{SingleProfileSingleFlow, SingleProfileSingleFlowView},
};
use peace_resources::{
    internal::StatesMut,
    paths::{FlowDir, StatesSavedFile},
    resources::ts::{Ensured, EnsuredDry, SetUp},
    states::{States, StatesCurrent, StatesEnsured, StatesEnsuredDry, StatesSaved},
    Resources,
};
use peace_rt_model::{
    cmd_context_params::ParamsKeys,
    outcomes::{ItemEnsureBoxed, ItemEnsurePartialBoxed},
    output::OutputWrite,
    Error, ItemSpecBoxed, ItemSpecGraph, ItemSpecRt, Storage,
};
use tokio::sync::{mpsc, mpsc::UnboundedSender};

use crate::BUFFERED_FUTURES_MAX;

cfg_if::cfg_if! {
    if #[cfg(feature = "output_progress")] {
        use peace_cfg::{
            progress::{
                ProgressComplete,
                ProgressDelta,
                ProgressSender,
                ProgressStatus,
                ProgressUpdate,
                ProgressUpdateAndId,
            },
            OpCheckStatus,
        };
        use peace_rt_model::CmdProgressTracker;
        use tokio::sync::mpsc::Sender;
    }
}

#[derive(Debug)]
pub struct EnsureCmd<E, O, PKeys>(PhantomData<(E, O, PKeys)>);

impl<E, O, PKeys> EnsureCmd<E, O, PKeys>
where
    E: std::error::Error + From<Error> + Send + 'static,
    PKeys: ParamsKeys + 'static,
    O: OutputWrite<E>,
{
    #[cfg(feature = "output_progress")]
    /// Maximum number of progress messages to buffer.
    const PROGRESS_COUNT_MAX: usize = 256;

    /// Conditionally runs [`EnsureOpSpec`]`::`[`exec_dry`] for each
    /// [`ItemSpec`].
    ///
    /// In practice this runs [`EnsureOpSpec::check`], and only runs
    /// [`exec_dry`] if execution is required.
    ///
    /// # Note
    ///
    /// To only make changes when they are *all* likely to work, we execute the
    /// functions as homogeneous groups instead of interleaving the functions
    /// together per `ItemSpec`:
    ///
    /// 1. Run [`EnsureOpSpec::check`] for all `ItemSpec`s.
    /// 2. Run [`EnsureOpSpec::exec_dry`] for all `ItemSpec`s.
    /// 3. Fetch `StatesCurrent` again, and compare.
    ///
    /// State cannot be fetched interleaved with `exec_dry` as it may use
    /// different `Data`.
    ///
    /// [`exec_dry`]: peace_cfg::EnsureOpSpec::exec_dry
    /// [`EnsureOpSpec::check`]: peace_cfg::EnsureOpSpec::check
    /// [`EnsureOpSpec::exec_dry`]: peace_cfg::EnsureOpSpec::exec_dry
    /// [`ItemSpec`]: peace_cfg::ItemSpec
    /// [`EnsureOpSpec`]: peace_cfg::ItemSpec::EnsureOpSpec
    pub async fn exec_dry(
        mut cmd_ctx: CmdCtx<'_, O, SingleProfileSingleFlow<E, PKeys, SetUp>>,
    ) -> Result<CmdCtx<'_, O, SingleProfileSingleFlow<E, PKeys, EnsuredDry>>, E> {
        let CmdCtxView { output, scope, .. } = cmd_ctx.view();
        let SingleProfileSingleFlowView {
            #[cfg(feature = "output_progress")]
            cmd_progress_tracker,
            flow,
            resources,
            ..
        } = scope.view();
        let item_spec_graph = flow.graph();

        let states_ensured_dry = Self::exec_internal(
            item_spec_graph,
            output,
            resources,
            #[cfg(feature = "output_progress")]
            cmd_progress_tracker,
            true,
        )
        .await?;
        let mut cmd_ctx = cmd_ctx.resources_update(|resources| {
            Resources::<EnsuredDry>::from((resources, states_ensured_dry))
        });

        {
            let CmdCtxView { output, scope, .. } = cmd_ctx.view();
            let resources = scope.resources();
            let states_ensured_dry = resources.borrow::<StatesEnsuredDry>();
            output.present(&*states_ensured_dry).await?;
        }

        Ok(cmd_ctx)
    }

    /// Conditionally runs [`EnsureOpSpec`]`::`[`exec`] for each
    /// [`ItemSpec`].
    ///
    /// In practice this runs [`EnsureOpSpec::check`], and only runs
    /// [`exec`] if execution is required.
    ///
    /// # Note
    ///
    /// To only make changes when they are *all* likely to work, we execute the
    /// functions as homogeneous groups instead of interleaving the functions
    /// together per `ItemSpec`:
    ///
    /// 1. Run [`EnsureOpSpec::check`] for all `ItemSpec`s.
    /// 2. Run [`EnsureOpSpec::exec`] for all `ItemSpec`s.
    /// 3. Fetch `StatesCurrent` again, and compare.
    ///
    /// State cannot be fetched interleaved with `exec` as it may use
    /// different `Data`.
    ///
    /// [`exec`]: peace_cfg::EnsureOpSpec::exec
    /// [`EnsureOpSpec::check`]: peace_cfg::EnsureOpSpec::check
    /// [`EnsureOpSpec::exec`]: peace_cfg::EnsureOpSpec::exec
    /// [`ItemSpec`]: peace_cfg::ItemSpec
    /// [`EnsureOpSpec`]: peace_cfg::ItemSpec::EnsureOpSpec
    pub async fn exec(
        mut cmd_ctx: CmdCtx<'_, O, SingleProfileSingleFlow<E, PKeys, SetUp>>,
    ) -> Result<CmdCtx<'_, O, SingleProfileSingleFlow<E, PKeys, Ensured>>, E> {
        let CmdCtxView { output, scope, .. } = cmd_ctx.view();
        let SingleProfileSingleFlowView {
            #[cfg(feature = "output_progress")]
            cmd_progress_tracker,
            flow,
            resources,
            ..
        } = scope.view();
        let item_spec_graph = flow.graph();

        let states_ensured = Self::exec_internal(
            item_spec_graph,
            output,
            resources,
            #[cfg(feature = "output_progress")]
            cmd_progress_tracker,
            false,
        )
        .await?;
        let mut cmd_ctx = cmd_ctx
            .resources_update(|resources| Resources::<Ensured>::from((resources, states_ensured)));

        {
            let CmdCtxView { output, scope, .. } = cmd_ctx.view();
            let resources = scope.resources();
            let states_ensured = resources.borrow::<StatesEnsured>();
            Self::serialize_internal(resources, &states_ensured).await?;
            output.present(&*states_ensured).await?;
        }

        Ok(cmd_ctx)
    }

    /// Conditionally runs [`EnsureOpSpec`]`::`[`exec`] for each [`ItemSpec`].
    ///
    /// Same as [`Self::exec`], but does not change the type state, and returns
    /// [`StatesEnsured`].
    ///
    /// [`exec`]: peace_cfg::EnsureOpSpec::exec
    /// [`ItemSpec`]: peace_cfg::ItemSpec
    /// [`EnsureOpSpec`]: peace_cfg::ItemSpec::EnsureOpSpec
    async fn exec_internal<StatesTs>(
        item_spec_graph: &ItemSpecGraph<E>,
        #[cfg(not(feature = "output_progress"))] _output: &mut O,
        #[cfg(feature = "output_progress")] output: &mut O,
        resources: &mut Resources<SetUp>,
        #[cfg(feature = "output_progress")] cmd_progress_tracker: &mut CmdProgressTracker,
        dry_run: bool,
    ) -> Result<States<StatesTs>, E>
    where
        for<'resources> States<StatesTs>: From<(StatesCurrent, &'resources Resources<SetUp>)>,
    {
        cfg_if::cfg_if! {
            if #[cfg(feature = "output_progress")] {
                output.progress_begin(cmd_progress_tracker).await;

                let CmdProgressTracker {
                    multi_progress: _,
                    progress_trackers,
                } = cmd_progress_tracker;

                let (progress_tx, mut progress_rx) =
                    mpsc::channel::<ProgressUpdateAndId>(Self::PROGRESS_COUNT_MAX);
            }
        }

        // `StatesEnsured` should begin as `StatesSaved`, and be mutated as new states
        // are read.
        let mut states_ensured_mut = if let Ok(states_saved) = resources.try_borrow::<StatesSaved>()
        {
            StatesMut::<StatesTs>::from((*states_saved).clone().into_inner())
        } else {
            StatesMut::<StatesTs>::with_capacity(item_spec_graph.node_count())
        };

        let (outcomes_tx, mut outcomes_rx) = mpsc::unbounded_channel::<ItemEnsureOutcome<E>>();

        let resources_ref = &*resources;
        let execution_task = async move {
            #[cfg(feature = "output_progress")]
            let progress_tx = &progress_tx;
            let outcomes_tx = &outcomes_tx;

            // It would be ideal if we can pass just the `ProgressBar` through
            // to `Self::item_ensure_exec`, and not hold the reference to
            // `progress_trackers` in the closure.
            //
            // This would allow us to hold a `&mut ProgressTracker` when
            // `progress_rx` receives `ProgressUpdateAndId` -- so that we can store
            // `progress_limit` inside `ProgressTracker`.
            //
            // Subsequently we can pass `&ProgressTracker` in
            // `OutputWrite::progress_update`, so that `OutputWrite`
            // implementations such as `CliOutput` can read the limit and adjust the
            // progress bar styling accordingly.
            let (Ok(()) | Err(())) = item_spec_graph
                .try_for_each_concurrent(BUFFERED_FUTURES_MAX, |item_spec| {
                    Self::item_ensure_exec(
                        resources_ref,
                        #[cfg(feature = "output_progress")]
                        progress_tx,
                        outcomes_tx,
                        item_spec,
                        dry_run,
                    )
                })
                .await
                .map_err(|_vec_units: Vec<()>| ());

            // `progress_tx` is dropped here, so `progress_rx` will safely end.
        };

        #[cfg(feature = "output_progress")]
        let progress_render_task = async {
            while let Some(progress_update_and_id) = progress_rx.recv().await {
                let ProgressUpdateAndId {
                    item_spec_id,
                    progress_update,
                } = &progress_update_and_id;

                let Some(progress_tracker) = progress_trackers.get_mut(item_spec_id) else {
                    panic!("Expected `progress_tracker` to exist for item spec: `{item_spec_id}`.");
                };
                match progress_update {
                    ProgressUpdate::Limit(progress_limit) => {
                        progress_tracker.set_progress_limit(*progress_limit);
                        progress_tracker.set_progress_status(ProgressStatus::ExecPending);
                    }
                    ProgressUpdate::Delta(delta) => {
                        match delta {
                            ProgressDelta::Tick => progress_tracker.tick(),
                            ProgressDelta::Inc(unit_count) => progress_tracker.inc(*unit_count),
                        }
                        progress_tracker.set_progress_status(ProgressStatus::Running);
                    }
                    ProgressUpdate::Complete(progress_complete) => {
                        progress_tracker.set_progress_status(ProgressStatus::Complete(
                            progress_complete.clone(),
                        ));
                    }
                }

                output
                    .progress_update(progress_tracker, &progress_update_and_id)
                    .await
            }
        };

        let outcomes_rx_task = async {
            while let Some(outcome) = outcomes_rx.recv().await {
                match outcome {
                    ItemEnsureOutcome::PrepareFail {
                        item_spec_id: _,
                        item_ensure_partial: _,
                        error: _,
                    } => todo!(),
                    ItemEnsureOutcome::Success {
                        item_spec_id,
                        item_ensure,
                    } => {
                        if let Some(state_ensured) = item_ensure.state_ensured() {
                            states_ensured_mut.insert_raw(item_spec_id, state_ensured);
                        } else {
                            // Item was already in the desired state.
                            // No change to saved state.
                        }
                    }
                    ItemEnsureOutcome::Fail {
                        item_spec_id,
                        item_ensure,
                        error: _, // TODO: save to report.
                    } => {
                        if let Some(state_ensured) = item_ensure.state_ensured() {
                            states_ensured_mut.insert_raw(item_spec_id, state_ensured);
                        }
                    }
                }
            }
        };

        cfg_if::cfg_if! {
            if #[cfg(feature = "output_progress")] {
                futures::join!(execution_task, progress_render_task, outcomes_rx_task);

                output.progress_end(cmd_progress_tracker).await;
            } else {
                futures::join!(execution_task, outcomes_rx_task);
            }
        }

        // TODO: Should we run `StatesCurrentFnSpec` again?
        //
        // i.e. is it part of `EnsureOpSpec::exec`'s contract to return the state.
        //
        // * It may be duplication of code.
        // * `FileDownloadItemSpec` needs to know the ETag from the last request, which:
        //     - in `StatesCurrentFnSpec` comes from `StatesSaved`
        //     - in `EnsureCmd` comes from `StatesEnsured`
        // * `ShCmdItemSpec` doesn't return the state in the ensure script, so in the
        //   item spec we run the state current script after the ensure exec script.
        let states_ensured = states_ensured_mut.into();

        Ok(states_ensured)
    }

    ///
    /// # Implementation Note
    ///
    /// Tried passing through the function to execute instead of a `dry_run`
    /// parameter, but couldn't convince the compiler that the lifetimes match
    /// up:
    ///
    /// ```rust,ignore
    /// async fn item_ensure_exec<F, Fut>(
    ///     resources: &Resources<SetUp>,
    ///     outcomes_tx: &UnboundedSender<ItemEnsureOutcome<E>>,
    ///     item_spec: FnRef<'_, ItemSpecBoxed<E>>,
    ///     f: F,
    /// ) -> bool
    /// where
    ///     F: (Fn(&dyn ItemSpecRt<E>, op_ctx: OpCtx<'_>, &Resources<SetUp>, &mut ItemEnsureBoxed) -> Fut) + Copy,
    ///     Fut: Future<Output = Result<(), E>>,
    /// ```
    async fn item_ensure_exec(
        resources: &Resources<SetUp>,
        #[cfg(feature = "output_progress")] progress_tx: &Sender<ProgressUpdateAndId>,
        outcomes_tx: &UnboundedSender<ItemEnsureOutcome<E>>,
        item_spec: &ItemSpecBoxed<E>,
        dry_run: bool,
    ) -> Result<(), ()> {
        let f = if dry_run {
            ItemSpecRt::ensure_exec_dry
        } else {
            ItemSpecRt::ensure_exec
        };
        match item_spec.ensure_prepare(resources).await {
            Ok(mut item_ensure) => {
                let item_spec_id = item_spec.id();
                #[cfg(feature = "output_progress")]
                let progress_sender = {
                    match item_ensure.op_check_status() {
                        #[cfg(not(feature = "output_progress"))]
                        OpCheckStatus::ExecRequired => {}
                        #[cfg(feature = "output_progress")]
                        OpCheckStatus::ExecRequired { progress_limit } => {
                            // Update `OutputWrite`s with progress limit.
                            let _progress_send_unused = progress_tx.try_send(ProgressUpdateAndId {
                                item_spec_id: item_spec_id.clone(),
                                progress_update: ProgressUpdate::Limit(progress_limit),
                            });
                        }
                        OpCheckStatus::ExecNotRequired => {}
                    }

                    ProgressSender::new(item_spec_id, progress_tx)
                };
                let op_ctx = OpCtx::new(
                    item_spec_id,
                    #[cfg(feature = "output_progress")]
                    progress_sender,
                );
                match f(&**item_spec, op_ctx, resources, &mut item_ensure).await {
                    Ok(()) => {
                        // ensure succeeded

                        #[cfg(feature = "output_progress")]
                        let _progress_send_unused = progress_tx.try_send(ProgressUpdateAndId {
                            item_spec_id: item_spec_id.clone(),
                            progress_update: ProgressUpdate::Complete(ProgressComplete::Success),
                        });

                        outcomes_tx
                            .send(ItemEnsureOutcome::Success {
                                item_spec_id: item_spec.id().clone(),
                                item_ensure,
                            })
                            .expect("unreachable: `outcomes_rx` is in a sibling task.");

                        Ok(())
                    }
                    Err(error) => {
                        // ensure failed

                        #[cfg(feature = "output_progress")]
                        let _progress_send_unused = progress_tx.try_send(ProgressUpdateAndId {
                            item_spec_id: item_spec_id.clone(),
                            progress_update: ProgressUpdate::Complete(ProgressComplete::Fail),
                        });

                        outcomes_tx
                            .send(ItemEnsureOutcome::Fail {
                                item_spec_id: item_spec.id().clone(),
                                item_ensure,
                                error,
                            })
                            .expect("unreachable: `outcomes_rx` is in a sibling task.");

                        // we should stop processing.
                        Err(())
                    }
                }
            }
            Err((error, item_ensure_partial)) => {
                #[cfg(feature = "output_progress")]
                let _progress_send_unused = progress_tx.try_send(ProgressUpdateAndId {
                    item_spec_id: item_spec.id().clone(),
                    progress_update: ProgressUpdate::Complete(ProgressComplete::Fail),
                });

                outcomes_tx
                    .send(ItemEnsureOutcome::PrepareFail {
                        item_spec_id: item_spec.id().clone(),
                        item_ensure_partial,
                        error,
                    })
                    .expect("unreachable: `outcomes_rx` is in a sibling task.");

                Err(())
            }
        }
    }

    // TODO: This duplicates a bit of code with `StatesCurrentDiscoverCmd`.
    async fn serialize_internal<TS>(
        resources: &Resources<TS>,
        states_ensured: &StatesEnsured,
    ) -> Result<(), E> {
        use peace_rt_model::StatesSerializer;

        let flow_dir = resources.borrow::<FlowDir>();
        let storage = resources.borrow::<Storage>();
        let states_saved_file = StatesSavedFile::from(&*flow_dir);

        StatesSerializer::serialize(&storage, states_ensured, &states_saved_file).await?;

        drop(flow_dir);
        drop(storage);

        Ok(())
    }
}

impl<E, O, PKeys> Default for EnsureCmd<E, O, PKeys> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

#[allow(dead_code)]
#[derive(Debug)]
enum ItemEnsureOutcome<E> {
    /// Error occurred when discovering current state, desired states, state
    /// diff, or `OpCheckStatus`.
    PrepareFail {
        item_spec_id: ItemSpecId,
        item_ensure_partial: ItemEnsurePartialBoxed,
        error: E,
    },
    /// Ensure execution succeeded.
    Success {
        item_spec_id: ItemSpecId,
        item_ensure: ItemEnsureBoxed,
    },
    /// Ensure execution failed.
    Fail {
        item_spec_id: ItemSpecId,
        item_ensure: ItemEnsureBoxed,
        error: E,
    },
}
