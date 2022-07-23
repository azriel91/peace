use std::{
    marker::PhantomData,
    ops::{Deref, DerefMut},
};

use crate::{resources_type_state::Empty, StatesDesiredRw, StatesRw};

/// Map of all types at runtime. [`resman::Resources`] newtype.
///
/// This augments the any-map functionality of [`resman::Resources`] with type
/// state, so that it is impossible for developers to pass `Resources` to
/// functions that require particular operations to have executed over the
/// resources beforehand.
///
/// For example, `Resources` must be `setup` before any `FnSpec` or `OpSpec` may
/// execute with it.
///
/// # Type Parameters
///
/// * `TS`: The type state of the `Resources` map.
///
/// [`FullSpecId`]: peace_cfg::FullSpecId
#[derive(Debug)]
pub struct Resources<TS> {
    inner: resman::Resources,
    marker: PhantomData<TS>,
}

impl Resources<Empty> {
    /// Returns a new `Resources`.
    pub fn new() -> Self {
        let mut inner = resman::Resources::new();
        inner.insert(StatesRw::new());
        inner.insert(StatesDesiredRw::new());

        Self {
            inner,
            marker: PhantomData,
        }
    }
}

// Common impl for all type states.
impl<TS> Resources<TS> {
    /// Returns the inner [`resman::Resources`].
    pub fn into_inner(self) -> resman::Resources {
        self.inner
    }
}

impl Default for Resources<Empty> {
    fn default() -> Self {
        Self::new()
    }
}

impl<TS> Deref for Resources<TS> {
    type Target = resman::Resources;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<TS> DerefMut for Resources<TS> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl<TS> From<resman::Resources> for Resources<TS> {
    fn from(inner: resman::Resources) -> Self {
        Self {
            inner,
            marker: PhantomData,
        }
    }
}
