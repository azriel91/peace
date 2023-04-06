use std::{fmt::Debug, marker::PhantomData};

use peace_cfg::{ItemSpecId, OpCheckStatus, OpCtx};
use peace_cmd::{
    ctx::CmdCtx,
    scopes::{SingleProfileSingleFlow, SingleProfileSingleFlowView},
};
use peace_resources::{
    internal::StatesMut,
    paths::{FlowDir, StatesDesiredFile, StatesSavedFile},
    resources::ts::SetUp,
    states::{ts::Desired, States, StatesCurrent, StatesDesired, StatesSaved},
    Resources,
};
use peace_rt_model::{
    outcomes::{CmdOutcome, ItemApplyBoxed, ItemApplyPartialBoxed},
    output::OutputWrite,
    params::ParamsKeys,
    Error, IndexMap, ItemSpecBoxed, ItemSpecRt, Storage,
};
use tokio::sync::{mpsc, mpsc::UnboundedSender};

use crate::BUFFERED_FUTURES_MAX;

cfg_if::cfg_if! {
    if #[cfg(feature = "output_progress")] {
        use peace_cfg::{
            progress::{
                ProgressComplete,
                ProgressMsgUpdate,
                ProgressSender,
                ProgressUpdate,
                ProgressUpdateAndId,
            },
        };
        use peace_rt_model::CmdProgressTracker;
        use tokio::sync::mpsc::Sender;
    }
}

#[derive(Debug)]
pub struct ApplyCmd<E, O, PKeys, StatesTsApply, StatesTsApplyDry>(
    PhantomData<(E, O, PKeys, StatesTsApply, StatesTsApplyDry)>,
);

impl<E, O, PKeys, StatesTsApply, StatesTsApplyDry>
    ApplyCmd<E, O, PKeys, StatesTsApply, StatesTsApplyDry>
