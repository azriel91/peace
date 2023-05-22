use peace_fmt::Presentable;
use peace_rt_model_core::{
    async_trait,
    output::{OutputFormat, OutputWrite},
    Error, NativeError,
};
use serde::Serialize;
use tokio::io::{AsyncWrite, AsyncWriteExt, Stdout};

use crate::output::{CliMdPresenter, CliOutputBuilder};

#[cfg(feature = "output_colorized")]
use crate::output::CliColorize;

cfg_if::cfg_if! {
    if #[cfg(feature = "output_progress")] {
        use peace_core::progress::{
            ProgressComplete,
            ProgressLimit,
            ProgressStatus,
            ProgressTracker,
            ProgressUpdate,
            ProgressUpdateAndId,
        };
        use peace_rt_model_core::{
            indicatif::{ProgressDrawTarget, ProgressStyle},
            CmdProgressTracker,
        };

        use crate::output::{CliOutputTarget, CliProgressFormat};
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
    pub(crate) outcome_format: OutputFormat,
    /// Whether output should be colorized.
    #[cfg(feature = "output_colorized")]
    pub(crate) colorize: CliColorize,
    #[cfg(feature = "output_progress")]
    /// Where to output progress updates to -- stdout or stderr.
    pub(crate) progress_target: CliOutputTarget,
    /// Whether the writer is an interactive terminal.
    ///
    /// This is detected on instantiation.
    #[cfg(feature = "output_progress")]
    pub(crate) progress_format: CliProgressFormat,
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

/// This is used when we are rendering a bar that is not calculated by
/// `ProgressBar`'s length and current value,
#[cfg(feature = "output_progress")]
const BAR_EMPTY: &str = "▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱";
#[cfg(all(feature = "output_progress", feature = "output_colorized"))]
const BAR_FULL: &str = "▰▰▰▰▰▰▰▰▰▰▰▰▰▰▰▰▰▰▰▰▰▰▰▰▰▰▰▰▰▰▰▰▰▰▰▰▰▰▰▰";
#[cfg(feature = "output_progress")]
const SPINNER_EMPTY: &str = "▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱";
#[cfg(feature = "output_progress")]
const SPINNER_FULL: &str = "▰▰▰▰▰▰▰▰▰▰▰▰▰▰▰▰▰▰▰▰▰▰▰▰▰▰▰▰▰▰▰▰▰▰▰▰▰▰▰▰";

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

    /// Returns how to format outcome output -- human readable or machine
    /// parsable.
    pub fn outcome_format(&self) -> OutputFormat {
        self.outcome_format
    }

    /// Returns whether output should be colorized.
    #[cfg(feature = "output_colorized")]
    pub fn colorize(&self) -> CliColorize {
        self.colorize
    }

    /// Returns where to output progress updates to -- stdout or stderr.
    ///
    /// If the `"output_in_memory"` feature is enabled, there is a third
    /// `InMemory` variant that holds the buffer for progress output. This
    /// variant is intended to be used for verifying output in tests.
    #[cfg(feature = "output_progress")]
    pub fn progress_target(&self) -> &CliOutputTarget {
        &self.progress_target
    }

    /// Returns how to format progress output -- progress bar or mimic outcome
    /// format.
    #[cfg(feature = "output_progress")]
    pub fn progress_format(&self) -> CliProgressFormat {
        self.progress_format
    }

    async fn output_presentable<E, P>(&mut self, presentable: P) -> Result<(), E>
    where
        E: std::error::Error + From<Error>,
        P: Presentable,
    {
        let presenter = &mut CliMdPresenter::new(self);
        presentable
            .present(presenter)
            .await
            .map_err(NativeError::CliOutputPresent)
            .map_err(Error::Native)?;

        self.writer
            .flush()
            .await
            .map_err(NativeError::CliOutputPresent)
            .map_err(Error::Native)?;

        Ok(())
    }

    async fn output_yaml<'f, E, T, F>(&mut self, t: &T, fn_error: F) -> Result<(), E>
    where
        E: std::error::Error + From<Error>,
        T: Serialize + ?Sized,
        F: FnOnce(serde_yaml::Error) -> Error,
    {
        let t_serialized = serde_yaml::to_string(t).map_err(fn_error)?;

        self.writer
            .write_all(t_serialized.as_bytes())
            .await
            .map_err(NativeError::StdoutWrite)
            .map_err(Error::Native)?;

        Ok(())
    }

    #[cfg(feature = "output_json")]
    async fn output_json<'f, E, T, F>(&mut self, t: &T, fn_error: F) -> Result<(), E>
    where
        E: std::error::Error + From<Error>,
        T: Serialize + ?Sized,
        F: FnOnce(serde_json::Error) -> Error,
    {
        let t_serialized = serde_json::to_string(t).map_err(fn_error)?;

        self.writer
            .write_all(t_serialized.as_bytes())
            .await
            .map_err(NativeError::StdoutWrite)
            .map_err(Error::Native)?;

        Ok(())
    }

    #[cfg(feature = "output_progress")]
    fn progress_bar_style_update(&self, progress_tracker: &ProgressTracker) {
        let template = self.progress_bar_template(progress_tracker);
        let progress_bar = progress_tracker.progress_bar();
        progress_bar.set_style(
            ProgressStyle::with_template(template.as_str())
                .unwrap_or_else(|error| {
                    panic!(
                        "`ProgressStyle` template was invalid. Template: `{template:?}`. Error: {error}"
                    )
                })
                .progress_chars("▰▱")
                .tick_strings(&[
                    SPINNER_EMPTY,
                    "▰▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱",
                    "▰▰▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱",
                    "▰▰▰▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱",
                    "▰▰▰▰▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱",
                    "▰▰▰▰▰▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱",
                    "▰▰▰▰▰▰▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱",
                    "▰▰▰▰▰▰▰▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱",
                    "▱▰▰▰▰▰▰▰▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱",
                    "▱▱▰▰▰▰▰▰▰▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱",
                    "▱▱▱▰▰▰▰▰▰▰▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱",
                    "▱▱▱▱▰▰▰▰▰▰▰▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱",
                    "▱▱▱▱▱▰▰▰▰▰▰▰▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱",
                    "▱▱▱▱▱▱▰▰▰▰▰▰▰▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱",
                    "▱▱▱▱▱▱▱▰▰▰▰▰▰▰▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱",
                    "▱▱▱▱▱▱▱▱▰▰▰▰▰▰▰▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱",
                    "▱▱▱▱▱▱▱▱▱▰▰▰▰▰▰▰▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱",
                    "▱▱▱▱▱▱▱▱▱▱▰▰▰▰▰▰▰▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱",
                    "▱▱▱▱▱▱▱▱▱▱▱▰▰▰▰▰▰▰▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱",
                    "▱▱▱▱▱▱▱▱▱▱▱▱▰▰▰▰▰▰▰▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱",
                    "▱▱▱▱▱▱▱▱▱▱▱▱▱▰▰▰▰▰▰▰▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱",
                    "▱▱▱▱▱▱▱▱▱▱▱▱▱▱▰▰▰▰▰▰▰▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱",
                    "▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▰▰▰▰▰▰▰▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱",
                    "▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▰▰▰▰▰▰▰▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱",
                    "▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▰▰▰▰▰▰▰▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱",
                    "▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▰▰▰▰▰▰▰▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱",
                    "▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▰▰▰▰▰▰▰▱▱▱▱▱▱▱▱▱▱▱▱▱▱",
                    "▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▰▰▰▰▰▰▰▱▱▱▱▱▱▱▱▱▱▱▱▱",
                    "▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▰▰▰▰▰▰▰▱▱▱▱▱▱▱▱▱▱▱▱",
                    "▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▰▰▰▰▰▰▰▱▱▱▱▱▱▱▱▱▱▱",
                    "▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▰▰▰▰▰▰▰▱▱▱▱▱▱▱▱▱▱",
                    "▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▰▰▰▰▰▰▰▱▱▱▱▱▱▱▱▱",
                    "▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▰▰▰▰▰▰▰▱▱▱▱▱▱▱▱",
                    "▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▰▰▰▰▰▰▰▱▱▱▱▱▱▱",
                    "▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▰▰▰▰▰▰▰▱▱▱▱▱▱",
                    "▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▰▰▰▰▰▰▰▱▱▱▱▱",
                    "▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▰▰▰▰▰▰▰▱▱▱▱",
                    "▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▰▰▰▰▰▰▰▱▱▱",
                    "▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▰▰▰▰▰▰▰▱▱",
                    "▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▰▰▰▰▰▰▰▱",
                    "▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▰▰▰▰▰▰▰",
                    "▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▰▰▰▰▰▰",
                    "▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▰▰▰▰▰",
                    "▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▰▰▰▰",
                    "▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▰▰▰",
                    "▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▰▰",
                    "▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▱▰",
                    SPINNER_FULL,
                ]),
        );

        // Rerender the progress bar after setting style.
        progress_bar.tick();
    }

    #[cfg(feature = "output_progress")]
    fn progress_bar_template(&self, progress_tracker: &ProgressTracker) -> String {
        let icon = match progress_tracker.progress_status() {
            ProgressStatus::Initialized | ProgressStatus::ExecPending | ProgressStatus::Running => {
                "⏳"
            }
            ProgressStatus::RunningStalled | ProgressStatus::UserPending => "⏰",
            ProgressStatus::Complete(ProgressComplete::Success) => "✅",
            ProgressStatus::Complete(ProgressComplete::Fail) => "❌",
        };

        cfg_if::cfg_if! {
            if #[cfg(feature = "output_colorized")] {

                // These are used to tell `indicatif` how to style the computed bar.
                //
                // 40: width
                //
                //  32: blue pale (running)
                //  17: blue dark (running background)
                // 222: yellow pale (stalled)
                //  75: indigo pale (user pending, item id)
                //  35: green pale (success)
                //  22: green dark (success background)
                // 160: red slightly dim (fail)
                //  88: red dark (fail background)

                const GRAY_DARK: u8 = 237;
                const GRAY_MED: u8 = 8;
                const GREEN_LIGHT: u8 = 35;
                const PURPLE: u8 = 128;
                const RED_DIM: u8 = 160;

                let bar_or_spinner = match self.colorize {
                    CliColorize::Colored => {
                        if progress_tracker.progress_limit().is_some() {
                            // Colored, with progress limit
                            match progress_tracker.progress_status() {
                                ProgressStatus::Initialized => console::style(BAR_EMPTY).color256(GRAY_DARK),
                                ProgressStatus::ExecPending | ProgressStatus::Running => {
                                    console::style("{bar:40.32}")
                                }
                                ProgressStatus::RunningStalled => console::style("{bar:40.222}"),
                                ProgressStatus::UserPending => console::style("{bar:40.75}"),
                                ProgressStatus::Complete(progress_complete) => match progress_complete {
                                    // Ideally we just use `"{bar:40.35}"`,
                                    // and the `ProgressBar` renders the filled green bar.
                                    //
                                    // However, it's still rendered as blue because
                                    // the `ProgressBar` is abandoned before getting one
                                    // final render.
                                    ProgressComplete::Success => {
                                        console::style(BAR_FULL).color256(GREEN_LIGHT)
                                    },
                                    ProgressComplete::Fail => console::style("{bar:40.160}"),
                                },
                            }
                        } else {
                            // Colored, no progress limit (as opposed to unknown)
                            match progress_tracker.progress_status() {
                                ProgressStatus::Initialized => console::style(SPINNER_EMPTY).color256(GRAY_MED),
                                ProgressStatus::ExecPending | ProgressStatus::Running => {
                                    console::style("{spinner:40.32}")
                                }
                                ProgressStatus::RunningStalled => console::style("{spinner:40.222}"),
                                ProgressStatus::UserPending => console::style("{spinner:40.75}"),
                                ProgressStatus::Complete(progress_complete) => match progress_complete {
                                    // Ideally we just use `"{spinner:40.35}"`,
                                    // and the `ProgressBar` renders the filled green spinner.
                                    // However, for a spinner, it just renders it empty for some
                                    // reason.
                                    ProgressComplete::Success => {
                                        console::style(SPINNER_FULL).color256(GREEN_LIGHT)
                                    },
                                    ProgressComplete::Fail => console::style(SPINNER_FULL).color256(RED_DIM),
                                },
                            }
                        }
                    }
                    CliColorize::Uncolored => {
                        if progress_tracker.progress_limit().is_some() {
                            match progress_tracker.progress_status() {
                                ProgressStatus::Initialized => console::style(BAR_EMPTY),
                                ProgressStatus::ExecPending | ProgressStatus::Running |
                                ProgressStatus::RunningStalled |
                                ProgressStatus::UserPending |
                                ProgressStatus::Complete(_) => console::style("{bar:40}"),
                            }
                        } else {
                            console::style("{spinner:40}")
                        }
                    },
                };
            } else {
                // "output_colorized" feature disabled
                let bar_or_spinner = if progress_tracker.progress_limit().is_some() {
                    match progress_tracker.progress_status() {
                        ProgressStatus::Initialized => console::style(BAR_EMPTY),
                        ProgressStatus::ExecPending | ProgressStatus::Running |
                        ProgressStatus::RunningStalled |
                        ProgressStatus::UserPending |
                        ProgressStatus::Complete(_) => console::style("{bar:40}"),
                    }
                } else {
                    console::style("{spinner:40}")
                };
            }
        }
        let prefix = "{prefix:20}";

        let (progress_is_complete, completion_is_successful) =
            match progress_tracker.progress_status() {
                ProgressStatus::Complete(progress_complete) => {
                    (true, progress_complete.is_successful())
                }
                _ => (false, false),
            };
        let units = if progress_is_complete && completion_is_successful {
            None
        } else {
            progress_tracker
                .progress_limit()
                .and_then(|progress_limit| match progress_limit {
                    ProgressLimit::Unknown => None,
                    ProgressLimit::Steps(_) => Some(" {pos}/{len}"),
                    ProgressLimit::Bytes(_) => Some(" {bytes}/{total_bytes}"),
                })
        };
        let elapsed_eta = if progress_is_complete {
            None
        } else {
            cfg_if::cfg_if! {
                if #[cfg(feature = "output_colorized")] {
                    let elapsed_eta = console::style("(el: {elapsed}, eta: {eta})");
                    match self.colorize {
                        CliColorize::Colored => Some(elapsed_eta.color256(PURPLE)),
                        CliColorize::Uncolored => Some(elapsed_eta),
                    }
                } else {
                    Some(console::style("(el: {elapsed}, eta: {eta})"))
                }
            }
        };

        // `prefix` is the item ID.
        let mut format_str = format!("{icon} {prefix} {bar_or_spinner}");
        if let Some(units) = units {
            format_str.push_str(units);
        }
        if progress_tracker.message().is_some() {
            format_str.push_str(" {msg}");
        }
        if let Some(elapsed_eta) = elapsed_eta {
            format_str.push_str(&format!(" {elapsed_eta}"));
        }

        format_str
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
        let progress_draw_target = match &self.progress_target {
            CliOutputTarget::Stdout => ProgressDrawTarget::stdout(),
            CliOutputTarget::Stderr => ProgressDrawTarget::stderr(),
            #[cfg(feature = "output_in_memory")]
            CliOutputTarget::InMemory(in_memory_term) => {
                ProgressDrawTarget::term_like(Box::new(in_memory_term.clone()))
            }
        };

        // avoid reborrowing `self` within `for_each`
        #[cfg(feature = "output_colorized")]
        let colorize = self.colorize;

        match self.progress_format {
            CliProgressFormat::ProgressBar => {
                cmd_progress_tracker
                    .multi_progress()
                    .set_draw_target(progress_draw_target);

                cmd_progress_tracker
                    .progress_trackers()
                    .iter()
                    .enumerate()
                    .for_each(|(index, (item_id, progress_tracker))| {
                        let progress_bar = progress_tracker.progress_bar();

                        // Hack: colourization done in `progress_begin` to get
                        // numerical index to be colorized.
                        let index = index + 1;
                        cfg_if::cfg_if! {
                            if #[cfg(feature = "output_colorized")] {
                                match colorize {
                                    CliColorize::Colored => {
                                        // white
                                        let index_colorized = console::Style::new()
                                            .color256(15)
                                            .apply_to(format!("{index}."));
                                        // blue
                                        let item_id_colorized = console::Style::new()
                                            .color256(75)
                                            .apply_to(format!("{item_id}"));
                                        progress_bar.set_prefix(
                                            format!("{index_colorized} {item_id_colorized}")
                                        );
                                    }
                                    CliColorize::Uncolored => {
                                        progress_bar.set_prefix(format!("{index}. {item_id}"));
                                    }
                                }
                            } else {
                                progress_bar.set_prefix(format!("{index}. {item_id}"));
                            }
                        }

                        self.progress_bar_style_update(progress_tracker);

                        // Hack: This should be done with a timer in `ApplyCmd`.
                        // This uses threads, which is not WASM compatible.
                        progress_bar.enable_steady_tick(std::time::Duration::from_millis(100));
                    });
            }
            CliProgressFormat::Outcome => {
                cmd_progress_tracker
                    .multi_progress()
                    .set_draw_target(progress_draw_target);

                let progress_style = ProgressStyle::with_template("").unwrap_or_else(|error| {
                    panic!("`ProgressStyle` template was invalid. Template: `\"\"`. Error: {error}")
                });
                cmd_progress_tracker.progress_trackers().iter().for_each(
                    |(item_id, progress_tracker)| {
                        let progress_bar = progress_tracker.progress_bar();
                        progress_bar.set_prefix(format!("{item_id}"));
                        progress_bar.set_style(progress_style.clone());
                    },
                );
            }
            CliProgressFormat::None => {}
        }
    }

    #[cfg(feature = "output_progress")]
    async fn progress_update(
        &mut self,
        progress_tracker: &ProgressTracker,
        progress_update_and_id: &ProgressUpdateAndId,
    ) {
        match self.progress_format {
            CliProgressFormat::ProgressBar => {
                // Don't need to write anything, as `indicatif` handles output
                // to terminal.

                // * Need to update progress bar colour on the first delta (grey to blue)
                // * Need to update progress bar colour on finish (blue to green)
                // * Need to update progress bar colour on error (blue to red)

                if let Some(message) = progress_tracker.message().cloned() {
                    progress_tracker.progress_bar().set_message(message);
                }

                match &progress_update_and_id.progress_update {
                    ProgressUpdate::Reset => {
                        self.progress_bar_style_update(progress_tracker);
                    }
                    ProgressUpdate::Limit(_progress_limit) => {
                        // Note: `progress_tracker` also carries the `progress_limit`
                        self.progress_bar_style_update(progress_tracker);
                    }
                    ProgressUpdate::Delta(_delta) => {
                        // Status may have changed from `ExecPending` to
                        // `Running`.
                        //
                        // We don't have the previous status though.
                        //
                        // TODO: Is this too much of a performance hit, and we send another message
                        // for spinners?
                        self.progress_bar_style_update(progress_tracker);
                    }
                    ProgressUpdate::Complete(progress_complete) => match progress_complete {
                        ProgressComplete::Success => {
                            self.progress_bar_style_update(progress_tracker);

                            let progress_bar = progress_tracker.progress_bar();
                            progress_bar.finish();
                        }
                        ProgressComplete::Fail => {
                            self.progress_bar_style_update(progress_tracker);

                            let progress_bar = progress_tracker.progress_bar();
                            progress_bar.abandon();
                        }
                    },
                }
            }
            CliProgressFormat::Outcome => {
                let progress_bar = progress_tracker.progress_bar();
                match self.outcome_format {
                    // Note: outputting yaml for Text output, because we aren't sending much
                    // progress information.
                    //
                    // We probably need to send more information in the `ProgressUpdate`, i.e. which
                    // item it came from.
                    OutputFormat::Text | OutputFormat::Yaml => {
                        let _progress_display_unused =
                            serde_yaml::to_string(progress_update_and_id).map(|t_serialized| {
                                progress_bar.println("---");
                                progress_bar.println(t_serialized);
                            });
                    }
                    #[cfg(feature = "output_json")]
                    OutputFormat::Json => {
                        let _progress_display_unused =
                            serde_json::to_string(progress_update_and_id).map(|t_serialized| {
                                progress_bar.println(t_serialized);
                            });
                    }
                }
            }
            CliProgressFormat::None => {}
        }
    }

    #[cfg(feature = "output_progress")]
    async fn progress_end(&mut self, cmd_progress_tracker: &CmdProgressTracker) {
        match self.progress_format {
            CliProgressFormat::ProgressBar => {
                // Hack: This should be done with a timer in `ApplyCmd`.
                // This uses threads, which is not WASM compatible.
                cmd_progress_tracker.progress_trackers().iter().for_each(
                    |(_item_id, progress_tracker)| {
                        let progress_bar = progress_tracker.progress_bar();
                        progress_bar.disable_steady_tick();
                        progress_bar.tick();
                    },
                );

                // Prevents progress bars from drawing over error messages.
                cmd_progress_tracker
                    .multi_progress
                    .set_draw_target(ProgressDrawTarget::hidden());

                // Add spacing between end of progress bars and next output.
                //
                // For some reason it needs two newlines, `indicatif` possibly
                // moves the cursor up a line.
                let (Ok(()) | Err(_)) = self.writer.write_all(b"\n\n").await;
            }
            CliProgressFormat::Outcome | CliProgressFormat::None => {}
        }
    }

    async fn present<P>(&mut self, presentable: P) -> Result<(), E>
    where
        P: Presentable,
    {
        match self.outcome_format {
            OutputFormat::Text => self.output_presentable(presentable).await,
            OutputFormat::Yaml => self.output_yaml(&presentable, Error::StatesSerialize).await,
            #[cfg(feature = "output_json")]
            OutputFormat::Json => {
                self.output_json(&presentable, Error::StatesSerializeJson)
                    .await
            }
        }
    }

    async fn write_err(&mut self, error: &E) -> Result<(), E> {
        match self.outcome_format {
            OutputFormat::Text => {
                self.writer
                    .write_all(format!("{error}\n").as_bytes())
                    .await
                    .map_err(NativeError::StdoutWrite)
                    .map_err(Error::Native)?;
            }
            OutputFormat::Yaml => {
                // TODO: proper parsable structure with error code.
                let error_serialized =
                    serde_yaml::to_string(&format!("{error}")).map_err(Error::ErrorSerialize)?;
                self.writer
                    .write_all(error_serialized.as_bytes())
                    .await
                    .map_err(NativeError::StdoutWrite)
                    .map_err(Error::Native)?;
            }
            #[cfg(feature = "output_json")]
            OutputFormat::Json => {
                // TODO: proper parsable structure with error code.
                let error_serialized = serde_json::to_string(&format!("{error}"))
                    .map_err(Error::ErrorSerializeJson)?;
                self.writer
                    .write_all(error_serialized.as_bytes())
                    .await
                    .map_err(NativeError::StdoutWrite)
                    .map_err(Error::Native)?;
            }
        }

        Ok(())
    }
}
