use std::process::Stdio;

use chrono::Utc;
use peace::cfg::State;
use tokio::process::Command;

use crate::{ShCmd, ShCmdError, ShCmdExecutionRecord, ShCmdState};

/// Common code to run `ShCmd`s.
#[derive(Debug)]
pub(crate) struct ShCmdExecutor;

impl ShCmdExecutor {
    /// Executes the provided `ShCmd` and returns execution information.
    pub async fn exec(
        sh_cmd: &ShCmd,
    ) -> Result<State<ShCmdState, ShCmdExecutionRecord>, ShCmdError> {
        let start_datetime = Utc::now();
        let mut command: Command = sh_cmd.into();
        let output = command
            .stdin(Stdio::null())
            .kill_on_drop(true)
            .output()
            .await
            .map_err(|error| {
                #[cfg(feature = "error_reporting")]
                let sh_cmd_string = format!("{sh_cmd}");

                ShCmdError::CmdExecFail {
                    sh_cmd: sh_cmd.clone(),
                    #[cfg(feature = "error_reporting")]
                    sh_cmd_string,
                    error,
                }
            })?;
        let end_datetime = Utc::now();

        let stdout = String::from_utf8(output.stdout).map_err(|from_utf8_error| {
            let stdout_lossy = String::from_utf8_lossy(from_utf8_error.as_bytes()).to_string();
            let error = from_utf8_error.utf8_error();
            #[cfg(feature = "error_reporting")]
            let invalid_span = {
                let start = error.valid_up_to();
                let len = error.error_len().unwrap_or(1);
                miette::SourceSpan::from((start, len))
            };

            ShCmdError::StdoutNonUtf8 {
                sh_cmd: sh_cmd.clone(),
                stdout_lossy,
                #[cfg(feature = "error_reporting")]
                invalid_span,
                error,
            }
        })?;

        let stderr = String::from_utf8(output.stderr).map_err(|from_utf8_error| {
            let stderr_lossy = String::from_utf8_lossy(from_utf8_error.as_bytes()).to_string();
            let error = from_utf8_error.utf8_error();
            #[cfg(feature = "error_reporting")]
            let invalid_span = {
                let start = error.valid_up_to();
                let len = error.error_len().unwrap_or(1);
                miette::SourceSpan::from((start, len))
            };

            ShCmdError::StderrNonUtf8 {
                sh_cmd: sh_cmd.clone(),
                stderr_lossy,
                #[cfg(feature = "error_reporting")]
                invalid_span,
                error,
            }
        })?;

        Ok(State::new(
            ShCmdState::Some(stdout.clone()),
            ShCmdExecutionRecord::Some {
                start_datetime,
                end_datetime,
                stdout,
                stderr,
                exit_code: output.status.code(),
            },
        ))
    }
}