where
    E: std::error::Error + From<Error> + Send + 'static,
    PKeys: ParamsKeys + 'static,
    O: OutputWrite<E>,
    StatesTsApply: Send + Sync + 'static,
    StatesTsApplyDry: Send + Sync + 'static,
    States<StatesTsApply>: From<StatesCurrent> + Send + Sync + 'static,
    States<StatesTsApplyDry>: From<StatesCurrent> + Send + Sync + 'static,
{
    /// Conditionally runs [`ApplyOpSpec`]`::`[`exec_dry`] for each
    /// [`ItemSpec`].
    ///
    /// In practice this runs [`ApplyOpSpec::check`], and only runs
    /// [`exec_dry`] if execution is required.
    ///
    /// # Note
    ///
    /// To only make changes when they are *all* likely to work, we execute the
    /// functions as homogeneous groups instead of interleaving the functions
    /// together per `ItemSpec`:
    ///
    /// 1. Run [`ApplyOpSpec::check`] for all `ItemSpec`s.
    /// 2. Run [`ApplyOpSpec::exec_dry`] for all `ItemSpec`s.
    /// 3. Fetch `StatesCurrent` again, and compare.
    ///
    /// State cannot be fetched interleaved with `exec_dry` as it may use
    /// different `Data`.
    ///
    /// [`exec_dry`]: peace_cfg::ApplyOpSpec::exec_dry
    /// [`ApplyOpSpec::check`]: peace_cfg::ApplyOpSpec::check
    /// [`ApplyOpSpec::exec_dry`]: peace_cfg::ApplyOpSpec::exec_dry
    /// [`ItemSpec`]: peace_cfg::ItemSpec
    /// [`ApplyOpSpec`]: peace_cfg::ItemSpec::ApplyOpSpec
    pub async fn exec_dry(
        cmd_ctx: &mut CmdCtx<SingleProfileSingleFlow<'_, E, O, PKeys, SetUp>>,
        states_saved: &StatesSaved,
        apply_for: ApplyFor,
    ) -> CmdOutcome<States<StatesTsApplyDry>, E> {
        let CmdOutcome {
            value: (states_applied, _states_desired),
            errors,
        } = Self::exec_internal(cmd_ctx, states_saved, apply_for, true).await;

        CmdOutcome {
            value: states_applied,
            errors,
        }
    }

    /// Conditionally runs [`ApplyOpSpec`]`::`[`exec`] for each
    /// [`ItemSpec`].
    ///
    /// In practice this runs [`ApplyOpSpec::check`], and only runs
    /// [`exec`] if execution is required.
    ///
    /// This function takes in a `StatesSaved`, but if you retrieve the state
    /// within the same execution, and have a `StatesCurrent`, you can turn this
    /// into `StatesSaved` by using `StatesSaved::from(states_current)` or
    /// calling the `.into()` method.
    ///
    /// # Note
    ///
    /// To only make changes when they are *all* likely to work, we execute the
    /// functions as homogeneous groups instead of interleaving the functions
    /// together per `ItemSpec`:
    ///
    /// 1. Run [`ApplyOpSpec::check`] for all `ItemSpec`s.
    /// 2. Run [`ApplyOpSpec::exec`] for all `ItemSpec`s.
    /// 3. Fetch `StatesCurrent` again, and compare.
    ///
    /// State cannot be fetched interleaved with `exec` as it may use
    /// different `Data`.
    ///
    /// [`exec`]: peace_cfg::ApplyOpSpec::exec
    /// [`ApplyOpSpec::check`]: peace_cfg::ApplyOpSpec::check
    /// [`ApplyOpSpec::exec`]: peace_cfg::ApplyOpSpec::exec
    /// [`ItemSpec`]: peace_cfg::ItemSpec
    /// [`ApplyOpSpec`]: peace_cfg::ItemSpec::ApplyOpSpec
    pub async fn exec(
        cmd_ctx: &mut CmdCtx<SingleProfileSingleFlow<'_, E, O, PKeys, SetUp>>,
        states_saved: &StatesSaved,
        apply_for: ApplyFor,
    ) -> Result<CmdOutcome<States<StatesTsApply>, E>, E> {
        let CmdOutcome {
            value: (states_applied, states_desired),
            errors,
        } = Self::exec_internal(cmd_ctx, states_saved, apply_for, false).await;
        Self::serialize_saved(cmd_ctx.resources(), &states_applied).await?;

        match apply_for {
            ApplyFor::Ensure => {
                Self::serialize_desired(cmd_ctx.resources(), &states_desired).await?;
            }
            ApplyFor::Clean => {}
        };

        let cmd_outcome = CmdOutcome {
            value: states_applied,
            errors,
        };
        Ok(cmd_outcome)
    }

    /// Conditionally runs [`ApplyOpSpec`]`::`[`exec`] for each [`ItemSpec`].
    ///
    /// Same as [`Self::exec`], but does not change the type state, and returns
    /// [`States<StatesTsApply>`].
    ///
    /// [`exec`]: peace_cfg::ApplyOpSpec::exec
    /// [`ItemSpec`]: peace_cfg::ItemSpec
    /// [`ApplyOpSpec`]: peace_cfg::ItemSpec::ApplyOpSpec
    async fn exec_internal<StatesTs>(
        cmd_ctx: &mut CmdCtx<SingleProfileSingleFlow<'_, E, O, PKeys, SetUp>>,
        states_saved: &StatesSaved,
        apply_for: ApplyFor,
        dry_run: bool,
    ) -> CmdOutcome<(States<StatesTs>, StatesDesired), E> {
        let SingleProfileSingleFlowView {
            #[cfg(feature = "output_progress")]
            output,
            #[cfg(feature = "output_progress")]
            cmd_progress_tracker,
            flow,
            resources,
            ..
        } = cmd_ctx.view();
        let item_spec_graph = flow.graph();

        cfg_if::cfg_if! {
            if #[cfg(feature = "output_progress")] {
                output.progress_begin(cmd_progress_tracker).await;

                let CmdProgressTracker {
                    multi_progress: _,
                    progress_trackers,
                } = cmd_progress_tracker;

                let (progress_tx, progress_rx) =
                    mpsc::channel::<ProgressUpdateAndId>(crate::PROGRESS_COUNT_MAX);
            }
        }

        // `StatesTsApply` represents the states of items *after* this cmd has run,
        // even if no change occurs. This means it should begin as `StatesSaved` or
        // `StatesCurrent`, and updated when a new state has been applied and
        // re-discovered.
        //
        // Notably, the initial `StatesSaved` / `StatesCurrent` may not contain a state
        // for item specs whose state cannot be discovered, e.g. a file on a remote
        // server, when the remote server doesn't exist.
        let mut states_applied_mut =
            StatesMut::<StatesTs>::from((*states_saved).clone().into_inner());
        let mut states_desired_mut = StatesMut::<Desired>::new();

        let (outcomes_tx, mut outcomes_rx) = mpsc::unbounded_channel::<ItemApplyOutcome<E>>();

        let resources_ref = &*resources;
        let execution_task = async move {
            #[cfg(feature = "output_progress")]
            let progress_tx = &progress_tx;
            let outcomes_tx = &outcomes_tx;

            match apply_for {
                ApplyFor::Ensure => {
                    let (Ok(()) | Err(())) = item_spec_graph
                        .try_for_each_concurrent(BUFFERED_FUTURES_MAX, |item_spec| {
                            Self::item_apply_exec(
                                resources_ref,
                                apply_for,
                                #[cfg(feature = "output_progress")]
                                progress_tx,
                                outcomes_tx,
                                item_spec,
                                dry_run,
                            )
                        })
                        .await
                        .map_err(|_vec_units: Vec<()>| ());
                }
                ApplyFor::Clean => {
                    let (Ok(()) | Err(())) = item_spec_graph
                        .try_for_each_concurrent_rev(BUFFERED_FUTURES_MAX, |item_spec| {
                            Self::item_apply_exec(
                                resources_ref,
                                apply_for,
                                #[cfg(feature = "output_progress")]
                                progress_tx,
                                outcomes_tx,
                                item_spec,
                                dry_run,
                            )
                        })
                        .await
                        .map_err(|_vec_units: Vec<()>| ());
                }
            }

            // `progress_tx` is dropped here, so `progress_rx` will safely end.
        };

        #[cfg(feature = "output_progress")]
        let progress_render_task =
            crate::progress::Progress::progress_render(output, progress_trackers, progress_rx);

        let mut errors = IndexMap::<ItemSpecId, E>::new();
        let outcomes_rx_task = async {
            while let Some(outcome) = outcomes_rx.recv().await {
                match outcome {
                    ItemApplyOutcome::PrepareFail {
                        item_spec_id,
                        item_apply_partial,
                        error,
                    } => {
                        errors.insert(item_spec_id.clone(), error);

                        // Save `state_target` (which is state_desired) if we are not cleaning up.
                        match apply_for {
                            ApplyFor::Ensure => {
                                if let Some(state_desired) = item_apply_partial.state_target() {
                                    states_desired_mut.insert_raw(item_spec_id, state_desired);
                                }
                            }
                            ApplyFor::Clean => {}
                        }
                    }
                    ItemApplyOutcome::Success {
                        item_spec_id,
                        item_apply,
                    } => {
                        if let Some(state_applied) = item_apply.state_applied() {
                            states_applied_mut.insert_raw(item_spec_id.clone(), state_applied);
                        } else {
                            // Item was already in the desired state.
                            // No change to saved state.
                        }

                        // Save `state_target` (which is state_desired) if we are not cleaning up.
                        match apply_for {
                            ApplyFor::Ensure => {
                                let state_desired = item_apply.state_target();
                                states_desired_mut.insert_raw(item_spec_id, state_desired);
                            }
                            ApplyFor::Clean => {}
                        }
                    }
                    ItemApplyOutcome::Fail {
                        item_spec_id,
                        item_apply,
                        error,
                    } => {
                        errors.insert(item_spec_id.clone(), error);
                        if let Some(state_applied) = item_apply.state_applied() {
                            states_applied_mut.insert_raw(item_spec_id.clone(), state_applied);
                        }

                        // Save `state_target` (which is state_desired) if we are not cleaning up.
                        match apply_for {
                            ApplyFor::Ensure => {
                                let state_desired = item_apply.state_target();
                                states_desired_mut.insert_raw(item_spec_id, state_desired);
                            }
                            ApplyFor::Clean => {}
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

        // TODO: Should we run `StatesCurrentFn` again?
        //
        // i.e. is it part of `ApplyOpSpec::exec`'s contract to return the state.
        //
        // * It may be duplication of code.
        // * `FileDownloadItemSpec` needs to know the ETag from the last request, which:
        //     - in `StatesCurrentFn` comes from `StatesSaved`
        //     - in `ApplyCmd` comes from `StatesTsApply`
        // * `ShCmdItemSpec` doesn't return the state in the apply script, so in the
        //   item spec we run the state current script after the apply exec script.
        let states_applied = states_applied_mut.into();
        let states_desired = states_desired_mut.into();

        CmdOutcome {
            value: (states_applied, states_desired),
            errors,
        }
    }

    ///
    /// # Implementation Note
    ///
    /// Tried passing through the function to execute instead of a `dry_run`
    /// parameter, but couldn't convince the compiler that the lifetimes match
    /// up:
    ///
    /// ```rust,ignore
    /// async fn item_apply_exec<F, Fut>(
    ///     resources: &Resources<SetUp>,
    ///     outcomes_tx: &UnboundedSender<ItemApplyOutcome<E>>,
    ///     item_spec: FnRef<'_, ItemSpecBoxed<E>>,
    ///     f: F,
    /// ) -> bool
    /// where
    ///     F: (Fn(&dyn ItemSpecRt<E>, op_ctx: OpCtx<'_>, &Resources<SetUp>, &mut ItemApplyBoxed) -> Fut) + Copy,
    ///     Fut: Future<Output = Result<(), E>>,
    /// ```
    async fn item_apply_exec(
        resources: &Resources<SetUp>,
        apply_for: ApplyFor,
        #[cfg(feature = "output_progress")] progress_tx: &Sender<ProgressUpdateAndId>,
        outcomes_tx: &UnboundedSender<ItemApplyOutcome<E>>,
        item_spec: &ItemSpecBoxed<E>,
        dry_run: bool,
    ) -> Result<(), ()> {
        let apply_fn = if dry_run {
            ItemSpecRt::apply_exec_dry
        } else {
            ItemSpecRt::apply_exec
        };

        let item_spec_id = item_spec.id();
        let op_ctx = OpCtx::new(
            item_spec_id,
            #[cfg(feature = "output_progress")]
            ProgressSender::new(item_spec_id, progress_tx),
        );
        let item_apply = match apply_for {
            ApplyFor::Ensure => ItemSpecRt::ensure_prepare(&**item_spec, op_ctx, resources).await,
            ApplyFor::Clean => ItemSpecRt::clean_prepare(&**item_spec, op_ctx, resources).await,
        };

        match item_apply {
            Ok(mut item_apply) => {
                match item_apply.op_check_status() {
                    #[cfg(not(feature = "output_progress"))]
                    OpCheckStatus::ExecRequired => {}
                    #[cfg(feature = "output_progress")]
                    OpCheckStatus::ExecRequired { progress_limit } => {
                        // Update `OutputWrite`s with progress limit.
                        let _progress_send_unused = progress_tx.try_send(ProgressUpdateAndId {
                            item_spec_id: item_spec_id.clone(),
                            progress_update: ProgressUpdate::Limit(progress_limit),
                            msg_update: ProgressMsgUpdate::Set(String::from("in progress")),
                        });
                    }
                    OpCheckStatus::ExecNotRequired => {
                        #[cfg(feature = "output_progress")]
                        let _progress_send_unused = progress_tx.try_send(ProgressUpdateAndId {
                            item_spec_id: item_spec_id.clone(),
                            progress_update: ProgressUpdate::Complete(ProgressComplete::Success),
                            msg_update: ProgressMsgUpdate::Set(String::from("nothing to do!")),
                        });

                        // TODO: write test for this case
                        // In case of an interrupt or power failure, we may not have written states
                        // to disk.
                        outcomes_tx
                            .send(ItemApplyOutcome::Success {
                                item_spec_id: item_spec.id().clone(),
                                item_apply,
                            })
                            .expect("unreachable: `outcomes_rx` is in a sibling task.");

                        // short-circuit
                        return Ok(());
                    }
                }
                match apply_fn(&**item_spec, op_ctx, resources, &mut item_apply).await {
                    Ok(()) => {
                        // apply succeeded

                        #[cfg(feature = "output_progress")]
                        let _progress_send_unused = progress_tx.try_send(ProgressUpdateAndId {
                            item_spec_id: item_spec_id.clone(),
                            progress_update: ProgressUpdate::Complete(ProgressComplete::Success),
                            msg_update: ProgressMsgUpdate::Set(String::from("done!")),
                        });

                        outcomes_tx
                            .send(ItemApplyOutcome::Success {
                                item_spec_id: item_spec.id().clone(),
                                item_apply,
                            })
                            .expect("unreachable: `outcomes_rx` is in a sibling task.");

                        Ok(())
                    }
                    Err(error) => {
                        // apply failed

                        #[cfg(feature = "output_progress")]
                        let _progress_send_unused = progress_tx.try_send(ProgressUpdateAndId {
                            item_spec_id: item_spec_id.clone(),
                            progress_update: ProgressUpdate::Complete(ProgressComplete::Fail),
                            msg_update: ProgressMsgUpdate::Set(
                                error
                                    .source()
                                    .map(|source| format!("{source}"))
                                    .unwrap_or_else(|| format!("{error}")),
                            ),
                        });

                        outcomes_tx
                            .send(ItemApplyOutcome::Fail {
                                item_spec_id: item_spec.id().clone(),
                                item_apply,
                                error,
                            })
                            .expect("unreachable: `outcomes_rx` is in a sibling task.");

                        // we should stop processing.
                        Err(())
                    }
                }
            }
            Err((error, item_apply_partial)) => {
                #[cfg(feature = "output_progress")]
                let _progress_send_unused = progress_tx.try_send(ProgressUpdateAndId {
                    item_spec_id: item_spec.id().clone(),
                    progress_update: ProgressUpdate::Complete(ProgressComplete::Fail),
                    msg_update: ProgressMsgUpdate::Set(
                        error
                            .source()
                            .map(|source| format!("{source}"))
                            .unwrap_or_else(|| format!("{error}")),
                    ),
                });

                outcomes_tx
                    .send(ItemApplyOutcome::PrepareFail {
                        item_spec_id: item_spec.id().clone(),
                        item_apply_partial,
                        error,
                    })
                    .expect("unreachable: `outcomes_rx` is in a sibling task.");

                Err(())
            }
        }
    }

    // TODO: This duplicates a bit of code with `StatesDiscoverCmd`,
    // `StatesCurrentDiscoverCmd`, `StatesDesiredDiscoverCmd`.
    async fn serialize_saved(
        resources: &Resources<SetUp>,
        states_applied: &States<StatesTsApply>,
    ) -> Result<(), E> {
        use peace_rt_model::StatesSerializer;

        let flow_dir = resources.borrow::<FlowDir>();
        let storage = resources.borrow::<Storage>();
        let states_saved_file = StatesSavedFile::from(&*flow_dir);

        StatesSerializer::serialize(&storage, states_applied, &states_saved_file).await?;

        drop(flow_dir);
        drop(storage);

        Ok(())
    }

    async fn serialize_desired(
        resources: &Resources<SetUp>,
        states_desired: &StatesDesired,
    ) -> Result<(), E> {
        use peace_rt_model::StatesSerializer;

        let flow_dir = resources.borrow::<FlowDir>();
        let storage = resources.borrow::<Storage>();
        let states_desired_file = StatesDesiredFile::from(&*flow_dir);

        StatesSerializer::serialize(&storage, states_desired, &states_desired_file).await?;

        drop(flow_dir);
        drop(storage);

        Ok(())
    }
}

impl<E, O, PKeys, StatesTsApply, StatesTsApplyDry> Default
    for ApplyCmd<E, O, PKeys, StatesTsApply, StatesTsApplyDry>
{
    fn default() -> Self {
        Self(PhantomData)
    }
}

#[allow(dead_code)]
#[derive(Debug)]
enum ItemApplyOutcome<E> {
    /// Error occurred when discovering current state, desired states, state
    /// diff, or `OpCheckStatus`.
    PrepareFail {
        item_spec_id: ItemSpecId,
        item_apply_partial: ItemApplyPartialBoxed,
        error: E,
    },
    /// Ensure execution succeeded.
    Success {
        item_spec_id: ItemSpecId,
        item_apply: ItemApplyBoxed,
    },
    /// Ensure execution failed.
    Fail {
        item_spec_id: ItemSpecId,
        item_apply: ItemApplyBoxed,
        error: E,
    },
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ApplyFor {
    Ensure,
    Clean,
}
