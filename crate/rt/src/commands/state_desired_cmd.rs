use std::marker::PhantomData;

use futures::stream::{StreamExt, TryStreamExt};
use peace_resources::{
    resources_type_state::{SetUp, WithStatesDesired},
    Resources, StatesDesired, StatesDesiredMut,
};
use peace_rt_model::FullSpecGraph;

use crate::BUFFERED_FUTURES_MAX;

#[derive(Debug)]
pub struct StateDesiredCmd<E>(PhantomData<E>);

impl<E> StateDesiredCmd<E>
where
    E: std::error::Error,
{
    /// Runs [`StateDesiredFnSpec`]`::`[`exec`] for each [`FullSpec`].
    ///
    /// At the end of this function, [`Resources`] will be populated with
    /// [`StatesDesired`].
    ///
    /// If any `StateDesiredFnSpec` needs to read the `State` from a previous
    /// `FullSpec`, the predecessor should insert a copy / clone of their
    /// desired state into `Resources`, and the successor should references
    /// it in their [`FnSpec::Data`].
    ///
    /// [`exec`]: peace_cfg::StateDesiredFnSpec::exec
    /// [`FnSpec::Data`]: peace_cfg::FnSpec::Data
    /// [`FullSpec`]: peace_cfg::FullSpec
    /// [`StatesDesired`]: peace_resources::StatesDesired
    /// [`StateDesiredFnSpec`]: peace_cfg::FullSpec::StateDesiredFnSpec
    pub async fn exec(
        full_spec_graph: &FullSpecGraph<E>,
        resources: Resources<SetUp>,
    ) -> Result<Resources<WithStatesDesired>, E> {
        let states_desired = Self::exec_internal(full_spec_graph, &resources).await?;

        Ok(Resources::<WithStatesDesired>::from((
            resources,
            states_desired,
        )))
    }

    /// Runs [`StateDesiredFnSpec`]`::`[`exec`] for each [`FullSpec`].
    ///
    /// Same as [`Self::exec`], but does not change the type state.
    ///
    /// [`exec`]: peace_cfg::StateDesiredFnSpec::exec
    /// [`FullSpec`]: peace_cfg::FullSpec
    /// [`StateDesiredFnSpec`]: peace_cfg::FullSpec::StateDesiredFnSpec
    pub(crate) async fn exec_internal(
        full_spec_graph: &FullSpecGraph<E>,
        resources: &Resources<SetUp>,
    ) -> Result<StatesDesired, E> {
        let states_desired_mut = full_spec_graph
            .stream()
            .map(Result::Ok)
            .map_ok(|full_spec| async move {
                let state_desired = full_spec.state_desired_fn_exec(resources).await?;
                Ok((full_spec.id(), state_desired))
            })
            .try_buffer_unordered(BUFFERED_FUTURES_MAX)
            .try_collect::<StatesDesiredMut>()
            .await?;

        Ok(StatesDesired::from(states_desired_mut))
    }
}
