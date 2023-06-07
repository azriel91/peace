use std::ops::{Deref, DerefMut};

use peace_cfg::ItemId;
use peace_resources::type_reg::untagged::{BoxDtDisplay, TypeReg};

/// Type registry for each item's `State`.
///
/// This is used to deserialize [`StatesSavedFile`] and [`StatesGoalFile`].
///
/// Note: [`ItemParamsTypeReg`] uses [`BoxDt`], whereas this uses
/// [`BoxDtDisplay`].
///
/// [`BoxDt`]: peace_resources::type_reg::untagged::BoxDt
/// [`BoxDtDisplay`]: peace_resources::type_reg::untagged::BoxDtDisplay
/// [`ItemParamsTypeReg`]: crate::ItemParamsTypeReg
/// [`Params`]: peace_cfg::Item::Params
/// [`StatesGoalFile`]: peace_resources::paths::StatesGoalFile
/// [`StatesSavedFile`]: peace_resources::paths::StatesSavedFile
#[derive(Debug, Default)]
pub struct StatesTypeReg(TypeReg<ItemId, BoxDtDisplay>);

impl StatesTypeReg {
    /// Returns new `StatesTypeReg`.
    pub fn new() -> Self {
        Self::default()
    }
}

impl Deref for StatesTypeReg {
    type Target = TypeReg<ItemId, BoxDtDisplay>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for StatesTypeReg {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
