use std::fmt;

use chrono::{offset::Local, DateTime, Utc};
use serde::{Deserialize, Serialize};

/// State of the command execution.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum ShCmdState {
    /// The command is not executed.
    ///
    /// Represents when the command has either never been executed, or has been
    /// cleaned up.
    NotExecuted,
    /// Record of the command's last execution.
    Executed {
        /// Timestamp of execution.
        datetime: chrono::DateTime<Utc>,
        /// Duration of execution.
        duration: chrono::DateTime<Utc>,
        /// stdout output.
        stdout: String,
        /// stderr output.
        stderr: String,
        /// Exit code.
        exit_code: u32,
    },
}

impl fmt::Display for ShCmdState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotExecuted => write!(f, "not executed"),
            Self::Executed {
                datetime,
                exit_code,
                ..
            } => {
                if *exit_code == 0 {
                    let datetime_local = DateTime::<Local>::from(*datetime);
                    write!(f, "executed successfully at {datetime_local}")
                } else {
                    write!(f, "execution failed with code: {exit_code}")
                }
            }
        }
    }
}
