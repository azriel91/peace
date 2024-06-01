use crate::outcomes::{StepApply, StepApplyRt};

/// A boxed `StepApply`.
#[derive(Clone, serde::Serialize)]
pub struct StepApplyBoxed(pub(crate) Box<dyn StepApplyRt>);

impl<State, StateDiff> From<StepApply<State, StateDiff>> for StepApplyBoxed
where
    StepApply<State, StateDiff>: StepApplyRt,
{
    /// Returns a `StepApplyBoxed` which erases a `StepApply`'s type
    /// parameters.
    fn from(step_apply: StepApply<State, StateDiff>) -> Self {
        Self(Box::new(step_apply))
    }
}

crate::outcomes::box_data_type_newtype!(StepApplyBoxed, StepApplyRt);
