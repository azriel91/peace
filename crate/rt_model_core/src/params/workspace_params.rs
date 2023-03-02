use std::{
    hash::Hash,
    marker::PhantomData,
    ops::{Deref, DerefMut},
};

use serde::Serialize;
use type_reg::untagged::{BoxDt, TypeMap};

/// Information that is shared across all profiles and flows in a workspace.
/// `TypeMap<K>` newtype.
///
/// Shared information are the ones that will not change when switching to
/// different profiles. For example, a user working on a project for a
/// particular customer may use the following information across profiles:
///
/// * User ID
/// * Customer ID
///
/// # Type Parameters
///
/// * `K`: Type of key for the `WorkspaceParams` map.
#[derive(Clone, Debug, Serialize)]
#[serde(transparent)] // Needed to serialize as a map instead of a list.
pub struct WorkspaceParams<K>(TypeMap<K, BoxDt>, PhantomData<K>)
where
    K: Eq + Hash;

impl<K> WorkspaceParams<K>
where
    K: Eq + Hash,
{
    /// Returns a new `WorkspaceParams` map.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates an empty `WorkspaceParams` map with the specified capacity.
    ///
    /// The `WorkspaceParams` will be able to hold at least capacity elements
    /// without reallocating. If capacity is 0, the map will not allocate.
    pub fn with_capacity(capacity: usize) -> Self {
        Self(TypeMap::with_capacity_typed(capacity), PhantomData)
    }

    /// Returns the inner map.
    pub fn into_inner(self) -> TypeMap<K, BoxDt> {
        self.0
    }
}

impl<K> Default for WorkspaceParams<K>
where
    K: Eq + Hash,
{
    fn default() -> Self {
        Self(TypeMap::default(), PhantomData)
    }
}

impl<K> Deref for WorkspaceParams<K>
where
    K: Eq + Hash,
{
    type Target = TypeMap<K, BoxDt>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<K> DerefMut for WorkspaceParams<K>
where
    K: Eq + Hash,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<K> From<TypeMap<K, BoxDt>> for WorkspaceParams<K>
where
    K: Eq + Hash,
{
    fn from(type_map: TypeMap<K, BoxDt>) -> Self {
        Self(type_map, PhantomData)
    }
}
