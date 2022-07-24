use std::ops::{Deref, DerefMut};

use tokio::sync::RwLock;

use crate::StatesDesiredMut;

/// Atomic RW access to `StatesDesiredMut`, `RwLock<StatesDesiredMut>` newtype.
#[derive(Debug, Default)]
pub struct StatesDesiredRw(RwLock<StatesDesiredMut>);

impl StatesDesiredRw {
    /// Returns a new [`StatesDesiredRw`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the inner `StatesDesiredMut`.
    pub fn into_inner(self) -> StatesDesiredMut {
        self.0.into_inner()
    }
}

impl Deref for StatesDesiredRw {
    type Target = RwLock<StatesDesiredMut>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for StatesDesiredRw {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
