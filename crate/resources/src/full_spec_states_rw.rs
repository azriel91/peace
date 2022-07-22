use std::ops::{Deref, DerefMut};

use tokio::sync::RwLock;

use crate::FullSpecStates;

/// Atomic RW access to `FullSpecStates`, `RwLock<FullSpecStates>` newtype.
#[derive(Debug, Default)]
pub struct FullSpecStatesRw(RwLock<FullSpecStates>);

impl FullSpecStatesRw {
    /// Returns a new [`FullSpecStatesRw`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the inner `FullSpecStates`.
    pub fn into_inner(self) -> FullSpecStates {
        self.0.into_inner()
    }
}

impl Deref for FullSpecStatesRw {
    type Target = RwLock<FullSpecStates>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for FullSpecStatesRw {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
