#[cfg(feature = "error_reporting")]
use peace::miette::{self, SourceSpan};

use crate::{CmdVariant, ShCmd};

/// Error while managing command execution.
#[cfg_attr(feature = "error_reporting", derive(peace::miette::Diagnostic))]
#[derive(Debug, thiserror::Error)]
pub enum ShCmdError {
    /// A command script was not resolved during execution.
    ///
    /// This could be due to it not being provided, or it failed to be looked up
    /// when needed.
    #[error("Script not resolved for: `{cmd_variant}`.")]
    #[cfg_attr(
        feature = "error_reporting",
        diagnostic(
            code(peace_item_sh_cmd::cmd_script_not_exists),
            help("Check if the `{cmd_variant}` is provided in params.")
        )
    )]
    CmdScriptNotResolved {
        /// The cmd variant that was not existent.
        cmd_variant: CmdVariant,
    },

    /// Failed to execute command.
    #[error("Failed to execute command: `{}`", sh_cmd)]
    #[cfg_attr(
        feature = "error_reporting",
        diagnostic(code(peace_item_sh_cmd::cmd_exec_fail))
    )]
    CmdExecFail {
        /// The command that failed to be executed.
        sh_cmd: ShCmd,
        /// The command that failed to be executed as a string.
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
        diagnostic(code(peace_item_sh_cmd::stdout_non_utf8)),
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
        diagnostic(code(peace_item_sh_cmd::stderr_non_utf8)),
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

    /// Ensure check shell command did not output "true" or "false".
    #[error(
        r#"Ensure check shell command did not return "true" or "false": `{}`"#,
        sh_cmd
    )]
    #[cfg_attr(
        feature = "error_reporting",
        diagnostic(code(peace_item_sh_cmd::ensure_check_value_not_boolean)),
        help(
            r#"Update the command to return "true" if execution is required, or "false" if not."#
        )
    )]
    EnsureCheckValueNotBoolean {
        /// The ensure check shell command.
        sh_cmd: ShCmd,
        /// The ensure check shell command as a string.
        #[cfg(feature = "error_reporting")]
        #[source_code]
        sh_cmd_string: String,
        /// Stdout.
        stdout: Option<String>,
    },

    /// Clean check shell command did not output "true" or "false".
    #[error(
        r#"Clean check shell command did not return "true" or "false": `{}`"#,
        sh_cmd
    )]
    #[cfg_attr(
        feature = "error_reporting",
        diagnostic(code(peace_item_sh_cmd::clean_check_value_not_boolean)),
        help(
            r#"Update the command to return "true" if execution is required, or "false" if not."#
        )
    )]
    CleanCheckValueNotBoolean {
        /// The clean check shell command.
        sh_cmd: ShCmd,
        /// The clean check shell command as a string.
        #[cfg(feature = "error_reporting")]
        #[source_code]
        sh_cmd_string: String,
        /// Stdout.
        stdout: Option<String>,
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
