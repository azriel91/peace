use std::marker::PhantomData;

use peace::cfg::State;

use crate::{
    ShCmd, ShCmdError, ShCmdExecutionRecord, ShCmdExecutor, ShCmdStateDiff, ShCmdStatePhysical,
};

/// Runs a shell command to obtain the `ShCmd` diff.
#[derive(Debug)]
pub struct ShCmdStateDiffFn<Id>(PhantomData<Id>);

impl<Id> ShCmdStateDiffFn<Id>
where
    Id: Send + Sync + 'static,
{
    pub async fn state_diff(
        state_diff_sh_cmd: ShCmd,
        state_current: &State<ShCmdStatePhysical<Id>, ShCmdExecutionRecord>,
        state_goal: &State<ShCmdStatePhysical<Id>, ShCmdExecutionRecord>,
    ) -> Result<ShCmdStateDiff, ShCmdError> {
        let state_current_arg = match &state_current.logical {
            ShCmdStatePhysical::None => "",
            ShCmdStatePhysical::Some { stdout, .. } => stdout.as_ref(),
        };
        let state_goal_arg = match &state_goal.logical {
            ShCmdStatePhysical::None => "",
            ShCmdStatePhysical::Some { stdout, .. } => stdout.as_ref(),
        };
        let state_diff_sh_cmd = state_diff_sh_cmd.arg(state_current_arg).arg(state_goal_arg);
        ShCmdExecutor::<Id>::exec(&state_diff_sh_cmd)
            .await
            .map(|state| match state.logical {
                ShCmdStatePhysical::None => ShCmdStateDiff::new(String::from(""), String::from("")),
                ShCmdStatePhysical::Some {
                    stdout,
                    stderr,
                    marker: _,
                } => ShCmdStateDiff::new(stdout, stderr),
            })
    }
}
