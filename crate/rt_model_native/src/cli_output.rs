use futures::{stream, StreamExt, TryStreamExt};
use peace_core::ItemSpecId;
use peace_resources::{
    states::{
        StateDiffs, StatesCleaned, StatesCleanedDry, StatesDesired, StatesEnsured,
        StatesEnsuredDry, StatesSaved,
    },
    type_reg::untagged::BoxDtDisplay,
};
use peace_rt_model_core::{async_trait, OutputFormat, OutputWrite};
use serde::Serialize;
use tokio::io::{AsyncWrite, AsyncWriteExt, Stdout};

use crate::Error;

/// An `OutputWrite` implementation that writes to the command line.
///
/// Currently this only outputs return values or errors, not progress.
#[derive(Debug)]
pub struct CliOutput<W> {
    /// Output stream to write to.
    writer: W,
    /// How to format command output -- human readable or machine parsable.
    format: OutputFormat,
    /// Whether output should be colorized.
    #[cfg(feature = "output_colorized")]
    colorized: bool,
}

impl CliOutput<Stdout> {
    /// Returns a new `CliOutput` using `io::stdout()` as the output stream.
    ///
    /// The default output is not colorized.
    pub fn new() -> Self {
        Self::default()
    }
}

impl<W> CliOutput<W>
where
    W: AsyncWrite + std::marker::Unpin,
{
    /// Returns a new `CliOutput` using `io::stdout()` as the output stream.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use peace_rt_model_native::CliOutput;
    /// // use peace::rt_model::CliOutput;
    ///
    /// let mut buffer = Vec::<u8>::new();
    /// let cli_output = CliOutput::new_with_writer(&mut buffer);
    /// ```
    pub fn new_with_writer(writer: W) -> Self {
        Self {
            writer,
            format: OutputFormat::Text,
            #[cfg(feature = "output_colorized")]
            colorized: false,
        }
    }

    /// Sets the output format for this `CliOutput`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use peace_rt_model_core::OutputFormat;
    /// # use peace_rt_model_native::CliOutput;
    /// // use peace::rt_model::{CliOutput, OutputFormat};
    ///
    /// let cli_output = CliOutput::new().output_format(OutputFormat::Yaml);
    /// ```
    pub fn output_format(mut self, output_format: OutputFormat) -> Self {
        self.format = output_format;
        self
    }

    /// Enables colorized output.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use peace_rt_model_native::CliOutput;
    /// // use peace::rt_model::CliOutput;
    ///
    /// let cli_output = CliOutput::new().colorized();
    /// ```
    #[cfg(feature = "output_colorized")]
    pub fn colorized(mut self) -> Self {
        self.colorized = true;
        self
    }

    #[cfg(not(feature = "output_colorized"))]
    async fn output_display<'f, E, I>(&mut self, iter: I) -> Result<(), E>
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

    #[cfg(feature = "output_colorized")]
    async fn output_display<'f, E, I>(&mut self, iter: I) -> Result<(), E>
    where
        E: std::error::Error + From<Error>,
        I: Iterator<Item = (&'f ItemSpecId, &'f BoxDtDisplay)>,
    {
        let item_spec_id_style = &console::Style::new().color256(69);
        let colorized = self.colorized;

        let writer = &mut self.writer;
        stream::iter(iter)
            .map(Result::<_, std::io::Error>::Ok)
            .try_fold(
                writer,
                |writer, (item_spec_id, item_spec_state)| async move {
                    if colorized {
                        let item_spec_id_colorized = item_spec_id_style.apply_to(item_spec_id);
                        writer
                            .write_all(format!("{item_spec_id_colorized}").as_bytes())
                            .await?;
                    } else {
                        writer.write_all(item_spec_id.as_bytes()).await?;
                    }

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

    async fn output_yaml<'f, E, T, F>(&mut self, t: &T, fn_error: F) -> Result<(), E>
    where
        E: std::error::Error + From<Error>,
        T: Serialize,
        F: FnOnce(serde_yaml::Error) -> Error,
    {
        let t_serialized = serde_yaml::to_string(t).map_err(fn_error)?;

        self.writer
            .write_all(t_serialized.as_bytes())
            .await
            .map_err(Error::StdoutWrite)?;

        Ok(())
    }

    #[cfg(feature = "output_json")]
    async fn output_json<'f, E, T, F>(&mut self, t: &T, fn_error: F) -> Result<(), E>
    where
        E: std::error::Error + From<Error>,
        T: Serialize,
        F: FnOnce(serde_json::Error) -> Error,
    {
        let t_serialized = serde_json::to_string(t).map_err(fn_error)?;

        self.writer
            .write_all(t_serialized.as_bytes())
            .await
            .map_err(Error::StdoutWrite)?;

        Ok(())
    }
}

impl Default for CliOutput<Stdout> {
    fn default() -> Self {
        Self {
            writer: tokio::io::stdout(),
            format: OutputFormat::Text,
            #[cfg(feature = "output_colorized")]
            colorized: false,
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
    #[cfg(feature = "output_progress")]
    async fn render(&mut self, progress_update: peace_core::ProgressUpdate) {
        let _ = self
            .writer
            .write(format!("{progress_update:?}").as_bytes())
            .await;

        let _ = self.writer.flush().await;
    }

    async fn write_states_saved(&mut self, states_saved: &StatesSaved) -> Result<(), E> {
        match self.format {
            OutputFormat::Text => self.output_display(states_saved.iter()).await,
            OutputFormat::Yaml => self.output_yaml(states_saved, Error::StatesSerialize).await,
            #[cfg(feature = "output_json")]
            OutputFormat::Json => {
                self.output_json(states_saved, Error::StatesSerializeJson)
                    .await
            }
        }
    }

    async fn write_states_desired(&mut self, states_desired: &StatesDesired) -> Result<(), E> {
        match self.format {
            OutputFormat::Text => self.output_display(states_desired.iter()).await,
            OutputFormat::Yaml => {
                self.output_yaml(states_desired, Error::StatesSerialize)
                    .await
            }
            #[cfg(feature = "output_json")]
            OutputFormat::Json => {
                self.output_json(states_desired, Error::StatesSerializeJson)
                    .await
            }
        }
    }

    async fn write_state_diffs(&mut self, state_diffs: &StateDiffs) -> Result<(), E> {
        match self.format {
            OutputFormat::Text => self.output_display(state_diffs.iter()).await,
            OutputFormat::Yaml => {
                self.output_yaml(state_diffs, Error::StateDiffsSerialize)
                    .await
            }
            #[cfg(feature = "output_json")]
            OutputFormat::Json => {
                self.output_json(state_diffs, Error::StateDiffsSerializeJson)
                    .await
            }
        }
    }

    async fn write_states_ensured_dry(
        &mut self,
        states_ensured_dry: &StatesEnsuredDry,
    ) -> Result<(), E> {
        match self.format {
            OutputFormat::Text => self.output_display(states_ensured_dry.iter()).await,
            OutputFormat::Yaml => {
                self.output_yaml(states_ensured_dry, Error::StatesSerialize)
                    .await
            }
            #[cfg(feature = "output_json")]
            OutputFormat::Json => {
                self.output_json(states_ensured_dry, Error::StatesSerializeJson)
                    .await
            }
        }
    }

    async fn write_states_ensured(&mut self, states_ensured: &StatesEnsured) -> Result<(), E> {
        match self.format {
            OutputFormat::Text => self.output_display(states_ensured.iter()).await,
            OutputFormat::Yaml => {
                self.output_yaml(states_ensured, Error::StatesSerialize)
                    .await
            }
            #[cfg(feature = "output_json")]
            OutputFormat::Json => {
                self.output_json(states_ensured, Error::StatesSerializeJson)
                    .await
            }
        }
    }

    async fn write_states_cleaned_dry(
        &mut self,
        states_cleaned_dry: &StatesCleanedDry,
    ) -> Result<(), E> {
        match self.format {
            OutputFormat::Text => self.output_display(states_cleaned_dry.iter()).await,
            OutputFormat::Yaml => {
                self.output_yaml(states_cleaned_dry, Error::StatesSerialize)
                    .await
            }
            #[cfg(feature = "output_json")]
            OutputFormat::Json => {
                self.output_json(states_cleaned_dry, Error::StatesSerializeJson)
                    .await
            }
        }
    }

    async fn write_states_cleaned(&mut self, states_cleaned: &StatesCleaned) -> Result<(), E> {
        match self.format {
            OutputFormat::Text => self.output_display(states_cleaned.iter()).await,
            OutputFormat::Yaml => {
                self.output_yaml(states_cleaned, Error::StatesSerialize)
                    .await
            }
            #[cfg(feature = "output_json")]
            OutputFormat::Json => {
                self.output_json(states_cleaned, Error::StatesSerializeJson)
                    .await
            }
        }
    }

    async fn write_err(&mut self, error: &E) -> Result<(), E> {
        match self.format {
            OutputFormat::Text => {
                self.writer
                    .write_all(format!("{error}\n").as_bytes())
                    .await
                    .map_err(Error::StdoutWrite)?;
            }
            OutputFormat::Yaml => {
                // TODO: proper parsable structure with error code.
                let error_serialized =
                    serde_yaml::to_string(&format!("{error}")).map_err(Error::ErrorSerialize)?;
                self.writer
                    .write_all(error_serialized.as_bytes())
                    .await
                    .map_err(Error::StdoutWrite)?;
            }
            #[cfg(feature = "output_json")]
            OutputFormat::Json => {
                // TODO: proper parsable structure with error code.
                let error_serialized = serde_json::to_string(&format!("{error}"))
                    .map_err(Error::ErrorSerializeJson)?;
                self.writer
                    .write_all(error_serialized.as_bytes())
                    .await
                    .map_err(Error::StdoutWrite)?;
            }
        }

        Ok(())
    }
}
