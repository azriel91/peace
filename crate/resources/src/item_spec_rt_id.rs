use std::ops::{Deref, DerefMut};

use peace_data::fn_graph::FnId;

/// Runtime identifier for an [`ItemSpec`]. [`FnId`] newtype.
///
/// This is a cheap identifier to copy around, instead of cloning
/// [`ItemSpecId`].
///
/// [`ItemSpecId`]: peace_cfg::ItemSpecId
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ItemSpecRtId(FnId);

impl ItemSpecRtId {
    /// Returns a new `ItemSpecRtId`.
    pub fn new(fn_id: FnId) -> Self {
        Self(fn_id)
    }

    /// Returns the inner [`FnId`].
    pub fn into_inner(self) -> FnId {
        self.0
    }
}

impl Deref for ItemSpecRtId {
    type Target = FnId;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ItemSpecRtId {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<usize> for ItemSpecRtId {
    fn from(index: usize) -> Self {
        Self(FnId::new(index))
    }
}

impl From<FnId> for ItemSpecRtId {
    fn from(fn_id: FnId) -> Self {
        Self(fn_id)
    }
}
