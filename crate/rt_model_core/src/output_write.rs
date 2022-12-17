use async_trait::async_trait;
use peace_resources::states::{
    StateDiffs, StatesCleaned, StatesCleanedDry, StatesDesired, StatesEnsured, StatesEnsuredDry,
    StatesPrevious,
};

/// Transforms return values or errors into a suitable output format.
///
/// Examples:
///
/// * A CLI implementation transforms the values into text to be printed.
/// * A REST implementation transforms the values into the response.
/// * A frontend implementation transforms the values into HTML elements.
///
/// # Design
///
/// The write functions currently take `&mut self`. From an API consumer
/// perspective, this should not be an annoyance as a return value / error value
/// is intended to be returned once per command.
///
/// Progress updates that are sent from `exec` functions would not be sent
/// through an `OutputWrite`, but possibly an `OutputProgressWrite`.
#[async_trait(?Send)]
pub trait OutputWrite<E> {
    /// Writes current states to the output.
    async fn write_states_previous(&mut self, states_previous: &StatesPrevious) -> Result<(), E>
    where
        E: std::error::Error;

    /// Writes desired states to the output.
    async fn write_states_desired(&mut self, states_desired: &StatesDesired) -> Result<(), E>
    where
        E: std::error::Error;

    /// Writes state diffs to the output.
    async fn write_state_diffs(&mut self, state_diffs: &StateDiffs) -> Result<(), E>
    where
        E: std::error::Error;

    /// Writes dry-ensured states to the output.
    ///
    /// These are the states that are simulated to be ensured.
    async fn write_states_ensured_dry(
        &mut self,
        states_ensured_dry: &StatesEnsuredDry,
    ) -> Result<(), E>
    where
        E: std::error::Error;

    /// Writes ensured states to the output.
    async fn write_states_ensured(&mut self, states_ensured: &StatesEnsured) -> Result<(), E>
    where
        E: std::error::Error;

    /// Writes dry-cleaned states to the output.
    ///
    /// These are the states that are simulated to be cleaned.
    async fn write_states_cleaned_dry(
        &mut self,
        states_cleaned_dry: &StatesCleanedDry,
    ) -> Result<(), E>
    where
        E: std::error::Error;

    /// Writes cleaned states to the output.
    async fn write_states_cleaned(&mut self, states_cleaned: &StatesCleaned) -> Result<(), E>
    where
        E: std::error::Error;

    /// Writes an error to the output.
    async fn write_err(&mut self, error: &E) -> Result<(), E>
    where
        E: std::error::Error;
}
