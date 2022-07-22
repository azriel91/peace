use std::marker::PhantomData;

use futures::stream::{StreamExt, TryStreamExt};
use peace_resources::{resources_type_state::SetUp, Resources};
use peace_rt_model::FullSpecGraph;

#[derive(Debug)]
pub struct StatusDesiredCommand<E>(PhantomData<E>);

impl<E> StatusDesiredCommand<E>
where
    E: std::error::Error,
{
    /// Runs [`FullSpec`]`::`[`StatusDesiredFnSpec`]`::`[`exec`] for each full
    /// spec.
    ///
    /// At the end of this function, [`Resources`] will be populated with
    /// [`FullSpecStatesDesired`].
    ///
    /// [`exec`]: peace_cfg::StatusDesiredFnSpec::exec
    /// [`StatusDesiredFnSpec::Data`]: peace_cfg::StatusDesiredFnSpec::Data
    /// [`FullSpec`]: peace_cfg::FullSpec
    /// [`FullSpecStatesDesired`]: peace_resources::FullSpecStatesDesired
    /// [`StatusDesiredFnSpec`]: peace_cfg::FullSpec::StatusDesiredFnSpec
    pub async fn exec(
        full_spec_graph: &FullSpecGraph<E>,
        resources: &Resources<SetUp>,
    ) -> Result<(), E> {
        full_spec_graph
            .stream()
            .map(Result::<_, E>::Ok)
            .try_for_each_concurrent(None, |full_spec| async move {
                full_spec.status_desired_fn_exec(resources).await
            })
            .await
    }
}
