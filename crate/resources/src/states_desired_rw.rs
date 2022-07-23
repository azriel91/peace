use std::ops::{Deref, DerefMut};

use tokio::sync::RwLock;

use crate::StatesDesired;

/// Atomic RW access to `StatesDesired`, `RwLock<StatesDesired>` newtype.
#[derive(Debug, Default)]
pub struct StatesDesiredRw(RwLock<StatesDesired>);

impl StatesDesiredRw {
    /// Returns a new [`StatesDesiredRw`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the inner `StatesDesired`.
    pub fn into_inner(self) -> StatesDesired {
        self.0.into_inner()
    }
}

impl Deref for StatesDesiredRw {
    type Target = RwLock<StatesDesired>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for StatesDesiredRw {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
