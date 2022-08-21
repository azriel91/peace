use std::ops::{Deref, DerefMut};

use fn_graph::FnGraph;
use futures::{StreamExt, TryStreamExt};
use peace_resources::{
    resources_type_state::{Empty, SetUp},
    Resources,
};

use crate::ItemSpecBoxed;

/// Graph of all [`ItemSpec`]s, `FnGraph<ItemSpecBoxed<E>>` newtype.
///
/// [`ItemSpec`]: peace_cfg::ItemSpec
#[derive(Debug)]
pub struct ItemSpecGraph<E>(FnGraph<ItemSpecBoxed<E>>)
where
    E: std::error::Error;

impl<E> ItemSpecGraph<E>
where
    E: std::error::Error,
{
    /// Returns the inner [`FnGraph`].
    pub fn into_inner(self) -> FnGraph<ItemSpecBoxed<E>> {
        self.0
    }

    /// Sets up [`Resources`] for the graph.
    ///
    /// # Parameters
    ///
    /// * `resources`: The resources to set up.
    pub async fn setup(&self, resources: Resources<Empty>) -> Result<Resources<SetUp>, E> {
        let resources = self
            .stream()
            .map(Ok::<_, E>)
            .try_fold(resources, |mut resources, item_spec| async move {
                item_spec.setup(&mut resources).await?;
                Ok(resources)
            })
            .await?;

        Ok(Resources::<SetUp>::from(resources))
    }
}

impl<E> Deref for ItemSpecGraph<E>
where
    E: std::error::Error,
{
    type Target = FnGraph<ItemSpecBoxed<E>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<E> DerefMut for ItemSpecGraph<E>
where
    E: std::error::Error,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<E> From<FnGraph<ItemSpecBoxed<E>>> for ItemSpecGraph<E>
where
    E: std::error::Error,
{
    fn from(graph: FnGraph<ItemSpecBoxed<E>>) -> Self {
        Self(graph)
    }
}
