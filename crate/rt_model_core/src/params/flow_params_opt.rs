use std::{
    hash::Hash,
    marker::PhantomData,
    ops::{Deref, DerefMut},
};

use serde::Serialize;
use type_reg::untagged::{BoxDt, TypeMapOpt};

/// Information that is applicable to a flow. `TypeMapOpt<K>` newtype.
///
/// This is used to keep track of [`FlowParams`] that need to be removed when
/// building a `CmdCtx*`.
///
/// [`FlowParams`]: crate::params::FlowParams
///
/// # Type Parameters
///
/// * `K`: Type of key for the `FlowParamsOpt` map.
#[derive(Clone, Debug, Serialize)]
#[serde(transparent)] // Needed to serialize as a map instead of a list.
pub struct FlowParamsOpt<K>(TypeMapOpt<K, BoxDt>, PhantomData<K>)
where
    K: Eq + Hash;

impl<K> FlowParamsOpt<K>
where
    K: Eq + Hash,
{
    /// Returns a new `FlowParamsOpt` map.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates an empty `FlowParamsOpt` map with the specified capacity.
    ///
    /// The `FlowParamsOpt` will be able to hold at least capacity elements
    /// without reallocating. If capacity is 0, the map will not allocate.
    pub fn with_capacity(capacity: usize) -> Self {
        Self(TypeMapOpt::with_capacity_typed(capacity), PhantomData)
    }

    /// Returns the inner map.
    pub fn into_inner(self) -> TypeMapOpt<K, BoxDt> {
        self.0
    }
}

impl<K> Default for FlowParamsOpt<K>
where
    K: Eq + Hash,
{
    fn default() -> Self {
        Self(TypeMapOpt::default(), PhantomData)
    }
}

impl<K> Deref for FlowParamsOpt<K>
where
    K: Eq + Hash,
{
    type Target = TypeMapOpt<K, BoxDt>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<K> DerefMut for FlowParamsOpt<K>
where
    K: Eq + Hash,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<K> From<TypeMapOpt<K, BoxDt>> for FlowParamsOpt<K>
where
    K: Eq + Hash,
{
    fn from(type_map: TypeMapOpt<K, BoxDt>) -> Self {
        Self(type_map, PhantomData)
    }
}
