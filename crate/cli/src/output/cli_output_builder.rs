use std::io::IsTerminal;

use peace_cli_model::OutputFormat;
use tokio::io::{AsyncWrite, Stdout};

use crate::output::{CliColorize, CliColorizeOpt, CliOutput};

cfg_if::cfg_if! {
    if #[cfg(feature = "output_progress")] {
        use crate::output::{CliOutputTarget, CliProgressFormatOpt, CliProgressFormat};
    }
}

/// An `OutputWrite` implementation that writes to the command line.
///
/// # Features
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
pub struct CliOutputBuilder<W> {
    /// Output stream to write the command outcome to.
    writer: W,
    /// How to format outcome output -- human readable or machine parsable.
    outcome_format: OutputFormat,
    /// Whether output should be colorized.
    colorize: CliColorizeOpt,
    /// Where to output progress updates to -- stdout or stderr.
    #[cfg(feature = "output_progress")]
    progress_target: CliOutputTarget,
    /// How to format progress output -- progress bar or mimic outcome format.
    ///
    /// This is detected on instantiation.
    #[cfg(feature = "output_progress")]
    progress_format: CliProgressFormatOpt,
}

impl CliOutputBuilder<Stdout> {
    /// Returns a new `CliOutputBuilder`.
    ///
    /// This uses:
    ///
    /// * `io::stdout()` as the outcome output stream.
    /// * `io::stderr()` as the progress output stream if `"output_progress"` is
    ///   enabled.
    pub fn new() -> Self {
        Self::default()
    }
}

