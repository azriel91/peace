use std::ops::{Deref, DerefMut};

use tokio::sync::RwLock;

use crate::FullSpecStatesDesired;

/// Atomic RW access to `FullSpecStatesDesired`, `RwLock<FullSpecStatesDesired>` newtype.
#[derive(Debug, Default)]
pub struct FullSpecStatesDesiredRw(RwLock<FullSpecStatesDesired>);

impl FullSpecStatesDesiredRw {
    /// Returns a new [`FullSpecStatesDesiredRw`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the inner `FullSpecStatesDesired`.
    pub fn into_inner(self) -> FullSpecStatesDesired {
        self.0.into_inner()
    }
}

impl Deref for FullSpecStatesDesiredRw {
    type Target = RwLock<FullSpecStatesDesired>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for FullSpecStatesDesiredRw {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
