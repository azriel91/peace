use peace_resources::states::{
    StateDiffs, StatesCleaned, StatesCleanedDry, StatesDesired, StatesEnsured, StatesEnsuredDry,
    StatesSaved,
};
use peace_rt_model_core::{async_trait, OutputWrite};

use crate::Error;

/// An `OutputWrite` implementation that writes to the command line.
///
/// Currently this only outputs return values or errors, not progress.
#[derive(Debug, Default)]
pub struct InMemoryTextOutput {
    /// Buffer to write to.
    buffer: String,
}

impl InMemoryTextOutput {
    /// Returns a new `InMemoryTextOutput`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the inner buffer.
    pub fn into_inner(self) -> String {
        self.buffer
    }
}

/// Simple serialization implementations for now.
///
/// See <https://github.com/azriel91/peace/issues/28> for further improvements.
#[async_trait(?Send)]
impl<E> OutputWrite<E> for InMemoryTextOutput
where
    E: std::error::Error + From<Error>,
{
    #[cfg(feature = "output_progress")]
    async fn progress_begin(&mut self, _cmd_progress_tracker: &crate::CmdProgressTracker) {}

    #[cfg(feature = "output_progress")]
    async fn progress_update(&mut self, _progress_update: peace_cfg::ProgressUpdate) {}

    #[cfg(feature = "output_progress")]
    async fn progress_end(&mut self, _cmd_progress_tracker: &crate::CmdProgressTracker) {}

    async fn write_states_saved(&mut self, states_saved: &StatesSaved) -> Result<(), E> {
        self.buffer = serde_yaml::to_string(states_saved).map_err(Error::StatesSerialize)?;

        Ok(())
    }

    async fn write_states_desired(&mut self, states_desired: &StatesDesired) -> Result<(), E> {
        self.buffer = serde_yaml::to_string(states_desired).map_err(Error::StatesSerialize)?;

        Ok(())
    }

    async fn write_state_diffs(&mut self, state_diffs: &StateDiffs) -> Result<(), E> {
        self.buffer = serde_yaml::to_string(state_diffs).map_err(Error::StateDiffsSerialize)?;

        Ok(())
    }

    async fn write_states_ensured_dry(
        &mut self,
        states_ensured_dry: &StatesEnsuredDry,
    ) -> Result<(), E> {
        self.buffer = serde_yaml::to_string(states_ensured_dry).map_err(Error::StatesSerialize)?;

        Ok(())
    }

    async fn write_states_ensured(&mut self, states_ensured: &StatesEnsured) -> Result<(), E> {
        self.buffer = serde_yaml::to_string(states_ensured).map_err(Error::StatesSerialize)?;

        Ok(())
    }

    async fn write_states_cleaned_dry(
        &mut self,
        states_cleaned_dry: &StatesCleanedDry,
    ) -> Result<(), E> {
        self.buffer = serde_yaml::to_string(states_cleaned_dry).map_err(Error::StatesSerialize)?;

        Ok(())
    }

    async fn write_states_cleaned(&mut self, states_cleaned: &StatesCleaned) -> Result<(), E> {
        self.buffer = serde_yaml::to_string(states_cleaned).map_err(Error::StatesSerialize)?;

        Ok(())
    }

    async fn write_err(&mut self, error: &E) -> Result<(), E> {
        self.buffer = format!("{error}\n");

        Ok(())
    }
}
