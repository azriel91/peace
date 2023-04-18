use std::ops::{Deref, DerefMut};

use peace_cfg::ItemSpecId;
use peace_resources::type_reg::untagged::{BoxDtDisplay, TypeReg};

/// Type registry for each item spec's `State`.
///
/// This is used to deserialize [`StatesSavedFile`] and [`StatesDesiredFile`].
///
/// Note: [`ItemSpecParamsTypeReg`] uses [`BoxDt`], whereas this uses
/// [`BoxDtDisplay`].
///
/// [`BoxDt`]: peace_resources::type_reg::untagged::BoxDt
/// [`BoxDtDisplay`]: peace_resources::type_reg::untagged::BoxDtDisplay
/// [`ItemSpecParamsTypeReg`]: crate::ItemSpecParamsTypeReg
/// [`Params`]: peace_cfg::ItemSpec::Params
/// [`StatesDesiredFile`]: peace_resources::paths::StatesDesiredFile
/// [`StatesSavedFile`]: peace_resources::paths::StatesSavedFile
#[derive(Debug, Default)]
pub struct StatesTypeReg(TypeReg<ItemSpecId, BoxDtDisplay>);

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
