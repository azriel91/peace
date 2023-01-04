use async_trait::async_trait;
use peace_resources::states::{
    StateDiffs, StatesCleaned, StatesCleanedDry, StatesDesired, StatesEnsured, StatesEnsuredDry,
    StatesSaved,
};

/// Transforms return values or errors into a suitable output format.
///
/// # Use cases
///
/// * A CLI implementation transforms the values into text to be printed.
/// * A REST implementation transforms the values into the response.
/// * A frontend implementation transforms the values into HTML elements.
///
/// # Design
///
/// The write functions currently take `&mut self`. From an API implementor
/// perspective, this should not be difficult to use as the return value / error
/// value is intended to be returned at the end of a command.
///
/// Progress updates sent during `EnsureOpSpec::exec` and `CleanOpSpec::exec`.
#[async_trait(?Send)]
pub trait OutputWrite<E> {
    /// Renders progress information, and returns when no more progress
    /// information is available to write.
    ///
    /// This function is infallible as progress information is considered
    /// transient, and loss of progress information is not considered as
    /// something worth stopping an operation.
    ///
    /// # Implementors
    ///
    /// This should create a new channel, and return the channel sender.
    ///
    /// The sender will be passed to each of the `EnsureOpSpec::exec` functions
    /// so that progress information can be sent within them.
    #[cfg(feature = "output_progress")]
    async fn progress_update(&mut self, progress_update: peace_core::ProgressUpdate);

    /// Writes current states to the output.
    async fn write_states_saved(&mut self, states_saved: &StatesSaved) -> Result<(), E>
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
