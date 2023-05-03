use std::marker::PhantomData;

use peace::cfg::State;

use crate::{ShCmd, ShCmdError, ShCmdExecutionRecord, ShCmdExecutor, ShCmdState, ShCmdStateDiff};

/// Runs a shell command to obtain the `ShCmd` diff.
#[derive(Debug)]
pub struct ShCmdStateDiffFn<Id>(PhantomData<Id>);

impl<Id> ShCmdStateDiffFn<Id>
where
    Id: Send + Sync + 'static,
{
    pub async fn state_diff(
        state_diff_sh_cmd: ShCmd,
        state_current: &State<ShCmdState<Id>, ShCmdExecutionRecord>,
        state_desired: &State<ShCmdState<Id>, ShCmdExecutionRecord>,
    ) -> Result<ShCmdStateDiff, ShCmdError> {
        let state_current_arg = match &state_current.logical {
            ShCmdState::None => "",
            ShCmdState::Some { stdout, .. } => stdout.as_ref(),
        };
        let state_desired_arg = match &state_desired.logical {
            ShCmdState::None => "",
            ShCmdState::Some { stdout, .. } => stdout.as_ref(),
        };
        let state_diff_sh_cmd = state_diff_sh_cmd
            .arg(state_current_arg)
            .arg(state_desired_arg);
        ShCmdExecutor::<Id>::exec(&state_diff_sh_cmd)
            .await
            .map(|state| match state.logical {
                ShCmdState::None => ShCmdStateDiff::new(String::from(""), String::from("")),
                ShCmdState::Some {
                    stdout,
                    stderr,
                    marker: _,
                } => ShCmdStateDiff::new(stdout, stderr),
            })
    }
}
