use peace::{
    cfg::async_trait,
    resources::states::{
        StateDiffs, StatesCleaned, StatesCleanedDry, StatesDesired, StatesEnsured,
        StatesEnsuredDry, StatesSaved,
    },
    rt_model::output::OutputWrite,
};

cfg_if::cfg_if! {
    if #[cfg(feature = "output_progress")] {
        use peace::{
            cfg::progress::{ProgressTracker, ProgressUpdate},
            rt_model::CmdProgressTracker,
        };
    }
}

/// An `OutputWrite` implementation that does nothing.
#[derive(Debug)]
pub struct NoOpOutput;

#[async_trait(?Send)]
impl<E> OutputWrite<E> for NoOpOutput
where
    E: std::error::Error,
{
    #[cfg(feature = "output_progress")]
    async fn progress_begin(&mut self, _cmd_progress_tracker: &CmdProgressTracker) {}

    #[cfg(feature = "output_progress")]
    async fn progress_update(
        &mut self,
        _progress_tracker: &ProgressTracker,
        _progress_update: ProgressUpdate,
    ) {
    }

    #[cfg(feature = "output_progress")]
    async fn progress_end(&mut self, _cmd_progress_tracker: &CmdProgressTracker) {}

    async fn write_states_saved(&mut self, _states_saved: &StatesSaved) -> Result<(), E> {
        Ok(())
    }

    async fn write_states_desired(&mut self, _states_desired: &StatesDesired) -> Result<(), E> {
        Ok(())
    }

    async fn write_state_diffs(&mut self, _state_diffs: &StateDiffs) -> Result<(), E> {
        Ok(())
    }

    async fn write_states_ensured_dry(
        &mut self,
        _states_ensured_dry: &StatesEnsuredDry,
    ) -> Result<(), E> {
        Ok(())
    }

    async fn write_states_ensured(&mut self, _states_ensured: &StatesEnsured) -> Result<(), E> {
        Ok(())
    }

    async fn write_states_cleaned_dry(
        &mut self,
        _states_cleaned_dry: &StatesCleanedDry,
    ) -> Result<(), E> {
        Ok(())
    }

    async fn write_states_cleaned(&mut self, _states_cleaned: &StatesCleaned) -> Result<(), E> {
        Ok(())
    }

    async fn write_err(&mut self, _error: &E) -> Result<(), E> {
        Ok(())
    }
}
