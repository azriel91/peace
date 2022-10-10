use futures::{stream, StreamExt, TryStreamExt};
use peace_core::ItemSpecId;
use peace_resources::{
    states::{
        StateDiffs, StatesCleaned, StatesCleanedDry, StatesCurrent, StatesDesired, StatesEnsured,
        StatesEnsuredDry,
    },
    type_reg::untagged::BoxDtDisplay,
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

    async fn output_any_display<'f, E, I>(&mut self, iter: I) -> Result<(), E>
    where
        E: std::error::Error + From<Error>,
        I: Iterator<Item = (&'f ItemSpecId, &'f BoxDtDisplay)>,
    {
        let writer = &mut self.writer;
        stream::iter(iter)
            .map(Result::<_, std::io::Error>::Ok)
            .try_fold(
                writer,
                |writer, (item_spec_id, item_spec_state)| async move {
                    writer.write_all(item_spec_id.as_bytes()).await?;
                    writer.write_all(b": ").await?;
                    writer
                        .write_all(format!("{item_spec_state}\n").as_bytes())
                        .await?;
                    Ok(writer)
                },
            )
            .await
            .map_err(Error::StdoutWrite)?;
        Ok(())
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
        self.output_any_display(states_current.iter()).await
    }

    async fn write_states_desired(&mut self, states_desired: &StatesDesired) -> Result<(), E> {
        self.output_any_display(states_desired.iter()).await
    }

    async fn write_state_diffs(&mut self, state_diffs: &StateDiffs) -> Result<(), E> {
        self.output_any_display(state_diffs.iter()).await
    }

    async fn write_states_ensured_dry(
        &mut self,
        states_ensured_dry: &StatesEnsuredDry,
    ) -> Result<(), E> {
        self.output_any_display(states_ensured_dry.iter()).await
    }

    async fn write_states_ensured(&mut self, states_ensured: &StatesEnsured) -> Result<(), E> {
        self.output_any_display(states_ensured.iter()).await
    }

    async fn write_states_cleaned_dry(
        &mut self,
        states_cleaned_dry: &StatesCleanedDry,
    ) -> Result<(), E> {
        self.output_any_display(states_cleaned_dry.iter()).await
    }

    async fn write_states_cleaned(&mut self, states_cleaned: &StatesCleaned) -> Result<(), E> {
        self.output_any_display(states_cleaned.iter()).await
    }

    async fn write_err(&mut self, error: &E) -> Result<(), E> {
        self.writer
            .write_all(format!("{error}\n").as_bytes())
            .await
            .map_err(Error::StdoutWrite)?;

        Ok(())
    }
}
