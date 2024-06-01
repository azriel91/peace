use std::ops::{Deref, DerefMut};

use peace_data::fn_graph::FnId;

/// Runtime identifier for a [`Step`]. [`FnId`] newtype.
///
/// This is a cheap identifier to copy around, instead of cloning
/// [`StepId`].
///
/// [`StepId`]: peace_cfg::StepId
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StepRtId(FnId);

impl StepRtId {
    /// Returns a new `StepRtId`.
    pub fn new(fn_id: FnId) -> Self {
        Self(fn_id)
    }

    /// Returns the inner [`FnId`].
    pub fn into_inner(self) -> FnId {
        self.0
    }
}

impl Deref for StepRtId {
    type Target = FnId;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for StepRtId {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<usize> for StepRtId {
    fn from(index: usize) -> Self {
        Self(FnId::new(index))
    }
}

impl From<FnId> for StepRtId {
    fn from(fn_id: FnId) -> Self {
        Self(fn_id)
    }
}
