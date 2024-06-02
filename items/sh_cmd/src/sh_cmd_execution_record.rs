use std::fmt;

use chrono::{offset::Local, DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Record of a shell command execution.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ShCmdExecutionRecord {
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
        /// Exit code of the process, if any.
        ///
        /// See [`ExitStatus::code()`].
        ///
        /// [`ExitStatus::code()`]: std::process::ExitStatus::code
        exit_code: Option<i32>,
    },
}

impl fmt::Display for ShCmdExecutionRecord {
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

/// We don't compare timestamps of execution because we're concerned about
/// state of whatever the shell command is reading, rather than when the shell
/// command was run.
impl PartialEq for ShCmdExecutionRecord {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (ShCmdExecutionRecord::None, ShCmdExecutionRecord::None) => true,
            (ShCmdExecutionRecord::None, ShCmdExecutionRecord::Some { .. })
            | (ShCmdExecutionRecord::Some { .. }, ShCmdExecutionRecord::None) => false,
            (
                ShCmdExecutionRecord::Some {
                    start_datetime: _,
                    end_datetime: _,
                    exit_code: exit_code_self,
                },
                ShCmdExecutionRecord::Some {
                    start_datetime: _,
                    end_datetime: _,
                    exit_code: exit_code_other,
                },
            ) => exit_code_self == exit_code_other,
        }
    }
}
