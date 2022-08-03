use std::marker::PhantomData;

use futures::stream::{StreamExt, TryStreamExt};
use peace_cfg::OpCheckStatus;
use peace_resources::{
    internal::OpCheckStatuses,
    resources_type_state::{Ensured, WithStateDiffs},
    Resources,
};
use peace_rt_model::FullSpecGraph;

#[derive(Debug)]
pub struct EnsureCmd<E>(PhantomData<E>);

impl<E> EnsureCmd<E>
where
    E: std::error::Error,
{
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
        resources: Resources<WithStateDiffs>,
    ) -> Result<Resources<Ensured>, E> {
        let states_ensured = Self::exec_internal(full_spec_graph, &resources).await?;

        Ok(Resources::<Ensured>::from((resources, states_ensured)))
    }

    /// Runs [`EnsureOpSpec`]`::`[`exec`] for each [`FullSpec`].
    ///
    /// Same as [`Self::exec`], but does not change the type state, and returns
    /// [`States`].
    ///
    /// [`exec`]: peace_cfg::EnsureOpSpec::exec
    /// [`FullSpec`]: peace_cfg::FullSpec
    /// [`EnsureOpSpec`]: peace_cfg::FullSpec::EnsureOpSpec
    pub(crate) async fn exec_internal(
        full_spec_graph: &FullSpecGraph<E>,
        resources: &Resources<WithStateDiffs>,
    ) -> Result<(), E> {
        let op_check_statuses = Self::ensure_op_spec_check(full_spec_graph, resources).await?;
        Self::ensure_op_spec_exec(full_spec_graph, resources, &op_check_statuses).await?;

        // TODO: re-fetch states and pass in.
        Ok(())
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
            .try_for_each(|full_spec| async move { full_spec.ensure_op_exec(resources).await })
            .await?;
        Ok(())
    }
}
