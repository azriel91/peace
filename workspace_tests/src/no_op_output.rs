use peace::{
    cfg::async_trait,
    resources::states::{
        StateDiffs, StatesCleaned, StatesCleanedDry, StatesDesired, StatesEnsured,
        StatesEnsuredDry, StatesSaved,
    },
    rt_model::OutputWrite,
};

/// An `OutputWrite` implementation that does nothing.
#[derive(Debug)]
pub struct NoOpOutput;

#[async_trait(?Send)]
impl<E> OutputWrite<E> for NoOpOutput
where
    E: std::error::Error,
{
    #[cfg(feature = "output_progress")]
    async fn render(&mut self, _progress_update: peace::cfg::ProgressUpdate) {}

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
