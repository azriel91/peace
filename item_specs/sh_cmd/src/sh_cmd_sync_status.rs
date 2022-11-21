use std::fmt;

use serde::{Deserialize, Serialize};

/// State of the command execution.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum ShCmdSyncStatus {
    /// The command is not executed.
    ///
    /// Represents when the command has either never been executed, or has been
    /// cleaned up.
    NotExecuted,
    /// Command has not been executed since the source files have been updated.
    ExecutionOutOfDate,
    /// Command has been executed since the source files have been updated.
    ExecutionUpToDate,
}

impl fmt::Display for ShCmdSyncStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotExecuted => write!(f, "not executed"),
            Self::ExecutionOutOfDate => write!(f, "out of date"),
            Self::ExecutionUpToDate => write!(f, "up to date"),
        }
    }
}
