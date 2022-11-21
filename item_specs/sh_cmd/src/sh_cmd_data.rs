use peace::{
    data::{Data, RMaybe, R},
    resources::states::StatesPrevious,
};

use crate::ShCmdParams;

/// Data used to run a shell command.
///
/// # Type Parameters
///
/// * `Id`: A zero-sized type used to distinguish different command execution
///   parameters from each other.
#[derive(Data, Debug)]
pub struct ShCmdData<'op, Id>
where
    Id: Send + Sync + 'static,
{
    /// Parameters to determine what shell command to run.
    sh_cmd_params: R<'op, ShCmdParams<Id>>,

    /// Previous states with this item spec's previous execution.
    states_previous: RMaybe<'op, StatesPrevious>,
}

impl<'op, Id> ShCmdData<'op, Id>
where
    Id: Send + Sync + 'static,
{
    /// Returns the parameters to determine what shell command to run.
    pub fn sh_cmd_params(&self) -> &ShCmdParams<Id> {
        &self.sh_cmd_params
    }

    /// Returns the previous states.
    pub fn states_previous(&self) -> Option<&StatesPrevious> {
        self.states_previous.as_deref()
    }
}
