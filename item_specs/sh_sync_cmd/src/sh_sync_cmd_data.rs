use peace::{
    data::{
        accessors::{RMaybe, R},
        Data,
    },
    resources::states::StatesSaved,
};

use crate::ShSyncCmdParams;

/// Data used to run a shell command.
///
/// # Type Parameters
///
/// * `Id`: A zero-sized type used to distinguish different command execution
///   parameters from each other.
#[derive(Data, Debug)]
pub struct ShSyncCmdData<'exec, Id>
where
    Id: Send + Sync + 'static,
{
    /// Parameters to determine what shell command to run.
    sh_sync_cmd_params: R<'exec, ShSyncCmdParams<Id>>,

    /// Saved states with this item spec's previous execution.
    states_saved: RMaybe<'exec, StatesSaved>,
}

impl<'exec, Id> ShSyncCmdData<'exec, Id>
where
    Id: Send + Sync + 'static,
{
    /// Returns the parameters to determine what shell command to run.
    pub fn sh_sync_cmd_params(&self) -> &ShSyncCmdParams<Id> {
        &self.sh_sync_cmd_params
    }

    /// Returns the previous states.
    pub fn states_saved(&self) -> Option<&StatesSaved> {
        self.states_saved.as_deref()
    }
}
