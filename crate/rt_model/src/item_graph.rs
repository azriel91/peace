use std::ops::{Deref, DerefMut};

use peace_data::fn_graph::FnGraph;

use crate::ItemBoxed;

/// Graph of all [`Item`]s, `FnGraph<ItemBoxed<E>>` newtype.
///
/// [`Item`]: peace_cfg::Item
#[derive(Debug)]
pub struct ItemGraph<E>(FnGraph<ItemBoxed<E>>);

// Manual implementation because derive requires `E` to be `Clone`,
// which causes `graph.clone()` to call `FnGraph::clone`.
impl<E> Clone for ItemGraph<E> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<E> PartialEq for ItemGraph<E>
where
    E: 'static,
{
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<E> Eq for ItemGraph<E> where E: 'static {}

impl<E> ItemGraph<E> {
    /// Returns the inner [`FnGraph`].
    pub fn into_inner(self) -> FnGraph<ItemBoxed<E>> {
        self.0
    }
}

impl<E> Deref for ItemGraph<E> {
    type Target = FnGraph<ItemBoxed<E>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<E> DerefMut for ItemGraph<E> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<E> From<FnGraph<ItemBoxed<E>>> for ItemGraph<E> {
    fn from(graph: FnGraph<ItemBoxed<E>>) -> Self {
        Self(graph)
    }
}
