use std::marker::PhantomData;

use futures::stream::{StreamExt, TryStreamExt};
use peace_resources::{
    resources_type_state::{SetUp, WithStates},
    Resources,
};
use peace_rt_model::FullSpecGraph;

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
    /// `FullSpec`, the [`StatesRw`] type should be used in
    /// [`FnSpec::Data`].
    ///
    /// [`exec`]: peace_cfg::FnSpec::exec
    /// [`FnSpec::Data`]: peace_cfg::FnSpec::Data
    /// [`FullSpec`]: peace_cfg::FullSpec
    /// [`States`]: peace_resources::States
    /// [`StatesRw`]: peace_resources::StatesRw
    /// [`StateNowFnSpec`]: peace_cfg::FullSpec::StateNowFnSpec
    pub async fn exec(
        full_spec_graph: &FullSpecGraph<E>,
        resources: Resources<SetUp>,
    ) -> Result<Resources<WithStates>, E> {
        Self::exec_internal(full_spec_graph, &resources).await?;

        Ok(Resources::<WithStates>::from(resources))
    }

    /// Runs [`FullSpec`]`::`[`StateNowFnSpec`]`::`[`exec`] for each full spec.
    ///
    /// Same as [`Self::exec`], but does not change the type state.
    ///
    /// [`exec`]: peace_cfg::FnSpec::exec
    /// [`FullSpec`]: peace_cfg::FullSpec
    /// [`StateNowFnSpec`]: peace_cfg::FullSpec::StateNowFnSpec
    pub(crate) async fn exec_internal(
        full_spec_graph: &FullSpecGraph<E>,
        resources: &Resources<SetUp>,
    ) -> Result<(), E> {
        full_spec_graph
            .stream()
            .map(Result::<_, E>::Ok)
            .try_for_each_concurrent(None, |full_spec| async move {
                full_spec.state_now_fn_exec(resources).await
            })
            .await?;

        Ok(())
    }
}