use std::marker::PhantomData;

use futures::stream::{StreamExt, TryStreamExt};
use peace_resources::{resources_type_state::SetUp, Resources};
use peace_rt_model::FullSpecGraph;

#[derive(Debug)]
pub struct StatusCommand<E>(PhantomData<E>);

impl<E> StatusCommand<E>
where
    E: std::error::Error,
{
    /// Runs [`FullSpec`]`::`[`StatusFnSpec`]`::`[`exec`] for each full spec.
    ///
    /// At the end of this function, [`Resources`] will be populated with
    /// [`States`].
    ///
    /// If any `StatusFnSpec` needs to read the `State` from a previous
    /// `FullSpec`, the [`StatesRw`] type should be used in
    /// [`FnSpec::Data`].
    ///
    /// [`exec`]: peace_cfg::FnSpec::exec
    /// [`FnSpec::Data`]: peace_cfg::FnSpec::Data
    /// [`FullSpec`]: peace_cfg::FullSpec
    /// [`States`]: peace_resources::States
    /// [`StatesRw`]: peace_resources::StatesRw
    /// [`StatusFnSpec`]: peace_cfg::FullSpec::StatusFnSpec
    pub async fn exec(
        full_spec_graph: &FullSpecGraph<E>,
        resources: &Resources<SetUp>,
    ) -> Result<(), E> {
        full_spec_graph
            .stream()
            .map(Result::<_, E>::Ok)
            .try_for_each_concurrent(None, |full_spec| async move {
                full_spec.status_fn_exec(resources).await
            })
            .await
    }
}
