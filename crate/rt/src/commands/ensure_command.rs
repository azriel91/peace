use std::marker::PhantomData;

use futures::stream::{StreamExt, TryStreamExt};
use peace_resources::{resources_type_state::SetUp, Resources};
use peace_rt_model::FullSpecGraph;

#[derive(Debug)]
pub struct EnsureCommand<E>(PhantomData<E>);

impl<E> EnsureCommand<E>
where
    E: std::error::Error,
{
    /// Runs [`FullSpec`]`::`[`EnsureOpSpec`]`::`[`desired`] for each full spec.
    ///
    /// At the end of this function, [`Resources`] will be populated with
    /// [`FullSpecStatesDesired`].
    ///
    /// [`desired`]: peace_cfg::EnsureOpSpec::desired
    /// [`EnsureOpSpec::Data`]: peace_cfg::EnsureOpSpec::Data
    /// [`FullSpec`]: peace_cfg::FullSpec
    /// [`FullSpecStatesDesired`]: peace_resources::FullSpecStatesDesired
    /// [`EnsureOpSpec`]: peace_cfg::FullSpec::EnsureOpSpec
    pub async fn desired(
        full_spec_graph: &FullSpecGraph<E>,
        resources: &Resources<SetUp>,
    ) -> Result<(), E> {
        full_spec_graph
            .stream()
            .map(Result::<_, E>::Ok)
            .try_for_each_concurrent(None, |full_spec| async move {
                full_spec.ensure_op_desired(resources).await
            })
            .await
    }
}
