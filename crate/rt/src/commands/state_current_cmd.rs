use std::marker::PhantomData;

use futures::stream::{StreamExt, TryStreamExt};
use peace_resources::{
    resources_type_state::{SetUp, WithStateDiffs, WithStates},
    Resources, States, StatesMut,
};
use peace_rt_model::FullSpecGraph;

use crate::BUFFERED_FUTURES_MAX;

#[derive(Debug)]
pub struct StateCurrentCmd<E>(PhantomData<E>);

impl<E> StateCurrentCmd<E>
where
    E: std::error::Error,
{
    /// Runs [`StateCurrentFnSpec`]`::`[`exec`] for each [`FullSpec`].
    ///
    /// At the end of this function, [`Resources`] will be populated with
    /// [`States`].
    ///
    /// If any `StateCurrentFnSpec` needs to read the `State` from a previous
    /// `FullSpec`, the predecessor should insert a copy / clone of their state
    /// into `Resources`, and the successor should references it in their
    /// [`FnSpec::Data`].
    ///
    /// [`exec`]: peace_cfg::FnSpec::exec
    /// [`FnSpec::Data`]: peace_cfg::FnSpec::Data
    /// [`FullSpec`]: peace_cfg::FullSpec
    /// [`StateCurrentFnSpec`]: peace_cfg::FullSpec::StateCurrentFnSpec
    pub async fn exec(
        full_spec_graph: &FullSpecGraph<E>,
        resources: Resources<SetUp>,
    ) -> Result<Resources<WithStates>, E> {
        let states = Self::exec_internal(full_spec_graph, &resources).await?;

        Ok(Resources::<WithStates>::from((resources, states)))
    }

    /// Runs [`StateCurrentFnSpec`]`::`[`exec`] for each [`FullSpec`].
    ///
    /// Same as [`Self::exec`], but does not change the type state, and returns
    /// [`States`].
    ///
    /// [`exec`]: peace_cfg::FnSpec::exec
    /// [`FullSpec`]: peace_cfg::FullSpec
    /// [`StateCurrentFnSpec`]: peace_cfg::FullSpec::StateCurrentFnSpec
    pub(crate) async fn exec_internal(
        full_spec_graph: &FullSpecGraph<E>,
        resources: &Resources<SetUp>,
    ) -> Result<States, E> {
        let states_mut = full_spec_graph
            .stream()
            .map(Result::Ok)
            .map_ok(|full_spec| async move {
                let state = full_spec.state_current_fn_exec(resources).await?;
                Ok((full_spec.id(), state))
            })
            .try_buffer_unordered(BUFFERED_FUTURES_MAX)
            .try_collect::<StatesMut>()
            .await?;

        Ok(States::from(states_mut))
    }

    /// Runs [`StateCurrentFnSpec`]`::`[`exec`] for each [`FullSpec`].
    ///
    /// Same as [`Self::exec`], but does not change the type state, and returns
    /// [`States`].
    ///
    /// [`exec`]: peace_cfg::FnSpec::exec
    /// [`FullSpec`]: peace_cfg::FullSpec
    /// [`StateCurrentFnSpec`]: peace_cfg::FullSpec::StateCurrentFnSpec
    pub(crate) async fn exec_internal_for_ensure(
        full_spec_graph: &FullSpecGraph<E>,
        resources: &Resources<WithStateDiffs>,
    ) -> Result<States, E> {
        let states_mut = full_spec_graph
            .stream()
            .map(Result::Ok)
            .map_ok(|full_spec| async move {
                let state = full_spec.state_ensured_fn_exec(resources).await?;
                Ok((full_spec.id(), state))
            })
            .try_buffer_unordered(BUFFERED_FUTURES_MAX)
            .try_collect::<StatesMut>()
            .await?;

        Ok(States::from(states_mut))
    }
}
