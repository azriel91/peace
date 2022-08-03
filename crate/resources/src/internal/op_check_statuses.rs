use std::{
    collections::HashMap,
    ops::{Deref, DerefMut},
};

use peace_core::{FullSpecId, OpCheckStatus};

/// [`OpCheckStatus`]es for all `FullSpec`s. `HashMap<FullSpecId,
/// OpCheckStatus>` newtype.
#[derive(Debug, Default)]
pub struct OpCheckStatuses(HashMap<FullSpecId, OpCheckStatus>);

impl OpCheckStatuses {
    /// Returns a new `OpCheckStatuses` map.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates an empty `OpCheckStatuses` map with the specified capacity.
    ///
    /// The `OpCheckStatuses` will be able to hold at least capacity elements
    /// without reallocating. If capacity is 0, the map will not allocate.
    pub fn with_capacity(capacity: usize) -> Self {
        Self(HashMap::with_capacity(capacity))
    }

    /// Returns the inner map.
    pub fn into_inner(self) -> HashMap<FullSpecId, OpCheckStatus> {
        self.0
    }
}

impl Deref for OpCheckStatuses {
    type Target = HashMap<FullSpecId, OpCheckStatus>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for OpCheckStatuses {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<HashMap<FullSpecId, OpCheckStatus>> for OpCheckStatuses {
    fn from(type_map: HashMap<FullSpecId, OpCheckStatus>) -> Self {
        Self(type_map)
    }
}

impl Extend<(FullSpecId, OpCheckStatus)> for OpCheckStatuses {
    fn extend<T: IntoIterator<Item = (FullSpecId, OpCheckStatus)>>(&mut self, iter: T) {
        iter.into_iter().for_each(|(full_spec_id, state)| {
            self.insert(full_spec_id, state);
        });
    }
}
