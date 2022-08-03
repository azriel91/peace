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
    /// Runs [`FullSpec`]`::`[`EnsureOpSpec`]`::`[`exec`] for each full
    /// spec.
    ///
    /// At the end of this function, [`Resources`] will be populated with
    /// [`States`].
    ///
    /// If any `EnsureOpSpec` needs to read the `State` from a previous
    /// `FullSpec`, the predecessor should insert a copy / clone of their state
    /// into `Resources`, and the successor should references it in their
    /// [`EnsureOpSpec::Data`].
    ///
    /// [`exec`]: peace_cfg::EnsureOpSpec::exec
    /// [`EnsureOpSpec::Data`]: peace_cfg::EnsureOpSpec::Data
    /// [`FullSpec`]: peace_cfg::FullSpec
    /// [`EnsureOpSpec`]: peace_cfg::FullSpec::EnsureOpSpec
    pub async fn exec(
        full_spec_graph: &FullSpecGraph<E>,
        resources: Resources<WithStateDiffs>,
    ) -> Result<Resources<Ensured>, E> {
        let states_ensured = Self::exec_internal(full_spec_graph, &resources).await?;

        Ok(Resources::<Ensured>::from((resources, states_ensured)))
    }

    /// Runs [`FullSpec`]`::`[`EnsureOpSpec`]`::`[`exec`] for each full
    /// spec.
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
        // To only make changes when they are *all* likely to work, we execute the
        // functions as homogenous groups instead of interleaving the functions together
        // per `FullSpec`:
        //
        // 1. Run `EnsureOpSpec::check` for all `FullSpec`s.
        // 2. Run `EnsureOpSpec`::`exec` for all `FullSpec`s.
        // 3. Fetch `States` again, and compare.
        //
        // State cannot be fetched interleaved with `exec` as it may use different
        // `Data`.
        let _op_check_statuses = full_spec_graph
            .stream()
            .map(Result::Ok)
            .and_then(|full_spec| async move {
                let op_check_status: OpCheckStatus = full_spec.ensure_op_check(resources).await?;
                Ok((full_spec.id(), op_check_status))
            })
            .try_collect::<OpCheckStatuses>()
            .await?;

        // TODO: re-fetch states and pass in.
        Ok(())
    }
}
