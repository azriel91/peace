use peace_resources::states::{
    StateDiffs, StatesCurrent, StatesDesired, StatesEnsured, StatesEnsuredDry,
};
use peace_rt_model_core::{async_trait, OutputWrite};
use tokio::io::{AsyncWrite, AsyncWriteExt, Stdout};

use crate::Error;

/// An `OutputWrite` implementation that writes to the command line.
///
/// Currently this only outputs return values or errors, not progress.
#[derive(Debug)]
pub struct CliOutput<W> {
    /// Output stream to write to.
    writer: W,
}

impl CliOutput<Stdout> {
    /// Returns a new `CliOutput` using `io::stdout()` as the output stream.
    pub fn new() -> Self {
        Self::default()
    }
}

impl<W> CliOutput<W>
where
    W: AsyncWrite + std::marker::Unpin,
{
    /// Returns a new `CliOutput` using `io::stdout()` as the output stream.
    pub fn new_with_writer(writer: W) -> Self {
        Self { writer }
    }
}

impl Default for CliOutput<Stdout> {
    fn default() -> Self {
        Self {
            writer: tokio::io::stdout(),
        }
    }
}

/// Simple serialization implementations for now.
///
/// See <https://github.com/azriel91/peace/issues/28> for further improvements.
#[async_trait(?Send)]
impl<E, W> OutputWrite<E> for CliOutput<W>
where
    E: std::error::Error + From<Error>,
    W: AsyncWrite + std::marker::Unpin,
{
    async fn write_states_current(&mut self, states_current: &StatesCurrent) -> Result<(), E> {
        let states_current_serialized =
            serde_yaml::to_string(states_current).map_err(Error::StatesCurrentSerialize)?;

        self.writer
            .write_all(states_current_serialized.as_bytes())
            .await
            .map_err(Error::StdoutWrite)?;

        Ok(())
    }

    async fn write_states_desired(&mut self, states_desired: &StatesDesired) -> Result<(), E> {
        let states_desired_serialized =
            serde_yaml::to_string(states_desired).map_err(Error::StatesDesiredSerialize)?;

        self.writer
            .write_all(states_desired_serialized.as_bytes())
            .await
            .map_err(Error::StdoutWrite)?;

        Ok(())
    }

    async fn write_state_diffs(&mut self, state_diffs: &StateDiffs) -> Result<(), E> {
        let state_diffs_serialized =
            serde_yaml::to_string(state_diffs).map_err(Error::StateDiffsSerialize)?;

        self.writer
            .write_all(state_diffs_serialized.as_bytes())
            .await
            .map_err(Error::StdoutWrite)?;

        Ok(())
    }

    async fn write_states_ensured_dry(
        &mut self,
        states_ensured_dry: &StatesEnsuredDry,
    ) -> Result<(), E> {
        let states_ensured_dry_serialized =
            serde_yaml::to_string(states_ensured_dry).map_err(Error::StatesEnsuredDrySerialize)?;

        self.writer
            .write_all(states_ensured_dry_serialized.as_bytes())
            .await
            .map_err(Error::StdoutWrite)?;

        Ok(())
    }

    async fn write_states_ensured(&mut self, states_ensured: &StatesEnsured) -> Result<(), E> {
        let states_ensured_serialized =
            serde_yaml::to_string(states_ensured).map_err(Error::StatesEnsuredSerialize)?;

        self.writer
            .write_all(states_ensured_serialized.as_bytes())
            .await
            .map_err(Error::StdoutWrite)?;

        Ok(())
    }

    async fn write_err(&mut self, error: &E) -> Result<(), E> {
        self.writer
            .write_all(format!("{error}\n").as_bytes())
            .await
            .map_err(Error::StdoutWrite)?;

        Ok(())
    }
}
