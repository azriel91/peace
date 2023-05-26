use std::ops::{Deref, DerefMut};

use peace_cfg::ItemId;
use peace_resources::type_reg::untagged::{BoxDt, TypeReg};

/// Type registry for each item's [`Params`].
///
/// This is used to deserialize [`ItemParamsFile`].
///
/// Note: [`StatesTypeReg`] uses [`BoxDtDisplay`], whereas this uses [`BoxDt`].
///
/// [`BoxDt`]: peace_resources::type_reg::untagged::BoxDt
/// [`BoxDtDisplay`]: peace_resources::type_reg::untagged::BoxDtDisplay
/// [`Params`]: peace_cfg::Item::Params
/// [`StatesTypeReg`]: crate::StatesTypeReg
#[derive(Debug, Default)]
pub struct ItemParamsTypeReg(TypeReg<ItemId, BoxDt>);

impl ItemParamsTypeReg {
    /// Returns new `ItemParamsTypeReg`.
    pub fn new() -> Self {
        Self::default()
    }
}

impl Deref for ItemParamsTypeReg {
    type Target = TypeReg<ItemId, BoxDt>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ItemParamsTypeReg {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
