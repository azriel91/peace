use fn_graph::StreamOutcomeState;
use peace_cfg::StepId;

/// How a `Flow` stream operation ended and IDs that were processed.
///
/// Currently this is constructed by `StepStreamOutcomeMapper`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StepStreamOutcome<T> {
    /// The value of the outcome.
    pub value: T,
    /// How a `Flow` stream operation ended.
    pub state: StreamOutcomeState,
    /// IDs of the steps that were processed.
    pub step_ids_processed: Vec<StepId>,
    /// IDs of the steps that were not processed.
    pub step_ids_not_processed: Vec<StepId>,
}

impl<T> StepStreamOutcome<T> {
    /// Returns a `StepStreamOutcome` that is `Finished<T>`.
    pub fn finished_with(value: T, step_ids_processed: Vec<StepId>) -> Self {
        Self {
            value,
            state: StreamOutcomeState::Finished,
            step_ids_processed,
            step_ids_not_processed: Vec::new(),
        }
    }

    /// Maps this outcome's value to another.
    pub fn map<TNew>(self, f: impl FnOnce(T) -> TNew) -> StepStreamOutcome<TNew> {
        let StepStreamOutcome {
            value,
            state,
            step_ids_processed,
            step_ids_not_processed,
        } = self;

        let value = f(value);

        StepStreamOutcome {
            value,
            state,
            step_ids_processed,
            step_ids_not_processed,
        }
    }

    /// Replaces the value from this outcome with another.
    pub fn replace<TNew>(self, value_new: TNew) -> (StepStreamOutcome<TNew>, T) {
        let StepStreamOutcome {
            value: value_existing,
            state,
            step_ids_processed,
            step_ids_not_processed,
        } = self;

        (
            StepStreamOutcome {
                value: value_new,
                state,
                step_ids_processed,
                step_ids_not_processed,
            },
            value_existing,
        )
    }

    /// Replaces the value from this outcome with another, taking the current
    /// value as a parameter.
    pub fn replace_with<TNew, U>(
        self,
        f: impl FnOnce(T) -> (TNew, U),
    ) -> (StepStreamOutcome<TNew>, U) {
        let StepStreamOutcome {
            value,
            state,
            step_ids_processed,
            step_ids_not_processed,
        } = self;

        let (value, extracted) = f(value);

        (
            StepStreamOutcome {
                value,
                state,
                step_ids_processed,
                step_ids_not_processed,
            },
            extracted,
        )
    }

    pub fn into_value(self) -> T {
        self.value
    }

    pub fn value(&self) -> &T {
        &self.value
    }

    pub fn value_mut(&mut self) -> &mut T {
        &mut self.value
    }

    pub fn state(&self) -> StreamOutcomeState {
        self.state
    }

    pub fn step_ids_processed(&self) -> &[StepId] {
        self.step_ids_processed.as_ref()
    }

    pub fn step_ids_not_processed(&self) -> &[StepId] {
        self.step_ids_not_processed.as_ref()
    }
}

impl<T> Default for StepStreamOutcome<T>
where
    T: Default,
{
    fn default() -> Self {
        Self {
            value: T::default(),
            state: StreamOutcomeState::NotStarted,
            step_ids_processed: Vec::new(),
            step_ids_not_processed: Vec::new(),
        }
    }
}