impl<W> CliOutputBuilder<W>
where
    W: AsyncWrite + std::marker::Unpin,
{
    /// Returns a new `CliOutput` using `io::stdout()` as the output stream.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use peace_rt_model_native::output::CliOutput;
    /// // use peace::rt_model::output::CliOutputBuilder;
    ///
    /// let mut buffer = Vec::<u8>::new();
    /// let cli_output = CliOutputBuilder::new_with_writer(&mut buffer).build();
    /// ```
    pub fn new_with_writer(writer: W) -> Self {
        Self {
            writer,
            outcome_format: OutputFormat::Text,
            colorize: CliColorizeOpt::Auto,
            #[cfg(feature = "output_progress")]
            progress_target: CliOutputTarget::default(),
            #[cfg(feature = "output_progress")]
            progress_format: CliProgressFormatOpt::Auto,
        }
    }

    /// Returns how to format outcome output -- human readable or machine
    /// parsable.
    pub fn outcome_format(&self) -> OutputFormat {
        self.outcome_format
    }

    /// Returns whether output should be colorized.
    pub fn colorize(&self) -> CliColorizeOpt {
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
    pub fn progress_format(&self) -> CliProgressFormatOpt {
        self.progress_format
    }

    /// Sets the outcome output format for this `CliOutput`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use peace_rt_model_core::output::OutputFormat;
    /// # use peace_rt_model_native::output::CliOutput;
    /// // use peace::rt_model::output::{CliOutput, OutputFormat};
    ///
    /// let cli_output = CliOutput::builder().with_outcome_format(OutputFormat::Yaml);
    /// ```
    pub fn with_outcome_format(mut self, output_format: OutputFormat) -> Self {
        self.outcome_format = output_format;
        self
    }

    /// Enables colorized output.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use peace_rt_model_native::output::CliOutput;
    /// // use peace::rt_model::output::{CliColorize, CliOutput};
    ///
    /// let cli_output = CliOutput::new().with_colorized(CliColorize::Auto);
    /// ```
    pub fn with_colorize(mut self, colorize: CliColorizeOpt) -> Self {
        self.colorize = colorize;
        self
    }

    /// Sets the progress output target -- stdout or stderr (default).
    #[cfg(feature = "output_progress")]
    pub fn with_progress_target(mut self, progress_target: CliOutputTarget) -> Self {
        self.progress_target = progress_target;
        self
    }

    /// Sets the progress output format.
    #[cfg(feature = "output_progress")]
    pub fn with_progress_format(mut self, progress_format: CliProgressFormatOpt) -> Self {
        self.progress_format = progress_format;
        self
    }

    /// Builds and returns the `CliOutput`.
    pub fn build(self) -> CliOutput<W> {
        let CliOutputBuilder {
            writer,
            outcome_format,
            colorize,
            #[cfg(feature = "output_progress")]
            progress_target,
            #[cfg(feature = "output_progress")]
            progress_format,
        } = self;

        let colorize = match colorize {
            CliColorizeOpt::Auto => {
                // Even though we're using `tokio::io::stdout` / `stderr`, `IsTerminal` is only
                // implemented on `std::io::stdout` / `stderr`.
                //
                // TODO: This should really determine this based on `W`, but:
                //
                // * We cannot easily tell if we are using `stdout`, `stderr`, or some arbitrary
                //   thing.
                // * We *could* implement a function per `CliOutputBuilder<Stdout>` or
                //   `CliOutputBuilder<Stderr>`, but then we're missing it for arbitrary `W`s.
                // * If we take in a `CliOutputTarget` for outcome output instead of `W`, then
                //   we cannot pass in an arbitrary `AsyncWrite`.
                // * If we extend `CliOutputTarget` to support any `W`, that variant will no
                //   longer be compatible with the progress output, handled by `indicatif`.
                // * We *could* add another enum just like `CliOutputTarget`, with the
                //   additional variant.
                if std::io::stdout().is_terminal() {
                    CliColorize::Colored
                } else {
                    CliColorize::Uncolored
                }
            }
            CliColorizeOpt::Always => CliColorize::Colored,
            CliColorizeOpt::Never => CliColorize::Uncolored,
        };

        #[cfg(feature = "output_progress")]
        let progress_format = match progress_format {
            CliProgressFormatOpt::Auto => {
                // Even though we're using `tokio::io::stdout` / `stderr`, `IsTerminal` is only
                // implemented on `std::io::stdout` / `stderr`.
                match &progress_target {
                    CliOutputTarget::Stdout => {
                        if std::io::stdout().is_terminal() {
                            CliProgressFormat::ProgressBar
                        } else {
                            CliProgressFormat::Outcome
                        }
                    }
                    CliOutputTarget::Stderr => {
                        if std::io::stderr().is_terminal() {
                            CliProgressFormat::ProgressBar
                        } else {
                            CliProgressFormat::Outcome
                        }
                    }
                    #[cfg(feature = "output_in_memory")]
                    CliOutputTarget::InMemory(_) => CliProgressFormat::ProgressBar,
                }
            }
            CliProgressFormatOpt::Outcome => CliProgressFormat::Outcome,
            CliProgressFormatOpt::ProgressBar => CliProgressFormat::ProgressBar,
            CliProgressFormatOpt::None => CliProgressFormat::None,
        };

        // We need to suppress the `^C\n` characters that come through the terminal.
        //
        // This is a combination of reading the following:
        //
        // * <https://stackoverflow.com/questions/42563400/hide-c-pressing-ctrl-c-in-c>
        // * <https://docs.rs/libc/latest/libc/constant.ECHOCTL.html>
        // * <https://docs.rs/raw_tty/0.1.0/raw_tty/struct.TtyWithGuard.html>
        //
        // Also considered were:
        //
        // * [`uu_stty`](https://crates.io/crates/uu_stty)], but it doesn't look like it
        //   supports modifying the terminal.
        // * [`unix_tty`](https://docs.rs/unix-tty/0.3.4/unix_tty/), which looks like it
        //   can do the job, but `raw_tty` has a drop guard to restore the original tty
        //   settings.
        #[cfg(unix)]
        let stdin_tty_with_guard = {
            use raw_tty::GuardMode;

            match std::io::stdin().guard_mode() {
                Ok(mut stdin_tty_with_guard) => {
                    let mode_modify_result = stdin_tty_with_guard.modify_mode(|mut ios| {
                        ios.c_lflag &= !libc::ECHOCTL;
                        ios
                    });
                    match mode_modify_result {
                        Ok(()) => {}
                        Err(error) => {
                            #[cfg(debug_assertions)]
                            eprintln!("warn: Failed to modify termios mode:\n{error}");
                        }
                    }
                    Some(stdin_tty_with_guard)
                }
                Err(error) => {
                    #[cfg(debug_assertions)]
                    eprintln!("warn: Failed to acquire stdin termios:\n{error}");
                    None
                }
            }
        };

        CliOutput {
            writer,
            outcome_format,
            colorize,
            #[cfg(feature = "output_progress")]
            progress_target,
            #[cfg(feature = "output_progress")]
            progress_format,
            #[cfg(feature = "output_progress")]
            pb_item_id_width: None,
            #[cfg(unix)]
            stdin_tty_with_guard,
        }
    }
}

impl Default for CliOutputBuilder<Stdout> {
    fn default() -> Self {
        let stdout = tokio::io::stdout();
        Self {
            writer: stdout,
            outcome_format: OutputFormat::Text,
            colorize: CliColorizeOpt::Auto,
            #[cfg(feature = "output_progress")]
            progress_target: CliOutputTarget::default(),
            #[cfg(feature = "output_progress")]
            progress_format: CliProgressFormatOpt::Auto,
        }
    }
}
