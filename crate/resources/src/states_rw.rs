use std::ops::{Deref, DerefMut};

use tokio::sync::RwLock;

use crate::StatesMut;

/// Atomic RW access to `StatesMut`, `RwLock<StatesMut>` newtype.
#[derive(Debug, Default)]
pub struct StatesRw(RwLock<StatesMut>);

impl StatesRw {
    /// Returns a new [`StatesRw`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the inner `StatesMut`.
    pub fn into_inner(self) -> StatesMut {
        self.0.into_inner()
    }
}

impl Deref for StatesRw {
    type Target = RwLock<StatesMut>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for StatesRw {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
