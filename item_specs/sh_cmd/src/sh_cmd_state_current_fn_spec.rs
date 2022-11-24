use std::{marker::PhantomData, process::Stdio};

use chrono::Utc;
use peace::cfg::{async_trait, FnSpec, State};
use tokio::process::Command;

use crate::{ShCmdData, ShCmdError, ShCmdExecutionRecord, ShCmdState};

/// Status `FnSpec` for the command to execute.
#[derive(Debug)]
pub struct ShCmdStateCurrentFnSpec<Id>(PhantomData<Id>);

#[async_trait(?Send)]
impl<Id> FnSpec for ShCmdStateCurrentFnSpec<Id>
where
    Id: Send + Sync + 'static,
{
    type Data<'op> = ShCmdData<'op, Id>;
    type Error = ShCmdError;
    type Output = State<ShCmdState, ShCmdExecutionRecord>;

    async fn exec(sh_cmd_data: ShCmdData<'_, Id>) -> Result<Self::Output, ShCmdError> {
        let start_datetime = Utc::now();
        let state_current_sh_cmd = sh_cmd_data.sh_cmd_params().state_current_sh_cmd();
        let mut command: Command = state_current_sh_cmd.into();
        let output = command
            .stdin(Stdio::null())
            .kill_on_drop(true)
            .output()
            .await
            .map_err(|error| {
                #[cfg(feature = "error_reporting")]
                let sh_cmd_string = format!("{state_current_sh_cmd}");

                ShCmdError::CmdExecFail {
                    sh_cmd: state_current_sh_cmd.clone(),
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
                sh_cmd: state_current_sh_cmd.clone(),
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
                sh_cmd: state_current_sh_cmd.clone(),
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
