use std::marker::PhantomData;

use futures::{
    stream::{StreamExt, TryStreamExt},
    TryStream,
};
use peace_cfg::OpCheckStatus;
use peace_resources::{
    internal::OpCheckStatuses,
    resources_type_state::{Ensured, EnsuredDry, SetUp, WithStateDiffs},
    Resources, StatesEnsured, StatesEnsuredDry,
};
use peace_rt_model::{CmdContext, FnRef, ItemSpecBoxed, ItemSpecGraph};

use crate::{DiffCmd, StateCurrentCmd};

#[derive(Debug)]
pub struct EnsureCmd<E>(PhantomData<E>);

impl<E> EnsureCmd<E>
where
    E: std::error::Error,
{
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
    /// 3. Fetch `States` again, and compare.
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
        cmd_context: CmdContext<'_, SetUp, E>,
    ) -> Result<CmdContext<EnsuredDry, E>, E> {
        let cmd_context = DiffCmd::exec(cmd_context).await?;
        let (workspace, item_spec_graph, resources) = cmd_context.into_inner();
        let states_ensured_dry = Self::exec_dry_internal(&item_spec_graph, &resources).await?;

        let resources = Resources::<EnsuredDry>::from((resources, states_ensured_dry));
        let cmd_context = CmdContext::from((workspace, item_spec_graph, resources));
        Ok(cmd_context)
    }

    /// Conditionally runs [`EnsureOpSpec`]`::`[`exec_dry`] for each
    /// [`ItemSpec`].
    ///
    /// Same as [`Self::exec_dry`], but does not change the type state, and
    /// returns [`StatesEnsured`].
    ///
    /// [`exec_dry`]: peace_cfg::EnsureOpSpec::exec_dry
    /// [`ItemSpec`]: peace_cfg::ItemSpec
    /// [`EnsureOpSpec`]: peace_cfg::ItemSpec::EnsureOpSpec
    pub(crate) async fn exec_dry_internal(
        item_spec_graph: &ItemSpecGraph<E>,
        resources: &Resources<WithStateDiffs>,
    ) -> Result<StatesEnsuredDry, E> {
        let op_check_statuses = Self::ensure_op_spec_check(item_spec_graph, resources).await?;
        Self::ensure_op_spec_exec_dry(item_spec_graph, resources, &op_check_statuses).await?;

        // TODO: This fetches the real state, whereas for a dry run, it would be useful
        // to show the imagined altered state.
        let states = StateCurrentCmd::exec_internal_for_ensure(item_spec_graph, resources).await?;

        Ok(StatesEnsuredDry::from((states, resources)))
    }

    async fn ensure_op_spec_exec_dry(
        item_spec_graph: &ItemSpecGraph<E>,
        resources: &Resources<WithStateDiffs>,
        op_check_statuses: &OpCheckStatuses,
    ) -> Result<(), E> {
        Self::ensure_op_spec_stream(item_spec_graph, op_check_statuses)
            .try_for_each(|item_spec| async move { item_spec.ensure_op_exec_dry(resources).await })
            .await?;
        Ok(())
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
    /// 3. Fetch `States` again, and compare.
    ///
    /// State cannot be fetched interleaved with `exec` as it may use different
    /// `Data`.
    ///
    /// [`exec`]: peace_cfg::EnsureOpSpec::exec
    /// [`EnsureOpSpec::check`]: peace_cfg::EnsureOpSpec::check
    /// [`EnsureOpSpec::exec`]: peace_cfg::EnsureOpSpec::exec
    /// [`ItemSpec`]: peace_cfg::ItemSpec
    /// [`EnsureOpSpec`]: peace_cfg::ItemSpec::EnsureOpSpec
    pub async fn exec(cmd_context: CmdContext<'_, SetUp, E>) -> Result<CmdContext<Ensured, E>, E> {
        let cmd_context = DiffCmd::exec(cmd_context).await?;
        let (workspace, item_spec_graph, resources) = cmd_context.into_inner();
        let states_ensured = Self::exec_internal(&item_spec_graph, &resources).await?;

        let resources = Resources::<Ensured>::from((resources, states_ensured));
        let cmd_context = CmdContext::from((workspace, item_spec_graph, resources));
        Ok(cmd_context)
    }

    /// Conditionally runs [`EnsureOpSpec`]`::`[`exec`] for each [`ItemSpec`].
    ///
    /// Same as [`Self::exec`], but does not change the type state, and returns
    /// [`StatesEnsured`].
    ///
    /// [`exec`]: peace_cfg::EnsureOpSpec::exec
    /// [`ItemSpec`]: peace_cfg::ItemSpec
    /// [`EnsureOpSpec`]: peace_cfg::ItemSpec::EnsureOpSpec
    pub(crate) async fn exec_internal(
        item_spec_graph: &ItemSpecGraph<E>,
        resources: &Resources<WithStateDiffs>,
    ) -> Result<StatesEnsured, E> {
        let op_check_statuses = Self::ensure_op_spec_check(item_spec_graph, resources).await?;
        Self::ensure_op_spec_exec(item_spec_graph, resources, &op_check_statuses).await?;

        let states = StateCurrentCmd::exec_internal_for_ensure(item_spec_graph, resources).await?;

        Ok(StatesEnsured::from((states, resources)))
    }

    async fn ensure_op_spec_check(
        item_spec_graph: &ItemSpecGraph<E>,
        resources: &Resources<WithStateDiffs>,
    ) -> Result<OpCheckStatuses, E> {
        let op_check_statuses = item_spec_graph
            .stream()
            .map(Result::Ok)
            .and_then(|item_spec| async move {
                let op_check_status = item_spec.ensure_op_check(resources).await?;
                Ok((item_spec.id(), op_check_status))
            })
            .try_collect::<OpCheckStatuses>()
            .await?;

        Ok(op_check_statuses)
    }

    async fn ensure_op_spec_exec(
        item_spec_graph: &ItemSpecGraph<E>,
        resources: &Resources<WithStateDiffs>,
        op_check_statuses: &OpCheckStatuses,
    ) -> Result<(), E> {
        Self::ensure_op_spec_stream(item_spec_graph, op_check_statuses)
            .try_for_each(|item_spec| async move { item_spec.ensure_op_exec(resources).await })
            .await?;
        Ok(())
    }

    fn ensure_op_spec_stream<'f>(
        item_spec_graph: &'f ItemSpecGraph<E>,
        op_check_statuses: &'f OpCheckStatuses,
    ) -> impl TryStream<Ok = FnRef<'f, ItemSpecBoxed<E>>, Error = E> {
        item_spec_graph
            .stream()
            .filter(|item_spec| {
                let exec_required = op_check_statuses
                    .get(&item_spec.id())
                    .map(|op_check_status| {
                        matches!(op_check_status, OpCheckStatus::ExecRequired { .. })
                    })
                    .unwrap_or(true); // Should be unreachable, but we just execute if we get to this state.

                async move { exec_required }
            })
            .map(Result::Ok)
    }
}
