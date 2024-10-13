use std::marker::PhantomData;

use crate::{ShCmd, ShCmdError, ShCmdExecutor, ShCmdState, ShCmdStateDiff, ShCmdStateLogical};

/// Runs a shell command to obtain the `ShCmd` diff.
#[derive(Debug)]
pub struct ShCmdStateDiffFn<Id>(PhantomData<Id>);

impl<Id> ShCmdStateDiffFn<Id>
where
    Id: Send + Sync + 'static,
{
    pub async fn state_diff(
        state_diff_sh_cmd: ShCmd,
        state_current: &ShCmdState<Id>,
        state_goal: &ShCmdState<Id>,
    ) -> Result<ShCmdStateDiff, ShCmdError> {
        let state_current_arg = match &state_current.0.logical {
            ShCmdStateLogical::None => "",
            ShCmdStateLogical::Some { stdout, .. } => stdout.as_ref(),
        };
        let state_goal_arg = match &state_goal.0.logical {
            ShCmdStateLogical::None => "",
            ShCmdStateLogical::Some { stdout, .. } => stdout.as_ref(),
        };
        let state_diff_sh_cmd = state_diff_sh_cmd.arg(state_current_arg).arg(state_goal_arg);
        ShCmdExecutor::<Id>::exec(&state_diff_sh_cmd)
            .await
            .map(|state| match state.0.logical {
                ShCmdStateLogical::None => ShCmdStateDiff::new(String::from(""), String::from("")),
                ShCmdStateLogical::Some {
                    stdout,
                    stderr,
                    marker: _,
                } => ShCmdStateDiff::new(stdout, stderr),
            })
    }
}
