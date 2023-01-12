use std::marker::PhantomData;

#[cfg(feature = "output_progress")]
use peace::cfg::progress::ProgressLimit;
use peace::cfg::{async_trait, EnsureOpSpec, OpCheckStatus, OpCtx, State};

use crate::{
    ShCmd, ShCmdData, ShCmdError, ShCmdExecutionRecord, ShCmdExecutor, ShCmdState, ShCmdStateDiff,
};

/// Ensure OpSpec for the command to execute.
#[derive(Debug)]
pub struct ShCmdEnsureOpSpec<Id>(PhantomData<Id>);

#[async_trait(?Send)]
impl<Id> EnsureOpSpec for ShCmdEnsureOpSpec<Id>
where
    Id: Send + Sync + 'static,
{
    type Data<'op> = ShCmdData<'op, Id>;
    type Error = ShCmdError;
    type StateDiff = ShCmdStateDiff;
    type StateLogical = ShCmdState<Id>;
    type StatePhysical = ShCmdExecutionRecord;

    async fn check(
        sh_cmd_data: ShCmdData<'_, Id>,
        state_current: &State<ShCmdState<Id>, ShCmdExecutionRecord>,
        state_desired: &State<ShCmdState<Id>, ShCmdExecutionRecord>,
        state_diff: &ShCmdStateDiff,
    ) -> Result<OpCheckStatus, ShCmdError> {
        let state_current_arg = match &state_current.logical {
            ShCmdState::None => "",
            ShCmdState::Some { stdout, .. } => stdout.as_ref(),
        };
        let state_desired_arg = match &state_desired.logical {
            ShCmdState::None => "",
            ShCmdState::Some { stdout, .. } => stdout.as_ref(),
        };
        let ensure_check_sh_cmd = sh_cmd_data
            .sh_cmd_params()
            .ensure_check_sh_cmd()
            .clone()
            .arg(state_current_arg)
            .arg(state_desired_arg)
            .arg(&**state_diff);

        ShCmdExecutor::<Id>::exec(&ensure_check_sh_cmd)
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
                    _ => Err(ShCmdError::EnsureCheckValueNotBoolean {
                        sh_cmd: ensure_check_sh_cmd.clone(),
                        #[cfg(feature = "error_reporting")]
                        sh_cmd_string: format!("{ensure_check_sh_cmd}"),
                        stdout: Some(stdout),
                    }),
                },
                _ => Err(ShCmdError::EnsureCheckValueNotBoolean {
                    sh_cmd: ensure_check_sh_cmd.clone(),
                    #[cfg(feature = "error_reporting")]
                    sh_cmd_string: format!("{ensure_check_sh_cmd}"),
                    stdout: None,
                }),
            })
    }

    async fn exec_dry(
        _op_ctx: OpCtx<'_>,
        sh_cmd_data: ShCmdData<'_, Id>,
        state_current: &State<ShCmdState<Id>, ShCmdExecutionRecord>,
        state_desired: &State<ShCmdState<Id>, ShCmdExecutionRecord>,
        state_diff: &ShCmdStateDiff,
    ) -> Result<ShCmdExecutionRecord, ShCmdError> {
        // TODO: implement properly
        let state_current_arg = match &state_current.logical {
            ShCmdState::None => "",
            ShCmdState::Some { stdout, .. } => stdout.as_ref(),
        };
        let state_desired_arg = match &state_desired.logical {
            ShCmdState::None => "",
            ShCmdState::Some { stdout, .. } => stdout.as_ref(),
        };
        let ensure_exec_sh_cmd = sh_cmd_data
            .sh_cmd_params()
            .ensure_exec_sh_cmd()
            .clone()
            .arg(state_current_arg)
            .arg(state_desired_arg)
            .arg(&**state_diff);

        ShCmdExecutor::<Id>::exec(&ShCmd::new("echo").arg(format!("{ensure_exec_sh_cmd}")))
            .await
            .map(|state| state.physical)
    }

    async fn exec(
        _op_ctx: OpCtx<'_>,
        sh_cmd_data: ShCmdData<'_, Id>,
        state_current: &State<ShCmdState<Id>, ShCmdExecutionRecord>,
        state_desired: &State<ShCmdState<Id>, ShCmdExecutionRecord>,
        state_diff: &ShCmdStateDiff,
    ) -> Result<ShCmdExecutionRecord, ShCmdError> {
        let state_current_arg = match &state_current.logical {
            ShCmdState::None => "",
            ShCmdState::Some { stdout, .. } => stdout.as_ref(),
        };
        let state_desired_arg = match &state_desired.logical {
            ShCmdState::None => "",
            ShCmdState::Some { stdout, .. } => stdout.as_ref(),
        };
        let ensure_exec_sh_cmd = sh_cmd_data
            .sh_cmd_params()
            .ensure_exec_sh_cmd()
            .clone()
            .arg(state_current_arg)
            .arg(state_desired_arg)
            .arg(&**state_diff);

        ShCmdExecutor::<Id>::exec(&ensure_exec_sh_cmd)
            .await
            .map(|state| state.physical)
    }
}
