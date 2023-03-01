use std::{fmt::Debug, marker::PhantomData};

use futures::{
    stream::{StreamExt, TryStreamExt},
    TryStream,
};
use peace_cfg::OpCheckStatus;
use peace_cmd::{
    ctx::{CmdCtx, CmdCtxView},
    scopes::{SingleProfileSingleFlow, SingleProfileSingleFlowView},
};
use peace_resources::{
    internal::OpCheckStatuses,
    resources::ts::{Cleaned, CleanedDry, SetUp, WithStatesCurrent},
    states::{StatesCleaned, StatesCleanedDry},
    Resources,
};
use peace_rt_model::{
    cmd::CmdContext, cmd_context_params::ParamsKeys, output::OutputWrite, Error, FnRef,
    ItemSpecBoxed, ItemSpecGraph,
};

use crate::cmds::sub::StatesCurrentDiscoverCmd;

#[derive(Debug)]
pub struct CleanCmd<E, O, PKeys>(PhantomData<(E, O, PKeys)>);

impl<E, O, PKeys> CleanCmd<E, O, PKeys>
where
    E: std::error::Error + From<Error> + Send + 'static,
    PKeys: ParamsKeys + 'static,
    O: OutputWrite<E>,
{
    /// Conditionally runs [`CleanOpSpec`]`::`[`exec_dry`] for each
    /// [`ItemSpec`].
    ///
    /// In practice this runs [`CleanOpSpec::check`], and only runs
    /// [`exec_dry`] if execution is required.
    ///
    /// # Note
    ///
    /// To only make changes when they are *all* likely to work, we execute the
    /// functions as homogeneous groups instead of interleaving the functions
    /// together per `ItemSpec`:
    ///
    /// 1. Run [`CleanOpSpec::check`] for all `ItemSpec`s.
    /// 2. Run [`CleanOpSpec::exec_dry`] for all `ItemSpec`s.
    /// 3. Fetch `StatesCurrent` again, and compare.
    ///
    /// State cannot be fetched interleaved with `exec_dry` as it may use
    /// different `Data`.
    ///
    /// [`exec_dry`]: peace_cfg::CleanOpSpec::exec
    /// [`CleanOpSpec::check`]: peace_cfg::CleanOpSpec::check
    /// [`CleanOpSpec::exec_dry`]: peace_cfg::CleanOpSpec::exec_dry
    /// [`ItemSpec`]: peace_cfg::ItemSpec
    /// [`CleanOpSpec`]: peace_cfg::ItemSpec::CleanOpSpec
    pub async fn exec_dry_v2(
        cmd_ctx: CmdCtx<'_, O, SingleProfileSingleFlow<E, PKeys, SetUp>, PKeys>,
    ) -> Result<CmdCtx<'_, O, SingleProfileSingleFlow<E, PKeys, CleanedDry>, PKeys>, E> {
        let cmd_ctx_result = Self::exec_dry_internal_v2(cmd_ctx).await;
        match cmd_ctx_result {
            Ok(mut cmd_ctx) => {
                {
                    let CmdCtxView { output, scope, .. } = cmd_ctx.view();
                    let resources = scope.resources();
                    let states_cleaned_dry = resources.borrow::<StatesCleanedDry>();
                    output.present(&*states_cleaned_dry).await?;
                }

                Ok(cmd_ctx)
            }
            Err(e) => Err(e),
        }
    }

    /// Conditionally runs [`CleanOpSpec`]`::`[`exec_dry`] for each
    /// [`ItemSpec`].
    ///
    /// In practice this runs [`CleanOpSpec::check`], and only runs
    /// [`exec_dry`] if execution is required.
    ///
    /// # Note
    ///
    /// To only make changes when they are *all* likely to work, we execute the
    /// functions as homogeneous groups instead of interleaving the functions
    /// together per `ItemSpec`:
    ///
    /// 1. Run [`CleanOpSpec::check`] for all `ItemSpec`s.
    /// 2. Run [`CleanOpSpec::exec_dry`] for all `ItemSpec`s.
    /// 3. Fetch `StatesCurrent` again, and compare.
    ///
    /// State cannot be fetched interleaved with `exec_dry` as it may use
    /// different `Data`.
    ///
    /// [`exec_dry`]: peace_cfg::CleanOpSpec::exec
    /// [`CleanOpSpec::check`]: peace_cfg::CleanOpSpec::check
    /// [`CleanOpSpec::exec_dry`]: peace_cfg::CleanOpSpec::exec_dry
    /// [`ItemSpec`]: peace_cfg::ItemSpec
    /// [`CleanOpSpec`]: peace_cfg::ItemSpec::CleanOpSpec
    pub async fn exec_dry(
        cmd_context: CmdContext<'_, E, O, SetUp, PKeys>,
    ) -> Result<CmdContext<'_, E, O, CleanedDry, PKeys>, E> {
        let CmdContext {
            workspace,
            item_spec_graph,
            output,
            resources,
            params_type_regs,
            states_type_regs,
            #[cfg(feature = "output_progress")]
            cmd_progress_tracker,
            ..
        } = cmd_context;
        let resources_result = Self::exec_dry_internal(item_spec_graph, resources).await;

        match resources_result {
            Ok(resources) => {
                {
                    let states_cleaned_dry = resources.borrow::<StatesCleanedDry>();
                    output.present(&*states_cleaned_dry).await?;
                }
                let cmd_context = CmdContext::from((
                    workspace,
                    item_spec_graph,
                    output,
                    resources,
                    params_type_regs,
                    states_type_regs,
                    #[cfg(feature = "output_progress")]
                    cmd_progress_tracker,
                ));
                Ok(cmd_context)
            }
            Err(e) => {
                output.write_err(&e).await?;
                Err(e)
            }
        }
    }

    /// Conditionally runs [`CleanOpSpec`]`::`[`exec_dry`] for each
    /// [`ItemSpec`].
    ///
    /// Same as [`Self::exec_dry`], but does not change the type state, and
    /// returns [`StatesCleaned`].
    ///
    /// [`exec_dry`]: peace_cfg::CleanOpSpec::exec_dry
    /// [`ItemSpec`]: peace_cfg::ItemSpec
    /// [`CleanOpSpec`]: peace_cfg::ItemSpec::CleanOpSpec
    pub(crate) async fn exec_dry_internal_v2(
        mut cmd_ctx: CmdCtx<'_, O, SingleProfileSingleFlow<E, PKeys, SetUp>, PKeys>,
    ) -> Result<CmdCtx<'_, O, SingleProfileSingleFlow<E, PKeys, CleanedDry>, PKeys>, E> {
        let SingleProfileSingleFlowView {
            flow, resources, ..
        } = cmd_ctx.scope_mut().view();
        let item_spec_graph = flow.graph();

        // https://github.com/rust-lang/rust-clippy/issues/9111
        #[allow(clippy::needless_borrow)]
        let states_current =
            StatesCurrentDiscoverCmd::<E, O, PKeys>::exec_internal(item_spec_graph, resources)
                .await?;
        let mut cmd_ctx = cmd_ctx.resources_update(|resources| {
            Resources::<WithStatesCurrent>::from((resources, states_current))
        });

        let SingleProfileSingleFlowView {
            flow, resources, ..
        } = cmd_ctx.scope_mut().view();
        let item_spec_graph = flow.graph();

        let op_check_statuses = Self::clean_op_spec_check(item_spec_graph, resources).await?;
        Self::clean_op_spec_exec_dry(item_spec_graph, resources, &op_check_statuses).await?;

        // TODO: This fetches the real state, whereas for a dry run, it would be useful
        // to show the imagined altered state.
        let states_current = StatesCurrentDiscoverCmd::<E, O, PKeys>::exec_internal_for_clean_dry(
            item_spec_graph,
            resources,
        )
        .await?;

        let states_cleaned_dry = StatesCleanedDry::from((states_current, &*resources));
        let cmd_ctx = cmd_ctx.resources_update(|resources| {
            Resources::<CleanedDry>::from((resources, states_cleaned_dry))
        });

        Ok(cmd_ctx)
    }

    /// Conditionally runs [`CleanOpSpec`]`::`[`exec_dry`] for each
    /// [`ItemSpec`].
    ///
    /// Same as [`Self::exec_dry`], but does not change the type state, and
    /// returns [`StatesCleaned`].
    ///
    /// [`exec_dry`]: peace_cfg::CleanOpSpec::exec_dry
    /// [`ItemSpec`]: peace_cfg::ItemSpec
    /// [`CleanOpSpec`]: peace_cfg::ItemSpec::CleanOpSpec
    pub(crate) async fn exec_dry_internal(
        item_spec_graph: &ItemSpecGraph<E>,
        mut resources: Resources<SetUp>,
    ) -> Result<Resources<CleanedDry>, E> {
        // https://github.com/rust-lang/rust-clippy/issues/9111
        #[allow(clippy::needless_borrow)]
        let states_current =
            StatesCurrentDiscoverCmd::<E, O, PKeys>::exec_internal(item_spec_graph, &mut resources)
                .await?;
        let resources = Resources::<WithStatesCurrent>::from((resources, states_current));
        let op_check_statuses = Self::clean_op_spec_check(item_spec_graph, &resources).await?;
        Self::clean_op_spec_exec_dry(item_spec_graph, &resources, &op_check_statuses).await?;

        // TODO: This fetches the real state, whereas for a dry run, it would be useful
        // to show the imagined altered state.
        let states_current = StatesCurrentDiscoverCmd::<E, O, PKeys>::exec_internal_for_clean_dry(
            item_spec_graph,
            &resources,
        )
        .await?;

        let states_cleaned_dry = StatesCleanedDry::from((states_current, &resources));
        let resources = Resources::<CleanedDry>::from((resources, states_cleaned_dry));

        Ok(resources)
    }

    async fn clean_op_spec_exec_dry(
        item_spec_graph: &ItemSpecGraph<E>,
        resources: &Resources<WithStatesCurrent>,
        op_check_statuses: &OpCheckStatuses,
    ) -> Result<(), E> {
        Self::clean_op_spec_stream(item_spec_graph, op_check_statuses)
            .try_for_each(|item_spec| async move { item_spec.clean_op_exec_dry(resources).await })
            .await?;
        Ok(())
    }

    /// Conditionally runs [`CleanOpSpec`]`::`[`exec`] for each [`ItemSpec`].
    ///
    /// At the end of this function, [`Resources`] will be populated with
    /// [`StatesCleaned`].
    ///
    /// In practice this runs [`CleanOpSpec::check`], and only runs [`exec`] if
    /// execution is required.
    ///
    /// # Note
    ///
    /// To only make changes when they are *all* likely to work, we execute the
    /// functions as homogeneous groups instead of interleaving the functions
    /// together per `ItemSpec`:
    ///
    /// 1. Run [`CleanOpSpec::check`] for all `ItemSpec`s.
    /// 2. Run [`CleanOpSpec::exec`] for all `ItemSpec`s.
    /// 3. Fetch `StatesCurrent` again, and compare.
    ///
    /// State cannot be fetched interleaved with `exec` as it may use different
    /// `Data`.
    ///
    /// [`exec`]: peace_cfg::CleanOpSpec::exec
    /// [`CleanOpSpec::check`]: peace_cfg::CleanOpSpec::check
    /// [`CleanOpSpec::exec`]: peace_cfg::CleanOpSpec::exec
    /// [`ItemSpec`]: peace_cfg::ItemSpec
    /// [`CleanOpSpec`]: peace_cfg::ItemSpec::CleanOpSpec
    pub async fn exec_v2(
        cmd_ctx: CmdCtx<'_, O, SingleProfileSingleFlow<E, PKeys, SetUp>, PKeys>,
    ) -> Result<CmdCtx<'_, O, SingleProfileSingleFlow<E, PKeys, Cleaned>, PKeys>, E> {
        let cmd_ctx_result = Self::exec_internal_v2(cmd_ctx).await;
        match cmd_ctx_result {
            Ok(mut cmd_ctx) => {
                {
                    let CmdCtxView { output, scope, .. } = cmd_ctx.view();
                    let resources = scope.resources();
                    let states_cleaned = resources.borrow::<StatesCleaned>();
                    output.present(&*states_cleaned).await?;
                }

                Ok(cmd_ctx)
            }
            Err(e) => Err(e),
        }
    }

    /// Conditionally runs [`CleanOpSpec`]`::`[`exec`] for each [`ItemSpec`].
    ///
    /// At the end of this function, [`Resources`] will be populated with
    /// [`StatesCleaned`].
    ///
    /// In practice this runs [`CleanOpSpec::check`], and only runs [`exec`] if
    /// execution is required.
    ///
    /// # Note
    ///
    /// To only make changes when they are *all* likely to work, we execute the
    /// functions as homogeneous groups instead of interleaving the functions
    /// together per `ItemSpec`:
    ///
    /// 1. Run [`CleanOpSpec::check`] for all `ItemSpec`s.
    /// 2. Run [`CleanOpSpec::exec`] for all `ItemSpec`s.
    /// 3. Fetch `StatesCurrent` again, and compare.
    ///
    /// State cannot be fetched interleaved with `exec` as it may use different
    /// `Data`.
    ///
    /// [`exec`]: peace_cfg::CleanOpSpec::exec
    /// [`CleanOpSpec::check`]: peace_cfg::CleanOpSpec::check
    /// [`CleanOpSpec::exec`]: peace_cfg::CleanOpSpec::exec
    /// [`ItemSpec`]: peace_cfg::ItemSpec
    /// [`CleanOpSpec`]: peace_cfg::ItemSpec::CleanOpSpec
    pub async fn exec(
        cmd_context: CmdContext<'_, E, O, SetUp, PKeys>,
    ) -> Result<CmdContext<'_, E, O, Cleaned, PKeys>, E> {
        let CmdContext {
            workspace,
            item_spec_graph,
            output,
            resources,
            params_type_regs,
            states_type_regs,
            #[cfg(feature = "output_progress")]
            cmd_progress_tracker,
            ..
        } = cmd_context;
        // https://github.com/rust-lang/rust-clippy/issues/9111
        #[allow(clippy::needless_borrow)]
        let resources_result = Self::exec_internal(item_spec_graph, resources).await;

        match resources_result {
            Ok(resources) => {
                {
                    let states_cleaned = resources.borrow::<StatesCleaned>();
                    output.present(&*states_cleaned).await?;
                }
                let cmd_context = CmdContext::from((
                    workspace,
                    item_spec_graph,
                    output,
                    resources,
                    params_type_regs,
                    states_type_regs,
                    #[cfg(feature = "output_progress")]
                    cmd_progress_tracker,
                ));
                Ok(cmd_context)
            }
            Err(e) => {
                output.write_err(&e).await?;
                Err(e)
            }
        }
    }

    /// Conditionally runs [`CleanOpSpec`]`::`[`exec`] for each [`ItemSpec`].
    ///
    /// Same as [`Self::exec`], but does not change the type state, and returns
    /// [`StatesCleaned`].
    ///
    /// [`exec`]: peace_cfg::CleanOpSpec::exec
    /// [`ItemSpec`]: peace_cfg::ItemSpec
    /// [`CleanOpSpec`]: peace_cfg::ItemSpec::CleanOpSpec
    pub(crate) async fn exec_internal_v2(
        mut cmd_ctx: CmdCtx<'_, O, SingleProfileSingleFlow<E, PKeys, SetUp>, PKeys>,
    ) -> Result<CmdCtx<'_, O, SingleProfileSingleFlow<E, PKeys, Cleaned>, PKeys>, E> {
        let SingleProfileSingleFlowView {
            flow, resources, ..
        } = cmd_ctx.scope_mut().view();
        let item_spec_graph = flow.graph();

        // https://github.com/rust-lang/rust-clippy/issues/9111
        #[allow(clippy::needless_borrow)]
        let states_current =
            StatesCurrentDiscoverCmd::<E, O, PKeys>::exec_internal(item_spec_graph, resources)
                .await?;
        let mut cmd_ctx = cmd_ctx.resources_update(|resources| {
            Resources::<WithStatesCurrent>::from((resources, states_current))
        });

        let SingleProfileSingleFlowView {
            flow, resources, ..
        } = cmd_ctx.scope_mut().view();
        let item_spec_graph = flow.graph();

        let op_check_statuses = Self::clean_op_spec_check(item_spec_graph, resources).await?;
        Self::clean_op_spec_exec(item_spec_graph, resources, &op_check_statuses).await?;

        let states_current = StatesCurrentDiscoverCmd::<E, O, PKeys>::exec_internal_for_clean(
            item_spec_graph,
            resources,
        )
        .await?;

        let states_cleaned = StatesCleaned::from((states_current, &*resources));
        let cmd_ctx = cmd_ctx
            .resources_update(|resources| Resources::<Cleaned>::from((resources, states_cleaned)));

        Ok(cmd_ctx)
    }

    /// Conditionally runs [`CleanOpSpec`]`::`[`exec`] for each [`ItemSpec`].
    ///
    /// Same as [`Self::exec`], but does not change the type state, and returns
    /// [`StatesCleaned`].
    ///
    /// [`exec`]: peace_cfg::CleanOpSpec::exec
    /// [`ItemSpec`]: peace_cfg::ItemSpec
    /// [`CleanOpSpec`]: peace_cfg::ItemSpec::CleanOpSpec
    pub(crate) async fn exec_internal(
        item_spec_graph: &ItemSpecGraph<E>,
        mut resources: Resources<SetUp>,
    ) -> Result<Resources<Cleaned>, E> {
        // https://github.com/rust-lang/rust-clippy/issues/9111
        #[allow(clippy::needless_borrow)]
        let states =
            StatesCurrentDiscoverCmd::<E, O, PKeys>::exec_internal(item_spec_graph, &mut resources)
                .await?;
        let mut resources = Resources::<WithStatesCurrent>::from((resources, states));
        let op_check_statuses = Self::clean_op_spec_check(item_spec_graph, &resources).await?;
        Self::clean_op_spec_exec(item_spec_graph, &resources, &op_check_statuses).await?;

        let states_current = StatesCurrentDiscoverCmd::<E, O, PKeys>::exec_internal_for_clean(
            item_spec_graph,
            &mut resources,
        )
        .await?;

        let states_cleaned = StatesCleaned::from((states_current, &resources));
        let resources = Resources::<Cleaned>::from((resources, states_cleaned));

        Ok(resources)
    }

    async fn clean_op_spec_check(
        item_spec_graph: &ItemSpecGraph<E>,
        resources: &Resources<WithStatesCurrent>,
    ) -> Result<OpCheckStatuses, E> {
        let op_check_statuses = item_spec_graph
            .stream()
            .map(Result::<_, E>::Ok)
            .and_then(|item_spec| async move {
                let op_check_status = item_spec.clean_op_check(resources).await?;
                Ok((item_spec.id().clone(), op_check_status))
            })
            .try_collect::<OpCheckStatuses>()
            .await?;

        Ok(op_check_statuses)
    }

    async fn clean_op_spec_exec(
        item_spec_graph: &ItemSpecGraph<E>,
        resources: &Resources<WithStatesCurrent>,
        op_check_statuses: &OpCheckStatuses,
    ) -> Result<(), E> {
        Self::clean_op_spec_stream(item_spec_graph, op_check_statuses)
            .try_for_each(|item_spec| async move { item_spec.clean_op_exec(resources).await })
            .await?;
        Ok(())
    }

    fn clean_op_spec_stream<'f>(
        item_spec_graph: &'f ItemSpecGraph<E>,
        op_check_statuses: &'f OpCheckStatuses,
    ) -> impl TryStream<Ok = FnRef<'f, ItemSpecBoxed<E>>, Error = E> {
        item_spec_graph
            .stream()
            .filter(|item_spec| {
                let exec_required = op_check_statuses
                    .get(item_spec.id())
                    .map(|op_check_status| {
                        matches!(op_check_status, OpCheckStatus::ExecRequired { .. })
                    })
                    .unwrap_or(true); // Should be unreachable, but we just execute if we get to this state.

                async move { exec_required }
            })
            .map(Result::Ok)
    }
}

impl<E, O, PKeys> Default for CleanCmd<E, O, PKeys> {
    fn default() -> Self {
        Self(PhantomData)
    }
}
