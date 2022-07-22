use std::ops::{Deref, DerefMut};

use fn_graph::FnGraph;
use futures::{StreamExt, TryStreamExt};
use peace_resources::{
    resources_type_state::{Empty, SetUp},
    Resources, StatesDesiredRw, StatesRw,
};

use crate::FullSpecBoxed;

/// Graph of all [`FullSpec`]s, `FnGraph<FullSpecBoxed<E>>` newtype.
///
/// [`FullSpec`]: peace_cfg::FullSpec
pub struct FullSpecGraph<E>(FnGraph<FullSpecBoxed<E>>)
where
    E: std::error::Error;

impl<E> FullSpecGraph<E>
where
    E: std::error::Error,
{
    /// Returns the inner [`FnGraph`].
    pub fn into_inner(self) -> FnGraph<FullSpecBoxed<E>> {
        self.0
    }

    /// Sets up [`Resources`] for the graph.
    ///
    /// # Parameters
    ///
    /// * `resources`: The resources to set up.
    pub async fn setup(&self, mut resources: Resources<Empty>) -> Result<Resources<SetUp>, E> {
        resources.insert(StatesRw::new());
        resources.insert(StatesDesiredRw::new());

        let resources = self
            .stream()
            .map(Ok::<_, E>)
            .try_fold(resources, |mut resources, full_spec| async move {
                full_spec.setup(&mut resources).await?;
                Ok(resources)
            })
            .await?;

        Ok(Resources::<SetUp>::from(resources.into_inner()))
    }
}

impl<E> Deref for FullSpecGraph<E>
where
    E: std::error::Error,
{
    type Target = FnGraph<FullSpecBoxed<E>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<E> DerefMut for FullSpecGraph<E>
where
    E: std::error::Error,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<E> From<FnGraph<FullSpecBoxed<E>>> for FullSpecGraph<E>
where
    E: std::error::Error,
{
    fn from(graph: FnGraph<FullSpecBoxed<E>>) -> Self {
        Self(graph)
    }
}
