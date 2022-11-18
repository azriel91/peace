use std::fmt;

use serde::{Deserialize, Serialize};

use crate::FileMetadata;

/// State of the command execution.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum ShCmdState {
    /// The command has never been executed.
    NotYetExecuted,
    /// Record of the command's last execution.
    Executed {
        /// Last execution time.
        execution_time: chrono::DateTime,
        /// Last execution stdout output.
        stdout: String,
        /// Last execution stderr output.
        stderr: String,
        /// Last execution output.
        exit_code: u32,
    },
}

impl fmt::Display for ShCmdState {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}
