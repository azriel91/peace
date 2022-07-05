use std::ops::{Deref, DerefMut};

use fn_graph::FnId;

/// Runtime identifier for a [`FullSpec`]. [`FnId`] newtype.
///
/// This is a cheap identifier to copy around, instead of cloning
/// [`FullSpecId`].
///
/// [`FullSpecId`]: peace_cfg::FullSpecId
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FullSpecRtId(FnId);

impl FullSpecRtId {
    /// Returns a new `FullSpecRtId`.
    pub fn new(fn_id: FnId) -> Self {
        Self(fn_id)
    }

    /// Returns the inner [`FnId`].
    pub fn into_inner(self) -> FnId {
        self.0
    }
}

impl Deref for FullSpecRtId {
    type Target = FnId;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for FullSpecRtId {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<usize> for FullSpecRtId {
    fn from(index: usize) -> Self {
        Self(FnId::new(index))
    }
}

impl From<FnId> for FullSpecRtId {
    fn from(fn_id: FnId) -> Self {
        Self(fn_id)
    }
}
