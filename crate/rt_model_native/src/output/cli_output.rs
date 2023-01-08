use futures::{stream, StreamExt, TryStreamExt};

use peace_core::ItemSpecId;
use peace_resources::{
    states::{
        StateDiffs, StatesCleaned, StatesCleanedDry, StatesDesired, StatesEnsured,
        StatesEnsuredDry, StatesSaved,
    },
    type_reg::untagged::BoxDtDisplay,
};
use peace_rt_model_core::{
    async_trait,
    output::{OutputFormat, OutputWrite},
};
use serde::Serialize;
use tokio::io::{AsyncWrite, AsyncWriteExt, Stdout};

use crate::Error;

#[cfg(feature = "output_colorized")]
use crate::output::{CliColorize, CliColorizeChosen};

cfg_if::cfg_if! {
    if #[cfg(feature = "output_progress")] {
        use peace_core::progress::ProgressUpdate;
        use peace_rt_model_core::{
            indicatif::{ProgressStyle, ProgressDrawTarget},
            CmdProgressTracker,
        };

        use crate::output::{CliOutputTarget, CliProgressFormat, CliProgressFormatChosen};
    }
}

#[cfg(any(feature = "output_colorized", feature = "output_progress"))]
use is_terminal::IsTerminal;

/// An `OutputWrite` implementation that writes to the command line.
///
/// # Features
///
/// ## `"output_colorized"`
///
/// When this feature is enabled, text output is coloured with ANSI codes when
/// the outcome output stream is a terminal (i.e. not piped to another process,
/// or redirected to a file).
///
/// If it is piped to another process or redirected to a file, then the outcome
/// output is not colourized.
///
/// This automatic detection can be overridden by calling the [`with_colorized`]
/// method.
///
/// ## `"output_progress"`
///
/// When this feature is enabled, progress is written to `stderr` by default.
///
/// By default, when the progress stream is a terminal (i.e. not piped to
/// another process, or redirected to a file), then the progress output format
/// is a progress bar.
///
/// If it is piped to another process or redirected to a file, then the progress
/// output format defaults to the same format at the outcome output format --
/// text, YAML, or JSON.
///
/// These defaults may be overridden through the [`with_progress_target`] and
/// [`with_progress_format`] methods.
///
/// # Implementation Note
///
/// `indicatif`'s internal writing to `stdout` / `stderr` is used, which is
/// sync. I didn't figure out how to write the in-memory term contents to the
/// `W` writer correctly.
///
/// [`with_colorized`]: Self::with_colorized
/// [`with_progress_format`]: Self::with_progress_format
/// [`with_progress_target`]: Self::with_progress_target
#[derive(Debug)]
pub struct CliOutput<W> {
    /// Output stream to write the command outcome to.
    writer: W,
    /// How to format command outcome output -- human readable or machine
    /// parsable.
    format: OutputFormat,
    /// Whether output should be colorized.
    #[cfg(feature = "output_colorized")]
    colorized: CliColorizeChosen,
    #[cfg(feature = "output_progress")]
    /// Where to output progress updates to -- stdout or stderr.
    progress_target: CliOutputTarget,
    /// Whether the writer is an interactive terminal.
    ///
    /// This is detected on instantiation.
    #[cfg(feature = "output_progress")]
    progress_format: CliProgressFormatChosen,
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
            colorized: CliColorizeChosen::Colored, // TODO: builder
            #[cfg(feature = "output_progress")]
            progress_target: CliOutputTarget::default(),
            #[cfg(feature = "output_progress")]
            progress_format: CliProgressFormatChosen::Output,
        }
    }

    /// Sets the progress output target -- stdout or stderr (default).
    #[cfg(feature = "output_progress")]
    pub fn with_progress_target(mut self, progress_target: CliOutputTarget) -> Self {
        self.progress_target = progress_target;
        self
    }

    /// Sets the progress output format.
    #[cfg(feature = "output_progress")]
    pub fn with_progress_format(mut self, progress_format: CliProgressFormat) -> Self {
        let progress_format_chosen = match progress_format {
            CliProgressFormat::Auto => {
                // Even though we're using `tokio::io::stdout` / `stderr`, `IsTerminal` is only
                // implemented on `std::io::stdout` / `stderr`.
                match self.progress_target {
                    CliOutputTarget::Stdout => {
                        if std::io::stdout().is_terminal() {
                            CliProgressFormatChosen::ProgressBar
                        } else {
                            CliProgressFormatChosen::Output
                        }
                    }
                    CliOutputTarget::Stderr => {
                        if std::io::stderr().is_terminal() {
                            CliProgressFormatChosen::ProgressBar
                        } else {
                            CliProgressFormatChosen::Output
                        }
                    }
                }
            }
            CliProgressFormat::Output => CliProgressFormatChosen::Output,
            CliProgressFormat::ProgressBar => CliProgressFormatChosen::ProgressBar,
        };

        self.progress_format = progress_format_chosen;
        self
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
    /// let cli_output = CliOutput::new().with_output_format(OutputFormat::Yaml);
    /// ```
    pub fn with_output_format(mut self, output_format: OutputFormat) -> Self {
        self.format = output_format;
        self
    }

    /// Enables colorized output.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use peace_rt_model_native::CliOutput;
    /// // use peace::rt_model::{CliColorize, CliOutput};
    ///
    /// # #[cfg(feature = "output_colorized")]
    /// let cli_output = CliOutput::new().with_colorized(CliColorize::Auto);
    /// ```
    #[cfg(feature = "output_colorized")]
    pub fn with_colorize(mut self, colorize: CliColorize) -> Self {
        self.colorized = match colorize {
            CliColorize::Auto => {
                // Even though we're using `tokio::io::stdout` / `stderr`, `IsTerminal` is only
                // implemented on `std::io::stdout` / `stderr`.
                if std::io::stdout().is_terminal() {
                    CliColorizeChosen::Colored
                } else {
                    CliColorizeChosen::Uncolored
                }
            }
            CliColorize::Always => CliColorizeChosen::Colored,
            CliColorize::Never => CliColorizeChosen::Uncolored,
        };
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
                    if colorized == CliColorizeChosen::Colored {
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
        let stdout = tokio::io::stdout();

        // Even though we're using `tokio::io::stdout`, `IsTerminal` is only implemented
        // on `std::io::stdout`.
        #[cfg(feature = "output_progress")]
        let progress_format = if std::io::stdout().is_terminal() {
            CliProgressFormatChosen::ProgressBar
        } else {
            CliProgressFormatChosen::Output
        };

        Self {
            writer: stdout,
            format: OutputFormat::Text,
            #[cfg(feature = "output_colorized")]
            colorized: CliColorizeChosen::Colored, // TODO: builder
            #[cfg(feature = "output_progress")]
            progress_target: CliOutputTarget::default(),
            #[cfg(feature = "output_progress")]
            progress_format,
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
    async fn progress_begin(&mut self, cmd_progress_tracker: &CmdProgressTracker) {
        if self.progress_format == CliProgressFormatChosen::ProgressBar {
            let progress_draw_target = match self.progress_target {
                CliOutputTarget::Stdout => ProgressDrawTarget::stdout(),
                CliOutputTarget::Stderr => ProgressDrawTarget::stderr(),
            };

            cmd_progress_tracker
                .multi_progress()
                .set_draw_target(progress_draw_target);

            cmd_progress_tracker.progress_trackers().iter().for_each(
                |(item_spec_id, progress_tracker)| {
                    let progress_bar = progress_tracker.progress_bar();
                    progress_bar.set_prefix(format!("{item_spec_id}"));
                    let template = format!("{{prefix:20}} ▕{pb}▏", pb = "{bar:40.green.on_17}");
                    progress_bar.set_style(
                        ProgressStyle::with_template(template.as_str())
                            .unwrap_or_else(|error| {
                                panic!(
                                    "`ProgressStyle` template was invalid. Template: `{template:?}`. Error: {error}"
                                )
                            })
                            .progress_chars("█▉▊▋▌▍▎▏  "),
                    );
                },
            );
        }
    }

    #[cfg(feature = "output_progress")]
    async fn progress_update(&mut self, progress_update: ProgressUpdate) {
        match self.progress_format {
            CliProgressFormatChosen::ProgressBar => {
                // Don't need to do anything, as `indicatif` handles output to
                // terminal.
            }
            CliProgressFormatChosen::Output => match self.format {
                // Note: outputting yaml for Text output, because we aren't sending much progress
                // information.
                //
                // We probably need to send more information in the `ProgressUpdate`, i.e. which
                // item it came from.
                OutputFormat::Text | OutputFormat::Yaml => {
                    let _unused = self
                        .output_yaml::<E, _, _>(&progress_update, Error::ProgressUpdateSerialize)
                        .await;
                }
                #[cfg(feature = "output_json")]
                OutputFormat::Json => {
                    let _unused = self
                        .output_json::<E, _, _>(
                            &progress_update,
                            Error::ProgressUpdateSerializeJson,
                        )
                        .await;
                }
            },
        }
    }

    #[cfg(feature = "output_progress")]
    async fn progress_end(&mut self, _cmd_progress_tracker: &CmdProgressTracker) {}

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
