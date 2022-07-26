use std::marker::PhantomData;

use futures::stream::{StreamExt, TryStreamExt};
use peace_resources::{
    resources_type_state::{SetUp, WithStatesDesired},
    Resources,
};
use peace_rt_model::FullSpecGraph;

#[derive(Debug)]
pub struct StateDesiredCmd<E>(PhantomData<E>);

impl<E> StateDesiredCmd<E>
where
    E: std::error::Error,
{
    /// Runs [`FullSpec`]`::`[`StateDesiredFnSpec`]`::`[`exec`] for each full
    /// spec.
    ///
    /// At the end of this function, [`Resources`] will be populated with
    /// [`StatesDesired`].
    ///
    /// If any `StateDesiredFnSpec` needs to read the desired `State` from a
    /// previous `FullSpec`, the [`StatesDesiredRw`] type should be used in
    /// [`FnSpec::Data`].
    ///
    /// [`exec`]: peace_cfg::StateDesiredFnSpec::exec
    /// [`StateDesiredFnSpec::Data`]: peace_cfg::StateDesiredFnSpec::Data
    /// [`FullSpec`]: peace_cfg::FullSpec
    /// [`StatesDesired`]: peace_resources::StatesDesired
    /// [`StateDesiredFnSpec`]: peace_cfg::FullSpec::StateDesiredFnSpec
    pub async fn exec(
        full_spec_graph: &FullSpecGraph<E>,
        resources: Resources<SetUp>,
    ) -> Result<Resources<WithStatesDesired>, E> {
        let resources_ref = &resources;
        full_spec_graph
            .stream()
            .map(Result::<_, E>::Ok)
            .try_for_each_concurrent(None, |full_spec| async move {
                full_spec.state_desired_fn_exec(resources_ref).await
            })
            .await?;

        Ok(Resources::<WithStatesDesired>::from(resources))
    }
}
