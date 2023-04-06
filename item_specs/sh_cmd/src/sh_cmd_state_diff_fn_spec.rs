use std::marker::PhantomData;

use peace::cfg::State;

use crate::{
    ShCmdData, ShCmdError, ShCmdExecutionRecord, ShCmdExecutor, ShCmdState, ShCmdStateDiff,
};

/// Runs a shell command to obtain the `ShCmd` diff.
#[derive(Debug)]
pub struct ShCmdStateDiffFnSpec<Id>(PhantomData<Id>);

impl<Id> ShCmdStateDiffFnSpec<Id>
where
    Id: Send + Sync + 'static,
{
    pub async fn state_diff(
        data: ShCmdData<'_, Id>,
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
        let state_diff_sh_cmd = data
            .sh_cmd_params()
            .state_diff_sh_cmd()
            .clone()
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
