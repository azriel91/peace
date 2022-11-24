#[cfg(feature = "error_reporting")]
use peace::miette::{self, SourceSpan};

use crate::ShCmd;

/// Error while managing command execution.
#[cfg_attr(feature = "error_reporting", derive(peace::miette::Diagnostic))]
#[derive(Debug, thiserror::Error)]
pub enum ShCmdError {
    /// Failed to execute command.
    #[error("Failed to execute command: `{}`", sh_cmd)]
    #[cfg_attr(
        feature = "error_reporting",
        diagnostic(code(peace_item_spec_sh_cmd::cmd_exec_fail))
    )]
    CmdExecFail {
        /// The command that failed to be executed.
        sh_cmd: ShCmd,
        #[cfg(feature = "error_reporting")]
        #[source_code]
        sh_cmd_string: String,
        /// Underlying IO error.
        #[source]
        error: std::io::Error,
    },

    /// Command produced non-UTF-8 stdout output.
    #[error("Command produced non-UTF-8 stdout output: `{}`", sh_cmd)]
    #[cfg_attr(
        feature = "error_reporting",
        diagnostic(code(peace_item_spec_sh_cmd::output_non_utf8)),
        help(
            "Update the command to something that outputs UTF8: `{}`\n\
            Perhaps encode the output using `base64`",
            sh_cmd
        )
    )]
    StdoutNonUtf8 {
        /// The command whose stdout is not a valid UTF-8 string.
        sh_cmd: ShCmd,
        /// Lossy UTF-8 conversion of stdout.
        #[cfg_attr(feature = "error_reporting", source_code)]
        stdout_lossy: String,
        /// Span where the invalid bytes occur.
        #[cfg(feature = "error_reporting")]
        #[label]
        invalid_span: SourceSpan,
        /// Underlying Utf8 error.
        #[source]
        error: std::str::Utf8Error,
    },

    /// Command produced non-UTF-8 stderr output.
    #[error("Command produced non-UTF-8 stderr output: `{}`", sh_cmd)]
    #[cfg_attr(
        feature = "error_reporting",
        diagnostic(code(peace_item_spec_sh_cmd::output_non_utf8)),
        help(
            "Update the command to something that outputs UTF8: `{}`\n\
            Perhaps encode the output using `base64`",
            sh_cmd
        )
    )]
    StderrNonUtf8 {
        /// The command whose stderr is not a valid UTF-8 string.
        sh_cmd: ShCmd,
        /// Lossy UTF-8 conversion of stderr.
        #[cfg_attr(feature = "error_reporting", source_code)]
        stderr_lossy: String,
        /// Span where the invalid bytes occur.
        #[cfg(feature = "error_reporting")]
        #[label]
        invalid_span: SourceSpan,
        /// Underlying Utf8 error.
        #[source]
        error: std::str::Utf8Error,
    },

    // === Framework errors === //
    /// A `peace` runtime error occurred.
    #[error("A `peace` runtime error occurred.")]
    PeaceRtError(
        #[cfg_attr(feature = "error_reporting", diagnostic_source)]
        #[source]
        #[from]
        peace::rt_model::Error,
    ),
}
