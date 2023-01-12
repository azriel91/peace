use async_trait::async_trait;
use peace_resources::states::{
    StateDiffs, StatesCleaned, StatesCleanedDry, StatesDesired, StatesEnsured, StatesEnsuredDry,
    StatesSaved,
};

cfg_if::cfg_if! {
    if #[cfg(feature = "output_progress")] {
        use peace_core::progress::{ProgressTracker, ProgressUpdateAndId};

        use crate::CmdProgressTracker;
    }
}

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
    /// Prepares this `OutputWrite` implementation for rendering progress.
    ///
    /// # Implementors
    ///
    /// This is called at the beginning of command execution, before any
    /// potential calls to `OutputWrite::progress_update`.
    ///
    /// At the end of command execution, `OutputWrite::progress_end` is called.
    #[cfg(feature = "output_progress")]
    async fn progress_begin(&mut self, cmd_progress_tracker: &CmdProgressTracker);

    /// Renders progress information, and returns when no more progress
    /// information is available to write.
    ///
    /// This function is infallible as progress information is considered
    /// transient, and loss of progress information is not considered as
    /// something worth stopping an operation.
    ///
    /// # Implementors
    ///
    /// This should render the progress update to the user in a way that is not
    /// overwhelming.
    #[cfg(feature = "output_progress")]
    async fn progress_update(
        &mut self,
        progress_tracker: &ProgressTracker,
        progress_update_and_id: &ProgressUpdateAndId,
    );

    /// Notifies this `OutputWrite` implementation to stop rendering progress.
    ///
    /// # Implementors
    ///
    /// This is called at the end of command execution. After this is called,
    /// there will be no more calls to `OutputWrite::progress_update` until
    /// another call to `OutputWrite::progress_begin`.
    #[cfg(feature = "output_progress")]
    async fn progress_end(&mut self, cmd_progress_tracker: &CmdProgressTracker);

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
