use std::marker::PhantomData;

use futures::stream::{StreamExt, TryStreamExt};
use peace_resources::{
    resources_type_state::{SetUp, WithStates},
    Resources, States, StatesMut,
};
use peace_rt_model::FullSpecGraph;

use crate::BUFFERED_FUTURES_MAX;

#[derive(Debug)]
pub struct StateNowCmd<E>(PhantomData<E>);

impl<E> StateNowCmd<E>
where
    E: std::error::Error,
{
    /// Runs [`FullSpec`]`::`[`StateNowFnSpec`]`::`[`exec`] for each full spec.
    ///
    /// At the end of this function, [`Resources`] will be populated with
    /// [`States`].
    ///
    /// If any `StateNowFnSpec` needs to read the `State` from a previous
    /// `FullSpec`, the predecessor should insert a copy / clone of their state
    /// into `Resources`, and the successor should references it in their
    /// [`FnSpec::Data`].
    ///
    /// [`exec`]: peace_cfg::FnSpec::exec
    /// [`FnSpec::Data`]: peace_cfg::FnSpec::Data
    /// [`FullSpec`]: peace_cfg::FullSpec
    /// [`StateNowFnSpec`]: peace_cfg::FullSpec::StateNowFnSpec
    pub async fn exec(
        full_spec_graph: &FullSpecGraph<E>,
        resources: Resources<SetUp>,
    ) -> Result<Resources<WithStates>, E> {
        let states = Self::exec_internal(full_spec_graph, &resources).await?;

        Ok(Resources::<WithStates>::from((resources, states)))
    }

    /// Runs [`FullSpec`]`::`[`StateNowFnSpec`]`::`[`exec`] for each full spec.
    ///
    /// Same as [`Self::exec`], but does not change the type state, and returns
    /// [`States`].
    ///
    /// [`exec`]: peace_cfg::FnSpec::exec
    /// [`FullSpec`]: peace_cfg::FullSpec
    /// [`StateNowFnSpec`]: peace_cfg::FullSpec::StateNowFnSpec
    pub(crate) async fn exec_internal(
        full_spec_graph: &FullSpecGraph<E>,
        resources: &Resources<SetUp>,
    ) -> Result<States, E> {
        let states_mut = full_spec_graph
            .stream()
            .map(Result::Ok)
            .map_ok(|full_spec| async move {
                let state = full_spec.state_now_fn_exec(resources).await?;
                Ok((full_spec.id(), state))
            })
            .try_buffer_unordered(BUFFERED_FUTURES_MAX)
            .try_collect::<StatesMut>()
            .await?;

        Ok(States::from(states_mut))
    }
}
