use std::marker::PhantomData;

#[cfg(feature = "output_progress")]
use peace::cfg::progress::ProgressLimit;
use peace::cfg::{async_trait, CleanOpSpec, OpCheckStatus, State};

use crate::{ShCmdData, ShCmdError, ShCmdExecutionRecord, ShCmdExecutor, ShCmdState};

/// `CleanOpSpec` for the command to execute.
#[derive(Debug, Default)]
pub struct ShCmdCleanOpSpec<Id>(PhantomData<Id>);

#[async_trait(?Send)]
impl<Id> CleanOpSpec for ShCmdCleanOpSpec<Id>
where
    Id: Send + Sync + 'static,
{
    type Data<'op> = ShCmdData<'op, Id>;
    type Error = ShCmdError;
    type StateLogical = ShCmdState<Id>;
    type StatePhysical = ShCmdExecutionRecord;

    async fn check(
        sh_cmd_data: ShCmdData<'_, Id>,
        state_current: &State<ShCmdState<Id>, ShCmdExecutionRecord>,
    ) -> Result<OpCheckStatus, ShCmdError> {
        let state_current_arg = match &state_current.logical {
            ShCmdState::None => "",
            ShCmdState::Some { stdout, .. } => stdout.as_ref(),
        };
        let clean_check_sh_cmd = sh_cmd_data
            .sh_cmd_params()
            .clean_check_sh_cmd()
            .clone()
            .arg(state_current_arg);

        ShCmdExecutor::<Id>::exec(&clean_check_sh_cmd)
            .await
            .and_then(|state| match state.logical {
                ShCmdState::Some { stdout, .. } => match stdout.trim().lines().rev().next() {
                    Some("true") => {
                        #[cfg(not(feature = "output_progress"))]
                        {
                            Ok(OpCheckStatus::ExecRequired)
                        }

                        #[cfg(feature = "output_progress")]
                        Ok(OpCheckStatus::ExecRequired {
                            progress_limit: ProgressLimit::Unknown,
                        })
                    }
                    Some("false") => Ok(OpCheckStatus::ExecNotRequired),
                    _ => Err(ShCmdError::CleanCheckValueNotBoolean {
                        sh_cmd: clean_check_sh_cmd.clone(),
                        #[cfg(feature = "error_reporting")]
                        sh_cmd_string: format!("{clean_check_sh_cmd}"),
                        stdout: Some(stdout),
                    }),
                },
                _ => Err(ShCmdError::CleanCheckValueNotBoolean {
                    sh_cmd: clean_check_sh_cmd.clone(),
                    #[cfg(feature = "error_reporting")]
                    sh_cmd_string: format!("{clean_check_sh_cmd}"),
                    stdout: None,
                }),
            })
    }

    async fn exec_dry(
        _sh_cmd_data: ShCmdData<'_, Id>,
        _state: &State<ShCmdState<Id>, ShCmdExecutionRecord>,
    ) -> Result<(), ShCmdError> {
        Ok(())
    }

    #[cfg(not(target_arch = "wasm32"))]
    async fn exec(
        sh_cmd_data: ShCmdData<'_, Id>,
        state_current: &State<ShCmdState<Id>, ShCmdExecutionRecord>,
    ) -> Result<(), ShCmdError> {
        let state_current_arg = match &state_current.logical {
            ShCmdState::None => "",
            ShCmdState::Some { stdout, .. } => stdout.as_ref(),
        };
        let clean_exec_sh_cmd = sh_cmd_data
            .sh_cmd_params()
            .clean_exec_sh_cmd()
            .clone()
            .arg(state_current_arg);

        ShCmdExecutor::<Id>::exec(&clean_exec_sh_cmd)
            .await
            .map(|_state| ())
    }

    #[cfg(target_arch = "wasm32")]
    async fn exec(
        sh_cmd_data: ShCmdData<'_, Id>,
        State {
            logical: file_state,
            ..
        }: &State<ShCmdState<Id>, ShCmdExecutionRecord>,
    ) -> Result<(), ShCmdError> {
        todo!()
    }
}
