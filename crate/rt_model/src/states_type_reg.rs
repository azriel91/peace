use peace_cfg::ItemSpecId;
use peace_resources::type_reg::untagged::{BoxDtDisplay, TypeReg};
use std::ops::{Deref, DerefMut};

/// Type registries to deserialize `StatesSavedFile` and `StatesDesiredFile`.
#[derive(Debug, Default)]
pub struct StatesTypeReg(
    /// Type registry for each item spec's `State`.
    TypeReg<ItemSpecId, BoxDtDisplay>,
);

impl StatesTypeReg {
    /// Returns new `StatesTypeReg`.
    pub fn new() -> Self {
        Self::default()
    }
}

impl Deref for StatesTypeReg {
    type Target = TypeReg<ItemSpecId, BoxDtDisplay>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for StatesTypeReg {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
