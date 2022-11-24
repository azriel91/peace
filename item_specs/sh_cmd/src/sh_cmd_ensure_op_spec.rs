use std::marker::PhantomData;

use peace::cfg::{async_trait, EnsureOpSpec, OpCheckStatus, ProgressLimit, State};

use crate::{
    ShCmdData, ShCmdError, ShCmdExecutionRecord, ShCmdExecutor, ShCmdState, ShCmdStateDiff,
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
    type StateLogical = ShCmdState;
    type StatePhysical = ShCmdExecutionRecord;

    async fn check(
        sh_cmd_data: ShCmdData<'_, Id>,
        state_current: &State<ShCmdState, ShCmdExecutionRecord>,
        state_desired: &ShCmdState,
        state_diff: &ShCmdStateDiff,
    ) -> Result<OpCheckStatus, ShCmdError> {
        let mut ensure_check_sh_cmd = sh_cmd_data.sh_cmd_params().ensure_check_sh_cmd().clone();

        let state_current_arg = match &state_current.logical {
            ShCmdState::None => "",
            ShCmdState::Some(s) => s.as_ref(),
        };
        let state_desired_arg = match state_desired {
            ShCmdState::None => "",
            ShCmdState::Some(s) => s.as_ref(),
        };
        ensure_check_sh_cmd
            .arg(state_current_arg)
            .arg(state_desired_arg)
            .arg(&**state_diff);

        ShCmdExecutor::exec(&ensure_check_sh_cmd)
            .await
            .and_then(|state| match state.logical {
                ShCmdState::Some(stdout) => match stdout.trim().lines().rev().next() {
                    Some("true") => Ok(OpCheckStatus::ExecRequired {
                        progress_limit: ProgressLimit::Unknown,
                    }),
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
        _sh_cmd_data: ShCmdData<'_, Id>,
        _state: &State<ShCmdState, ShCmdExecutionRecord>,
        _file_state_desired: &ShCmdState,
        _diff: &ShCmdStateDiff,
    ) -> Result<ShCmdExecutionRecord, ShCmdError> {
        todo!()
    }

    async fn exec(
        _sh_cmd_data: ShCmdData<'_, Id>,
        _state: &State<ShCmdState, ShCmdExecutionRecord>,
        _file_state_desired: &ShCmdState,
        _diff: &ShCmdStateDiff,
    ) -> Result<ShCmdExecutionRecord, ShCmdError> {
        todo!();
    }
}
