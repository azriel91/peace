use crate::outcomes::{StepApplyPartial, StepApplyPartialRt};

/// A boxed `StepApplyPartial`.
#[derive(Clone, serde::Serialize)]
pub struct StepApplyPartialBoxed(pub(crate) Box<dyn StepApplyPartialRt>);

impl<State, StateDiff> From<StepApplyPartial<State, StateDiff>> for StepApplyPartialBoxed
where
    StepApplyPartial<State, StateDiff>: StepApplyPartialRt,
{
    /// Returns a `StepApplyPartialBoxed` which erases an
    /// `StepApplyPartial`'s type parameters.
    fn from(step_apply: StepApplyPartial<State, StateDiff>) -> Self {
        Self(Box::new(step_apply))
    }
}

crate::outcomes::box_data_type_newtype!(StepApplyPartialBoxed, StepApplyPartialRt);
