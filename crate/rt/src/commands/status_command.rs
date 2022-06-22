use std::marker::PhantomData;

use futures::stream::{StreamExt, TryStreamExt};
use peace_data::Resources;
use peace_rt_model::{Error, FullSpecGraph};

#[derive(Debug)]
pub struct StatusCommand<E>(PhantomData<E>);

impl<E> StatusCommand<E>
where
    E: std::error::Error,
{
    pub async fn exec<'graph, 'res: 'graph>(
        full_spec_graph: FullSpecGraph<'graph, E>,
        resources: &'res Resources,
    ) -> Result<(), Error<E>> {
        full_spec_graph
            .stream()
            .map(Result::<_, Error<E>>::Ok)
            .try_for_each(|full_spec| async move { full_spec.status_fn_exec(resources).await })
            .await
    }
}
