use std::ops::{Deref, DerefMut};

use peace_cfg::StepId;
use peace_params::AnySpecRtBoxed;
use peace_resources::type_reg::untagged::TypeReg;

/// Type registry for each step's [`Params`]'s Spec.
///
/// This is used to deserialize [`ParamsSpecsFile`].
///
/// Note: [`StatesTypeReg`] uses [`BoxDtDisplay`], whereas this uses
/// [`AnySpecRtBoxed`].
///
/// [`AnySpecRtBoxed`]: peace_params::AnySpecRtBoxed
/// [`BoxDtDisplay`]: peace_resources::type_reg::untagged::BoxDtDisplay
/// [`Params`]: peace_cfg::Step::Params
/// [`StatesTypeReg`]: crate::StatesTypeReg
#[derive(Debug, Default)]
pub struct ParamsSpecsTypeReg(TypeReg<StepId, AnySpecRtBoxed>);

impl ParamsSpecsTypeReg {
    /// Returns a new `ParamsSpecsTypeReg`.
    pub fn new() -> Self {
        Self::default()
    }
}

impl Deref for ParamsSpecsTypeReg {
    type Target = TypeReg<StepId, AnySpecRtBoxed>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ParamsSpecsTypeReg {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
