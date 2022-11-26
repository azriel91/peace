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
        /// Timestamp of beginning of execution.
        start_datetime: chrono::DateTime<Utc>,
        /// Timestamp that the execution ended.
        end_datetime: chrono::DateTime<Utc>,
        /// stdout output.
        stdout: String,
        /// stderr output.
        stderr: String,
        /// Exit code of the process, if any.
        ///
        /// See [`ExitStatus::code()`].
        ///
        /// [`ExitStatus::code()`]: std::process::ExitStatus::code
        exit_code: Option<i32>,
    },
}

impl fmt::Display for ShSyncCmdExecutionRecord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::None => write!(f, "not executed"),
            Self::Some {
                start_datetime,
                exit_code,
                ..
            } => match exit_code {
                Some(0) => {
                    let start_datetime_local = DateTime::<Local>::from(*start_datetime);
                    write!(f, "executed successfully at {start_datetime_local}")
                }
                Some(code) => write!(f, "execution failed with code: {code}"),
                None => write!(f, "execution was interrupted"),
            },
        }
    }
}
