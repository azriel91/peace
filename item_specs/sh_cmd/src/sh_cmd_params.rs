use std::{fmt, marker::PhantomData};

use serde::{Deserialize, Serialize};

use crate::ShCmd;

/// Grouping of commands to run a shell command idempotently.
///
/// The `Id` type parameter is needed for each command execution params to be a
/// distinct type.
///
/// # Type Parameters
///
/// * `Id`: A zero-sized type used to distinguish different command execution
///   parameters from each other.
#[derive(Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct ShCmdParams<Id> {
    /// Shell command to run that does the work.
    work_sh_command: ShCmd,
    /// Marker for unique command execution parameters type.
    marker: PhantomData<Id>,
}

impl<Id> ShCmdParams<Id> {
    /// Returns new `ShCmdParams`.
    pub fn new(work_sh_command: ShCmd) -> Self {
        Self {
            work_sh_command,
            marker: PhantomData,
        }
    }

    /// Returns the shell command that does the work.
    pub fn work_sh_command(&self) -> &ShCmd {
        &self.work_sh_command
    }
}

impl<Id> fmt::Debug for ShCmdParams<Id> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ShCmdParams")
            .field("work_sh_command", &self.work_sh_command)
            .finish()
    }
}
