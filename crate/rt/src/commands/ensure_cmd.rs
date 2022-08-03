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
use peace_rt_model::{FnRef, FullSpecBoxed, FullSpecGraph};

use crate::{DiffCmd, StateCurrentCmd};

#[derive(Debug)]
pub struct EnsureCmd<E>(PhantomData<E>);

impl<E> EnsureCmd<E>
where
    E: std::error::Error,
{
    /// Conditionally runs [`EnsureOpSpec`]`::`[`exec_dry`] for each
    /// [`FullSpec`].
    ///
    /// In practice this runs [`EnsureOpSpec::check`], and only runs
    /// [`exec_dry`] if execution is required.
    ///
    /// # Note
    ///
    /// To only make changes when they are *all* likely to work, we execute the
    /// functions as homogenous groups instead of interleaving the functions
    /// together per `FullSpec`:
    ///
    /// 1. Run [`EnsureOpSpec::check`] for all `FullSpec`s.
    /// 2. Run [`EnsureOpSpec::exec_dry`] for all `FullSpec`s.
    /// 3. Fetch `States` again, and compare.
    ///
    /// State cannot be fetched interleaved with `exec_dry` as it may use
    /// different `Data`.
    ///
    /// [`exec_dry`]: peace_cfg::EnsureOpSpec::exec
    /// [`EnsureOpSpec::check`]: peace_cfg::EnsureOpSpec::check
    /// [`EnsureOpSpec::exec_dry`]: peace_cfg::EnsureOpSpec::exec_dry
    /// [`FullSpec`]: peace_cfg::FullSpec
    /// [`EnsureOpSpec`]: peace_cfg::FullSpec::EnsureOpSpec
    pub async fn exec_dry(
        full_spec_graph: &FullSpecGraph<E>,
        resources: Resources<SetUp>,
    ) -> Result<Resources<EnsuredDry>, E> {
        let resources = DiffCmd::exec(full_spec_graph, resources).await?;
        let states_ensured_dry = Self::exec_dry_internal(full_spec_graph, &resources).await?;

        Ok(Resources::<EnsuredDry>::from((
            resources,
            states_ensured_dry,
        )))
    }

    /// Conditionally runs [`EnsureOpSpec`]`::`[`exec_dry`] for each
    /// [`FullSpec`].
    ///
    /// Same as [`Self::exec_dry`], but does not change the type state, and
    /// returns [`StatesEnsured`].
    ///
    /// [`exec_dry`]: peace_cfg::EnsureOpSpec::exec_dry
    /// [`FullSpec`]: peace_cfg::FullSpec
    /// [`EnsureOpSpec`]: peace_cfg::FullSpec::EnsureOpSpec
    pub(crate) async fn exec_dry_internal(
        full_spec_graph: &FullSpecGraph<E>,
        resources: &Resources<WithStateDiffs>,
    ) -> Result<StatesEnsuredDry, E> {
        let op_check_statuses = Self::ensure_op_spec_check(full_spec_graph, resources).await?;
        Self::ensure_op_spec_exec_dry(full_spec_graph, resources, &op_check_statuses).await?;

        let states = StateCurrentCmd::exec_internal_for_ensure(full_spec_graph, resources).await?;

        Ok(StatesEnsuredDry::from((states, resources)))
    }

    async fn ensure_op_spec_exec_dry(
        full_spec_graph: &FullSpecGraph<E>,
        resources: &Resources<WithStateDiffs>,
        op_check_statuses: &OpCheckStatuses,
    ) -> Result<(), E> {
        Self::ensure_op_spec_stream(full_spec_graph, op_check_statuses)
            .try_for_each(|full_spec| async move { full_spec.ensure_op_exec_dry(resources).await })
            .await?;
        Ok(())
    }

    /// Conditionally runs [`EnsureOpSpec`]`::`[`exec`] for each [`FullSpec`].
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
    /// functions as homogenous groups instead of interleaving the functions
    /// together per `FullSpec`:
    ///
    /// 1. Run [`EnsureOpSpec::check`] for all `FullSpec`s.
    /// 2. Run [`EnsureOpSpec::exec`] for all `FullSpec`s.
    /// 3. Fetch `States` again, and compare.
    ///
    /// State cannot be fetched interleaved with `exec` as it may use different
    /// `Data`.
    ///
    /// [`exec`]: peace_cfg::EnsureOpSpec::exec
    /// [`EnsureOpSpec::check`]: peace_cfg::EnsureOpSpec::check
    /// [`EnsureOpSpec::exec`]: peace_cfg::EnsureOpSpec::exec
    /// [`FullSpec`]: peace_cfg::FullSpec
    /// [`EnsureOpSpec`]: peace_cfg::FullSpec::EnsureOpSpec
    pub async fn exec(
        full_spec_graph: &FullSpecGraph<E>,
        resources: Resources<SetUp>,
    ) -> Result<Resources<Ensured>, E> {
        let resources = DiffCmd::exec(full_spec_graph, resources).await?;
        let states_ensured = Self::exec_internal(full_spec_graph, &resources).await?;

        Ok(Resources::<Ensured>::from((resources, states_ensured)))
    }

    /// Conditionally runs [`EnsureOpSpec`]`::`[`exec`] for each [`FullSpec`].
    ///
    /// Same as [`Self::exec`], but does not change the type state, and returns
    /// [`StatesEnsured`].
    ///
    /// [`exec`]: peace_cfg::EnsureOpSpec::exec
    /// [`FullSpec`]: peace_cfg::FullSpec
    /// [`EnsureOpSpec`]: peace_cfg::FullSpec::EnsureOpSpec
    pub(crate) async fn exec_internal(
        full_spec_graph: &FullSpecGraph<E>,
        resources: &Resources<WithStateDiffs>,
    ) -> Result<StatesEnsured, E> {
        let op_check_statuses = Self::ensure_op_spec_check(full_spec_graph, resources).await?;
        Self::ensure_op_spec_exec(full_spec_graph, resources, &op_check_statuses).await?;

        let states = StateCurrentCmd::exec_internal_for_ensure(full_spec_graph, resources).await?;

        Ok(StatesEnsured::from((states, resources)))
    }

    async fn ensure_op_spec_check(
        full_spec_graph: &FullSpecGraph<E>,
        resources: &Resources<WithStateDiffs>,
    ) -> Result<OpCheckStatuses, E> {
        let op_check_statuses = full_spec_graph
            .stream()
            .map(Result::Ok)
            .and_then(|full_spec| async move {
                let op_check_status = full_spec.ensure_op_check(resources).await?;
                Ok((full_spec.id(), op_check_status))
            })
            .try_collect::<OpCheckStatuses>()
            .await?;

        Ok(op_check_statuses)
    }

    async fn ensure_op_spec_exec(
        full_spec_graph: &FullSpecGraph<E>,
        resources: &Resources<WithStateDiffs>,
        op_check_statuses: &OpCheckStatuses,
    ) -> Result<(), E> {
        Self::ensure_op_spec_stream(full_spec_graph, op_check_statuses)
            .try_for_each(|full_spec| async move { full_spec.ensure_op_exec(resources).await })
            .await?;
        Ok(())
    }

    fn ensure_op_spec_stream<'f>(
        full_spec_graph: &'f FullSpecGraph<E>,
        op_check_statuses: &'f OpCheckStatuses,
    ) -> impl TryStream<Ok = FnRef<'f, FullSpecBoxed<E>>, Error = E> {
        full_spec_graph
            .stream()
            .filter(|full_spec| {
                let exec_required = op_check_statuses
                    .get(&full_spec.id())
                    .map(|op_check_status| {
                        matches!(op_check_status, OpCheckStatus::ExecRequired { .. })
                    })
                    .unwrap_or(true); // Should be unreachable, but we just execute if we get to this state.

                async move { exec_required }
            })
            .map(Result::Ok)
    }
}
