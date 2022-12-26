use std::{
    hash::Hash,
    marker::PhantomData,
    ops::{Deref, DerefMut},
};

use serde::Serialize;
use type_reg::untagged::{BoxDt, TypeMap};

/// Information that is shared across flows within a profile. `TypeMap<K>`
/// newtype.
///
/// Shared information are the ones that will not change when using different
/// flows. For example, deploying a set of servers, or exporting configuration
/// from those servers will use the same values for the following:
///
/// * Profile name
/// * Server hostnames
///
/// # Type Parameters
///
/// * `K`: Type of key for the `ProfileParams` map.
#[derive(Debug, Serialize)]
#[serde(transparent)] // Needed to serialize as a map instead of a list.
pub struct ProfileParams<K>(TypeMap<K, BoxDt>, PhantomData<K>)
where
    K: Eq + Hash;

impl<K> ProfileParams<K>
where
    K: Eq + Hash,
{
    /// Returns a new `ProfileParams` map.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates an empty `ProfileParams` map with the specified capacity.
    ///
    /// The `ProfileParams` will be able to hold at least capacity elements
    /// without reallocating. If capacity is 0, the map will not allocate.
    pub fn with_capacity(capacity: usize) -> Self {
        Self(TypeMap::with_capacity_typed(capacity), PhantomData)
    }

    /// Returns the inner map.
    pub fn into_inner(self) -> TypeMap<K, BoxDt> {
        self.0
    }
}

impl<K> Default for ProfileParams<K>
where
    K: Eq + Hash,
{
    fn default() -> Self {
        Self(TypeMap::default(), PhantomData)
    }
}

impl<K> Deref for ProfileParams<K>
where
    K: Eq + Hash,
{
    type Target = TypeMap<K, BoxDt>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<K> DerefMut for ProfileParams<K>
where
    K: Eq + Hash,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<K> From<TypeMap<K, BoxDt>> for ProfileParams<K>
where
    K: Eq + Hash,
{
    fn from(type_map: TypeMap<K, BoxDt>) -> Self {
        Self(type_map, PhantomData)
    }
}
