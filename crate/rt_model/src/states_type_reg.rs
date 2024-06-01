use std::ops::{Deref, DerefMut};

use peace_cfg::StepId;
use peace_resources::type_reg::untagged::{BoxDtDisplay, TypeReg};

/// Type registry for each step's `State`.
///
/// This is used to deserialize [`StatesCurrentFile`] and [`StatesGoalFile`].
///
/// Note: [`StepParamsTypeReg`] uses [`BoxDt`], whereas this uses
/// [`BoxDtDisplay`].
///
/// [`BoxDt`]: peace_resources::type_reg::untagged::BoxDt
/// [`BoxDtDisplay`]: peace_resources::type_reg::untagged::BoxDtDisplay
/// [`StepParamsTypeReg`]: crate::StepParamsTypeReg
/// [`Params`]: peace_cfg::Step::Params
/// [`StatesGoalFile`]: peace_resources::paths::StatesGoalFile
/// [`StatesCurrentFile`]: peace_resources::paths::StatesCurrentFile
#[derive(Debug, Default)]
pub struct StatesTypeReg(TypeReg<StepId, BoxDtDisplay>);

impl StatesTypeReg {
    /// Returns new `StatesTypeReg`.
    pub fn new() -> Self {
        Self::default()
    }
}

impl Deref for StatesTypeReg {
    type Target = TypeReg<StepId, BoxDtDisplay>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for StatesTypeReg {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
