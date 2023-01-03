use std::marker::PhantomData;

use peace_cfg::{ItemSpecId, OpCtx};
use peace_resources::{
    resources::ts::{Ensured, EnsuredDry, SetUp},
    states::{self, States, StatesCurrent, StatesEnsured, StatesEnsuredDry},
    Resources,
};
use peace_rt_model::{
    outcomes::{ItemEnsureBoxed, ItemEnsurePartialBoxed},
    CmdContext, Error, ItemSpecBoxed, ItemSpecGraph, ItemSpecRt, OutputWrite,
};
use tokio::sync::{mpsc, mpsc::UnboundedSender};

use crate::{cmds::sub::StatesCurrentDiscoverCmd, BUFFERED_FUTURES_MAX};

#[derive(Debug)]
pub struct EnsureCmd<E, O>(PhantomData<(E, O)>);

impl<E, O> EnsureCmd<E, O>
where
    E: std::error::Error + From<Error> + Send,
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
    /// [`exec_dry`]: peace_cfg::EnsureOpSpec::exec
    /// [`EnsureOpSpec::check`]: peace_cfg::EnsureOpSpec::check
    /// [`EnsureOpSpec::exec_dry`]: peace_cfg::EnsureOpSpec::exec_dry
    /// [`ItemSpec`]: peace_cfg::ItemSpec
    /// [`EnsureOpSpec`]: peace_cfg::ItemSpec::EnsureOpSpec
    pub async fn exec_dry(
        cmd_context: CmdContext<'_, E, O, SetUp>,
    ) -> Result<CmdContext<'_, E, O, EnsuredDry>, E> {
        let (workspace, item_spec_graph, output, resources, states_type_regs) =
            cmd_context.into_inner();
        let resources_result = Self::exec_internal::<EnsuredDry, states::ts::EnsuredDry>(
            item_spec_graph,
            output,
            resources,
            true,
        )
        .await;

        match resources_result {
            Ok(resources) => {
                {
                    let states_ensured_dry = resources.borrow::<StatesEnsuredDry>();
                    output.write_states_ensured_dry(&states_ensured_dry).await?;
                }
                let cmd_context = CmdContext::from((
                    workspace,
                    item_spec_graph,
                    output,
                    resources,
                    states_type_regs,
                ));
                Ok(cmd_context)
            }
            Err(e) => {
                output.write_err(&e).await?;
                Err(e)
            }
        }
    }

    /// Conditionally runs [`EnsureOpSpec`]`::`[`exec`] for each [`ItemSpec`].
    ///
    /// At the end of this function, [`Resources`] will be populated with
    /// [`StatesEnsured`].
    ///
    /// In practice this runs [`EnsureOpSpec::check`], and only runs [`exec`] if
    /// execution is required.
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
    /// State cannot be fetched interleaved with `exec` as it may use different
    /// `Data`.
    ///
    /// [`exec`]: peace_cfg::EnsureOpSpec::exec
    /// [`EnsureOpSpec::check`]: peace_cfg::EnsureOpSpec::check
    /// [`EnsureOpSpec::exec`]: peace_cfg::EnsureOpSpec::exec
    /// [`ItemSpec`]: peace_cfg::ItemSpec
    /// [`EnsureOpSpec`]: peace_cfg::ItemSpec::EnsureOpSpec
    pub async fn exec(
        cmd_context: CmdContext<'_, E, O, SetUp>,
    ) -> Result<CmdContext<'_, E, O, Ensured>, E> {
        let (workspace, item_spec_graph, output, resources, states_type_regs) =
            cmd_context.into_inner();
        // https://github.com/rust-lang/rust-clippy/issues/9111
        #[allow(clippy::needless_borrow)]
        let resources_result = Self::exec_internal::<Ensured, states::ts::Ensured>(
            item_spec_graph,
            output,
            resources,
            false,
        )
        .await;

        match resources_result {
            Ok(resources) => {
                {
                    let states_ensured = resources.borrow::<StatesEnsured>();
                    output.write_states_ensured(&states_ensured).await?;
                }
                let cmd_context = CmdContext::from((
                    workspace,
                    item_spec_graph,
                    output,
                    resources,
                    states_type_regs,
                ));
                Ok(cmd_context)
            }
            Err(e) => {
                output.write_err(&e).await?;
                Err(e)
            }
        }
    }

    /// Conditionally runs [`EnsureOpSpec`]`::`[`exec`] for each [`ItemSpec`].
    ///
    /// Same as [`Self::exec`], but does not change the type state, and returns
    /// [`StatesEnsured`].
    ///
    /// [`exec`]: peace_cfg::EnsureOpSpec::exec
    /// [`ItemSpec`]: peace_cfg::ItemSpec
    /// [`EnsureOpSpec`]: peace_cfg::ItemSpec::EnsureOpSpec
    async fn exec_internal<ResourcesTs, StatesTs>(
        item_spec_graph: &ItemSpecGraph<E>,
        #[cfg(not(feature = "output_progress"))] _output: &mut O,
        #[cfg(feature = "output_progress")] output: &mut O,
        mut resources: Resources<SetUp>,
        dry_run: bool,
    ) -> Result<Resources<ResourcesTs>, E>
    where
        for<'resources> States<StatesTs>: From<(StatesCurrent, &'resources Resources<SetUp>)>,
        Resources<ResourcesTs>: From<(Resources<SetUp>, States<StatesTs>)>,
    {
        #[cfg(feature = "output_progress")]
        let (progress_tx, mut progress_rx) = mpsc::channel(Self::PROGRESS_COUNT_MAX);
        let (outcomes_tx, mut outcomes_rx) = mpsc::unbounded_channel::<ItemEnsureOutcome<E>>();

        let resources_ref = &resources;
        let execution_task = async move {
            #[cfg(feature = "output_progress")]
            let progress_tx = &progress_tx;
            let outcomes_tx = &outcomes_tx;
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
        };

        #[cfg(feature = "output_progress")]
        let progress_render_task = async move {
            while let Some(progress_update) = progress_rx.recv().await {
                output.render(progress_update).await
            }
        };

        let _outcomes_rx_task = async move {
            while let Some(outcome) = outcomes_rx.recv().await {
                match outcome {
                    ItemEnsureOutcome::PrepareFail {
                        item_spec_id: _,
                        item_ensure_partial: _,
                        error: _,
                    } => todo!(),
                    ItemEnsureOutcome::Success {
                        item_spec_id: _,
                        item_ensure: _,
                    } => todo!(),
                    ItemEnsureOutcome::Fail {
                        item_spec_id: _,
                        item_ensure: _,
                        error: _,
                    } => todo!(),
                }
            }
        };

        cfg_if::cfg_if! {
            if #[cfg(feature = "output_progress")] {
                futures::join!(execution_task, progress_render_task);
            } else {
                futures::join!(execution_task);
            }
        }

        let states_current =
            StatesCurrentDiscoverCmd::<E, O>::exec_internal(item_spec_graph, &mut resources)
                .await?;

        let states_ensured = States::<StatesTs>::from((states_current, &resources));
        let resources = Resources::<ResourcesTs>::from((resources, states_ensured));

        Ok(resources)
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
        #[cfg(feature = "output_progress")] progress_tx: &peace_cfg::Sender<
            peace_cfg::ProgressUpdate,
        >,
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
                let op_ctx = OpCtx::new(
                    #[cfg(feature = "output_progress")]
                    progress_tx,
                );
                match f(&**item_spec, op_ctx, resources, &mut item_ensure).await {
                    Ok(()) => {
                        // ensure succeeded
                        outcomes_tx
                            .send(ItemEnsureOutcome::Success {
                                item_spec_id: item_spec.id(),
                                item_ensure,
                            })
                            .expect("unreachable: `outcomes_rx` is in a sibling task.");

                        Ok(())
                    }
                    Err(error) => {
                        // ensure failed
                        outcomes_tx
                            .send(ItemEnsureOutcome::Fail {
                                item_spec_id: item_spec.id(),
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
                outcomes_tx
                    .send(ItemEnsureOutcome::PrepareFail {
                        item_spec_id: item_spec.id(),
                        item_ensure_partial,
                        error,
                    })
                    .expect("unreachable: `outcomes_rx` is in a sibling task.");

                Err(())
            }
        }
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
