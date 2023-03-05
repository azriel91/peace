use std::{fmt::Debug, marker::PhantomData};

use futures::{
    stream::{StreamExt, TryStreamExt},
    TryStream,
};
use peace_cfg::OpCheckStatus;
use peace_cmd::{
    ctx::CmdCtx,
    scopes::{SingleProfileSingleFlow, SingleProfileSingleFlowView},
};
use peace_resources::{
    internal::OpCheckStatuses,
    resources::ts::SetUp,
    states::{StatesCleaned, StatesCleanedDry, StatesCurrent},
    Resources,
};
use peace_rt_model::{
    output::OutputWrite, params::ParamsKeys, Error, FnRef, ItemSpecBoxed, ItemSpecGraph,
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
    pub async fn exec_dry(
        cmd_ctx: &mut CmdCtx<SingleProfileSingleFlow<'_, E, O, PKeys, SetUp>>,
    ) -> Result<StatesCleanedDry, E> {
        let states_current = StatesCurrentDiscoverCmd::<E, O, PKeys>::exec(cmd_ctx).await?;

        let SingleProfileSingleFlowView {
            flow, resources, ..
        } = cmd_ctx.scope_mut().view();
        let item_spec_graph = flow.graph();

        let op_check_statuses =
            Self::clean_op_spec_check(item_spec_graph, resources, &states_current).await?;
        Self::clean_op_spec_exec_dry(
            item_spec_graph,
            resources,
            &states_current,
            &op_check_statuses,
        )
        .await?;

        // TODO: This fetches the real state, whereas for a dry run, it would be useful
        // to show the imagined altered state.
        let states_cleaned_dry =
            StatesCleanedDry::from(StatesCurrentDiscoverCmd::<E, O, PKeys>::exec(cmd_ctx).await?);

        Ok(states_cleaned_dry)
    }

    async fn clean_op_spec_exec_dry(
        item_spec_graph: &ItemSpecGraph<E>,
        resources: &Resources<SetUp>,
        states_current: &StatesCurrent,
        op_check_statuses: &OpCheckStatuses,
    ) -> Result<(), E> {
        Self::clean_op_spec_stream(item_spec_graph, op_check_statuses)
            .try_for_each(|item_spec| async move {
                item_spec.clean_op_exec_dry(resources, states_current).await
            })
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
    pub async fn exec(
        cmd_ctx: &mut CmdCtx<SingleProfileSingleFlow<'_, E, O, PKeys, SetUp>>,
    ) -> Result<StatesCleaned, E> {
        // TODO: pass in and confirm that `StatesSaved` matches `StatesCurrent`.
        let states_current = StatesCurrentDiscoverCmd::<E, O, PKeys>::exec(cmd_ctx).await?;

        let SingleProfileSingleFlowView {
            flow, resources, ..
        } = cmd_ctx.scope_mut().view();
        let item_spec_graph = flow.graph();

        let op_check_statuses =
            Self::clean_op_spec_check(item_spec_graph, resources, &states_current).await?;
        Self::clean_op_spec_exec(
            item_spec_graph,
            resources,
            &states_current,
            &op_check_statuses,
        )
        .await?;

        let states_cleaned =
            StatesCleaned::from(StatesCurrentDiscoverCmd::<E, O, PKeys>::exec(cmd_ctx).await?);

        Ok(states_cleaned)
    }

    async fn clean_op_spec_check(
        item_spec_graph: &ItemSpecGraph<E>,
        resources: &Resources<SetUp>,
        states_current: &StatesCurrent,
    ) -> Result<OpCheckStatuses, E> {
        let op_check_statuses = item_spec_graph
            .stream()
            .map(Result::<_, E>::Ok)
            .and_then(|item_spec| async move {
                let op_check_status = item_spec.clean_op_check(resources, states_current).await?;
                Ok((item_spec.id().clone(), op_check_status))
            })
            .try_collect::<OpCheckStatuses>()
            .await?;

        Ok(op_check_statuses)
    }

    async fn clean_op_spec_exec(
        item_spec_graph: &ItemSpecGraph<E>,
        resources: &Resources<SetUp>,
        states_current: &StatesCurrent,
        op_check_statuses: &OpCheckStatuses,
    ) -> Result<(), E> {
        Self::clean_op_spec_stream(item_spec_graph, op_check_statuses)
            .try_for_each(|item_spec| async move {
                item_spec.clean_op_exec(resources, states_current).await
            })
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
