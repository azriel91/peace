use std::marker::PhantomData;

#[cfg(feature = "output_progress")]
use peace::cfg::progress::ProgressLimit;
use peace::cfg::{ApplyCheck, FnCtx, State};

use crate::{
    ShCmd, ShCmdData, ShCmdError, ShCmdExecutionRecord, ShCmdExecutor, ShCmdParams, ShCmdState,
    ShCmdStateDiff,
};

/// ApplyFns for the command to execute.
#[derive(Debug)]
pub struct ShCmdApplyFns<Id>(PhantomData<Id>);

impl<Id> ShCmdApplyFns<Id>
where
    Id: Send + Sync + 'static,
{
    pub async fn apply_check(
        params: &ShCmdParams<Id>,
        _data: ShCmdData<'_, Id>,
        state_current: &State<ShCmdState<Id>, ShCmdExecutionRecord>,
        state_desired: &State<ShCmdState<Id>, ShCmdExecutionRecord>,
        state_diff: &ShCmdStateDiff,
    ) -> Result<ApplyCheck, ShCmdError> {
        let state_current_arg = match &state_current.logical {
            ShCmdState::None => "",
            ShCmdState::Some { stdout, .. } => stdout.as_ref(),
        };
        let state_desired_arg = match &state_desired.logical {
            ShCmdState::None => "",
            ShCmdState::Some { stdout, .. } => stdout.as_ref(),
        };
        let apply_check_sh_cmd = params
            .apply_check_sh_cmd()
            .clone()
            .arg(state_current_arg)
            .arg(state_desired_arg)
            .arg(&**state_diff);

        ShCmdExecutor::<Id>::exec(&apply_check_sh_cmd)
            .await
            .and_then(|state| match state.logical {
                ShCmdState::Some { stdout, .. } => match stdout.trim().lines().next_back() {
                    Some("true") => {
                        #[cfg(not(feature = "output_progress"))]
                        {
                            Ok(ApplyCheck::ExecRequired)
                        }

                        #[cfg(feature = "output_progress")]
                        Ok(ApplyCheck::ExecRequired {
                            progress_limit: ProgressLimit::Unknown,
                        })
                    }
                    Some("false") => Ok(ApplyCheck::ExecNotRequired),
                    _ => Err(ShCmdError::EnsureCheckValueNotBoolean {
                        sh_cmd: apply_check_sh_cmd.clone(),
                        #[cfg(feature = "error_reporting")]
                        sh_cmd_string: format!("{apply_check_sh_cmd}"),
                        stdout: Some(stdout),
                    }),
                },
                _ => Err(ShCmdError::EnsureCheckValueNotBoolean {
                    sh_cmd: apply_check_sh_cmd.clone(),
                    #[cfg(feature = "error_reporting")]
                    sh_cmd_string: format!("{apply_check_sh_cmd}"),
                    stdout: None,
                }),
            })
    }

    pub async fn apply_dry(
        _fn_ctx: FnCtx<'_>,
        params: &ShCmdParams<Id>,
        _data: ShCmdData<'_, Id>,
        state_current: &State<ShCmdState<Id>, ShCmdExecutionRecord>,
        state_desired: &State<ShCmdState<Id>, ShCmdExecutionRecord>,
        state_diff: &ShCmdStateDiff,
    ) -> Result<State<ShCmdState<Id>, ShCmdExecutionRecord>, ShCmdError> {
        // TODO: implement properly
        let state_current_arg = match &state_current.logical {
            ShCmdState::None => "",
            ShCmdState::Some { stdout, .. } => stdout.as_ref(),
        };
        let state_desired_arg = match &state_desired.logical {
            ShCmdState::None => "",
            ShCmdState::Some { stdout, .. } => stdout.as_ref(),
        };
        let apply_exec_sh_cmd = params
            .apply_exec_sh_cmd()
            .clone()
            .arg(state_current_arg)
            .arg(state_desired_arg)
            .arg(&**state_diff);

        ShCmdExecutor::<Id>::exec(&ShCmd::new("echo").arg(format!("{apply_exec_sh_cmd}"))).await
    }

    pub async fn apply(
        _fn_ctx: FnCtx<'_>,
        params: &ShCmdParams<Id>,
        _data: ShCmdData<'_, Id>,
        state_current: &State<ShCmdState<Id>, ShCmdExecutionRecord>,
        state_desired: &State<ShCmdState<Id>, ShCmdExecutionRecord>,
        state_diff: &ShCmdStateDiff,
    ) -> Result<State<ShCmdState<Id>, ShCmdExecutionRecord>, ShCmdError> {
        let state_current_arg = match &state_current.logical {
            ShCmdState::None => "",
            ShCmdState::Some { stdout, .. } => stdout.as_ref(),
        };
        let state_desired_arg = match &state_desired.logical {
            ShCmdState::None => "",
            ShCmdState::Some { stdout, .. } => stdout.as_ref(),
        };
        let apply_exec_sh_cmd = params
            .apply_exec_sh_cmd()
            .clone()
            .arg(state_current_arg)
            .arg(state_desired_arg)
            .arg(&**state_diff);

        ShCmdExecutor::<Id>::exec(&apply_exec_sh_cmd).await?;
        ShCmdExecutor::<Id>::exec(params.state_current_sh_cmd()).await
    }
}
