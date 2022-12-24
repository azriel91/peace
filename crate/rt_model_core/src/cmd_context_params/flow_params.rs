use std::{
    hash::Hash,
    marker::PhantomData,
    ops::{Deref, DerefMut},
};

use serde::Serialize;
use type_reg::untagged::{BoxDt, TypeMap};

/// Information that is applicable to a flow. `TypeMap<K>` newtype.
///
/// The information may not be of the same type across flows, as flows are
/// different in what they are doing. Example information include:
///
/// * Server count: applicable to `deploy`
/// * Force remove: applicable to `clean`
///
/// # Type Parameters
///
/// * `K`: Type of key for the `FlowParams` map.
#[derive(Debug, Serialize)]
#[serde(transparent)] // Needed to serialize as a map instead of a list.
pub struct FlowParams<K>(TypeMap<K, BoxDt>, PhantomData<K>)
where
    K: Eq + Hash;

impl<K> FlowParams<K>
where
    K: Eq + Hash,
{
    /// Returns a new `FlowParams` map.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates an empty `FlowParams` map with the specified capacity.
    ///
    /// The `FlowParams` will be able to hold at least capacity elements
    /// without reallocating. If capacity is 0, the map will not allocate.
    pub fn with_capacity(capacity: usize) -> Self {
        Self(TypeMap::with_capacity_typed(capacity), PhantomData)
    }

    /// Returns the inner map.
    pub fn into_inner(self) -> TypeMap<K, BoxDt> {
        self.0
    }
}

impl<K> Default for FlowParams<K>
where
    K: Eq + Hash,
{
    fn default() -> Self {
        Self(TypeMap::default(), PhantomData)
    }
}

impl<K> Deref for FlowParams<K>
where
    K: Eq + Hash,
{
    type Target = TypeMap<K, BoxDt>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<K> DerefMut for FlowParams<K>
where
    K: Eq + Hash,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<K> From<TypeMap<K, BoxDt>> for FlowParams<K>
where
    K: Eq + Hash,
{
    fn from(type_map: TypeMap<K, BoxDt>) -> Self {
        Self(type_map, PhantomData)
    }
}
