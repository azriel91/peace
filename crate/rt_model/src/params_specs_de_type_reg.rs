use std::ops::{Deref, DerefMut};

use peace_cfg::ItemSpecId;
use peace_resources::type_reg::untagged::{BoxDt, TypeReg};

/// Type registry for each item spec's [`Params`]'s SpecDe.
///
/// This is used to deserialize [`ParamsSpecsFile`].
///
/// Note: [`StatesTypeReg`] uses [`BoxDtDisplay`], whereas this uses [`BoxDt`].
///
/// [`BoxDt`]: peace_resources::type_reg::untagged::BoxDt
/// [`BoxDtDisplay`]: peace_resources::type_reg::untagged::BoxDtDisplay
/// [`Params`]: peace_cfg::ItemSpec::Params
/// [`StatesTypeReg`]: crate::StatesTypeReg
#[derive(Debug, Default)]
pub struct ParamsSpecsDeTypeReg(TypeReg<ItemSpecId, BoxDt>);

impl ParamsSpecsDeTypeReg {
    /// Returns a new `ParamsSpecsTypeReg`.
    pub fn new() -> Self {
        Self::default()
    }
}

impl Deref for ParamsSpecsDeTypeReg {
    type Target = TypeReg<ItemSpecId, BoxDt>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ParamsSpecsDeTypeReg {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
