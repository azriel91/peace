use peace::{
    data::{Data, RMaybe, R},
    resources::states::StatesPrevious,
};

use crate::ShSyncCmdParams;

/// Data used to run a shell command.
///
/// # Type Parameters
///
/// * `Id`: A zero-sized type used to distinguish different command execution
///   parameters from each other.
#[derive(Data, Debug)]
pub struct ShSyncCmdData<'op, Id>
where
    Id: Send + Sync + 'static,
{
    /// Parameters to determine what shell command to run.
    sh_sync_cmd_params: R<'op, ShSyncCmdParams<Id>>,

    /// Previous states with this item spec's previous execution.
    states_previous: RMaybe<'op, StatesPrevious>,
}

impl<'op, Id> ShSyncCmdData<'op, Id>
where
    Id: Send + Sync + 'static,
{
    /// Returns the parameters to determine what shell command to run.
    pub fn sh_sync_cmd_params(&self) -> &ShSyncCmdParams<Id> {
        &self.sh_sync_cmd_params
    }

    /// Returns the previous states.
    pub fn states_previous(&self) -> Option<&StatesPrevious> {
        self.states_previous.as_deref()
    }
}