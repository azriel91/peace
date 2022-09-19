use peace::{
    cfg::async_trait,
    resources::states::{
        StateDiffs, StatesCurrent, StatesDesired, StatesEnsured, StatesEnsuredDry,
    },
    rt_model::OutputWrite,
};

use crate::FnInvocation;

/// An `OutputWrite` implementation that tracks function invocations.
#[derive(Debug, Default, PartialEq, Eq)]
pub struct FnTrackerOutput {
    /// List of function invocations.
    fn_invocations: Vec<FnInvocation>,
}

impl FnTrackerOutput {
    /// Returns a new `FnTrackerOutput`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the recorded function invocations.
    pub fn fn_invocations(&self) -> &[FnInvocation] {
        self.fn_invocations.as_ref()
    }
}

#[async_trait(?Send)]
impl<E> OutputWrite<E> for FnTrackerOutput
where
    E: std::error::Error,
{
    async fn write_states_current(&mut self, states_current: &StatesCurrent) -> Result<(), E> {
        self.fn_invocations.push(FnInvocation::new(
            "write_states_current",
            vec![Some(format!("{states_current:?}"))],
        ));
        Ok(())
    }

    async fn write_states_desired(&mut self, states_desired: &StatesDesired) -> Result<(), E> {
        self.fn_invocations.push(FnInvocation::new(
            "write_states_desired",
            vec![Some(format!("{states_desired:?}"))],
        ));
        Ok(())
    }

    async fn write_state_diffs(&mut self, state_diffs: &StateDiffs) -> Result<(), E> {
        self.fn_invocations.push(FnInvocation::new(
            "write_state_diffs",
            vec![Some(format!("{state_diffs:?}"))],
        ));
        Ok(())
    }

    async fn write_states_ensured_dry(
        &mut self,
        states_ensured_dry: &StatesEnsuredDry,
    ) -> Result<(), E> {
        self.fn_invocations.push(FnInvocation::new(
            "write_states_ensured_dry",
            vec![Some(format!("{states_ensured_dry:?}"))],
        ));
        Ok(())
    }

    async fn write_states_ensured(&mut self, states_ensured: &StatesEnsured) -> Result<(), E> {
        self.fn_invocations.push(FnInvocation::new(
            "write_states_ensured",
            vec![Some(format!("{states_ensured:?}"))],
        ));
        Ok(())
    }

    async fn write_err(&mut self, error: &E) -> Result<(), E> {
        self.fn_invocations.push(FnInvocation::new(
            "write_err",
            vec![Some(format!("{error:?}"))],
        ));
        Ok(())
    }
}