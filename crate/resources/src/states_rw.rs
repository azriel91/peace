use std::ops::{Deref, DerefMut};

use tokio::sync::RwLock;

use crate::States;

/// Atomic RW access to `States`, `RwLock<States>` newtype.
#[derive(Debug, Default)]
pub struct StatesRw(RwLock<States>);

impl StatesRw {
    /// Returns a new [`StatesRw`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the inner `States`.
    pub fn into_inner(self) -> States {
        self.0.into_inner()
    }
}

impl Deref for StatesRw {
    type Target = RwLock<States>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for StatesRw {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
