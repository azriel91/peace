use std::{fmt, marker::PhantomData};

use serde::{Deserialize, Serialize};

/// State of the shell command execution.
///
/// * If the command has never been executed, this will be `None`.
/// * If it has been executed, this is `Some(String)` captured from stdout.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum ShCmdState<Id> {
    /// The command is not executed.
    ///
    /// Represents when the command has either never been executed, or has been
    /// cleaned up.
    None,
    /// Command has not been executed since the source files have been updated.
    Some {
        /// stdout output.
        stdout: String,
        /// stderr output.
        stderr: String,
        /// Marker.
        marker: PhantomData<Id>,
    },
}

// Manual implementation to avoid `Id: Clone` bound.
impl<Id> std::clone::Clone for ShCmdState<Id> {
    fn clone(&self) -> Self {
        match self {
            Self::None => Self::None,
            Self::Some {
                stdout,
                stderr,
                marker: _,
            } => Self::Some {
                stdout: stdout.clone(),
                stderr: stderr.clone(),
                marker: PhantomData,
            },
        }
    }
}

impl<Id> fmt::Display for ShCmdState<Id> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::None => write!(f, "<none>"),
            Self::Some { stderr, .. } => stderr.fmt(f),
        }
    }
}
