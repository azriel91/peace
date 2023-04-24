use std::{fmt, marker::PhantomData};

use peace::params::Params;
use serde::{Deserialize, Serialize};

use crate::ShSyncCmd;

/// Grouping of commands to run a shell command idempotently.
///
/// The `Id` type parameter is needed for each command execution params to be a
/// distinct type.
///
/// # Type Parameters
///
/// * `Id`: A zero-sized type used to distinguish different command execution
///   parameters from each other.
#[derive(Params, PartialEq, Eq, Deserialize, Serialize)]
pub struct ShSyncCmdParams<Id> {
    /// Shell command to run that does the work.
    work_sh_command: ShSyncCmd,
    /// Marker for unique command execution parameters type.
    marker: PhantomData<Id>,
}

impl<Id> Clone for ShSyncCmdParams<Id> {
    fn clone(&self) -> Self {
        Self {
            work_sh_command: self.work_sh_command.clone(),
            marker: PhantomData,
        }
    }
}

impl<Id> ShSyncCmdParams<Id> {
    /// Returns new `ShSyncCmdParams`.
    pub fn new(work_sh_command: ShSyncCmd) -> Self {
        Self {
            work_sh_command,
            marker: PhantomData,
        }
    }

    /// Returns the shell command that does the work.
    pub fn work_sh_command(&self) -> &ShSyncCmd {
        &self.work_sh_command
    }
}

impl<Id> fmt::Debug for ShSyncCmdParams<Id> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ShSyncCmdParams")
            .field("work_sh_command", &self.work_sh_command)
            .finish()
    }
}
