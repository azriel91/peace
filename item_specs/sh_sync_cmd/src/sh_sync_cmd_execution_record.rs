use std::fmt;

use chrono::{offset::Local, DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Record of a shell command execution.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum ShSyncCmdExecutionRecord {
    /// There is no execution record.
    ///
    /// Represents when the command has either never been executed, or has been
    /// cleaned up.
    None,
    /// Record of the command's last execution.
    Some {
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

impl fmt::Display for ShSyncCmdExecutionRecord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::None => write!(f, "not executed"),
            Self::Some {
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
