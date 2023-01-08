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

use crate::{output::CliOutputBuilder, Error};

#[cfg(feature = "output_colorized")]
use crate::output::CliColorizeChosen;

cfg_if::cfg_if! {
    if #[cfg(feature = "output_progress")] {
        use peace_core::progress::{ProgressLimit, ProgressTracker, ProgressUpdate};
        use peace_rt_model_core::{
            indicatif::{ProgressBar, ProgressStyle, ProgressDrawTarget},
            CmdProgressTracker,
        };

        use crate::output::{CliOutputTarget, CliProgressFormatChosen};
    }
}

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
/// `indicatif`'s internal writing to `stdout` / `stderr` is used for rendering
/// progress bars, which uses sync Rust. I didn't figure out how to write the
/// in-memory term contents to the `W` writer correctly.
///
/// [`with_colorized`]: CliOutputBuilder::with_colorized
/// [`with_progress_format`]: CliOutputBuilder::with_progress_format
/// [`with_progress_target`]: CliOutputBuilder::with_progress_target
#[derive(Debug)]
pub struct CliOutput<W> {
    /// Output stream to write the command outcome to.
    pub(crate) writer: W,
    /// How to format command outcome output -- human readable or machine
    /// parsable.
    pub(crate) format: OutputFormat,
    /// Whether output should be colorized.
    #[cfg(feature = "output_colorized")]
    pub(crate) colorize: CliColorizeChosen,
    #[cfg(feature = "output_progress")]
    /// Where to output progress updates to -- stdout or stderr.
    pub(crate) progress_target: CliOutputTarget,
    /// Whether the writer is an interactive terminal.
    ///
    /// This is detected on instantiation.
    #[cfg(feature = "output_progress")]
    pub(crate) progress_format: CliProgressFormatChosen,
}

impl CliOutput<Stdout> {
    /// Returns a new `CliOutput`.
    ///
    /// This uses:
    ///
    /// * `io::stdout()` as the outcome output stream.
    /// * `io::stderr()` as the progress output stream if `"output_progress"` is
    ///   enabled.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns a new `CliOutputBuilder`.
    ///
    /// The builder allows additional options to be configured, such as forcing
    /// colorized output, outcome output format, and progress format.
    pub fn builder() -> CliOutputBuilder<Stdout> {
        CliOutputBuilder::new()
    }
}

impl<W> CliOutput<W>
where
    W: AsyncWrite + std::marker::Unpin,
{
    /// Returns a new `CliOutput` using the given writer for the output stream.
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
        CliOutputBuilder::new_with_writer(writer).build()
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
        let colorized = self.colorize;

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

    #[cfg(feature = "output_progress")]
    fn progress_bar_style_update(
        progress_bar: &ProgressBar,
        progress_limit: Option<ProgressLimit>,
    ) {
        let template = Self::progress_bar_template(progress_limit);
        progress_bar.set_style(
            ProgressStyle::with_template(template.as_str())
                .unwrap_or_else(|error| {
                    panic!(
                        "`ProgressStyle` template was invalid. Template: `{template:?}`. Error: {error}"
                    )
                })
                .progress_chars("█▉▊▋▌▍▎▏  "),
        );

        // Rerender the progress bar after setting style.
        progress_bar.tick();
    }

    #[cfg(feature = "output_progress")]
    fn progress_bar_template(progress_limit: Option<ProgressLimit>) -> String {
        // 40: width
        // 35: pale green
        // 32: pale blue
        // 17: dark blue
        let bar = "{bar:40.32.on_17}"; // TODO: change based on progress status
        let units = if let Some(progress_limit) = progress_limit {
            match progress_limit {
                ProgressLimit::Unknown => "",
                ProgressLimit::Steps(_) => "{pos}/{len}",
                ProgressLimit::Bytes(_) => "{bytes}/{total_bytes}",
            }
        } else {
            ""
        };

        // `prefix` is the item spec ID.
        format!("{{prefix:20}} ▕{bar}▏{units}")
    }
}

impl Default for CliOutput<Stdout> {
    fn default() -> Self {
        Self::builder().build()
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
                    Self::progress_bar_style_update(progress_bar, None);
                },
            );
        }
    }

    #[cfg(feature = "output_progress")]
    async fn progress_update(
        &mut self,
        progress_tracker: &ProgressTracker,
        progress_update: ProgressUpdate,
    ) {
        match self.progress_format {
            CliProgressFormatChosen::ProgressBar => {
                // Don't need to write anything, as `indicatif` handles output
                // to terminal.

                // * Need to update progress bar colour on the first delta (grey to blue)
                // * Need to update progress bar colour on finish (blue to green)
                // * Need to update progress bar colour on error (blue to red)

                match progress_update {
                    ProgressUpdate::Limit { limit } => {
                        Self::progress_bar_style_update(
                            progress_tracker.progress_bar(),
                            Some(limit),
                        );
                    }
                    ProgressUpdate::Delta { delta: _ } => {
                        // Nothing to do -- the progress bar is already updated
                        // within Peace.
                    }
                }
            }
            CliProgressFormatChosen::Output => match self.format {
                // Note: outputting yaml for Text output, because we aren't sending much progress
                // information.
                //
                // We probably need to send more information in the `ProgressUpdate`, i.e. which
                // item it came from.
                OutputFormat::Text | OutputFormat::Yaml => {
                    let _progress_send_unused = self
                        .output_yaml::<E, _, _>(&progress_update, Error::ProgressUpdateSerialize)
                        .await;
                }
                #[cfg(feature = "output_json")]
                OutputFormat::Json => {
                    let _progress_send_unused = self
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
