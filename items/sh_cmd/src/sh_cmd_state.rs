use std::{fmt, marker::PhantomData};

use derivative::Derivative;
use serde::{Deserialize, Serialize};

#[cfg(feature = "output_progress")]
use peace::item_model::ItemLocationState;

/// State of the shell command execution.
///
/// * If the command has never been executed, this will be `None`.
/// * If it has been executed, this is `Some(String)` captured from stdout.
#[derive(Derivative, Serialize, Deserialize, Eq)]
#[derivative(Clone, Debug, PartialEq)]
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

impl<Id> fmt::Display for ShCmdState<Id> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::None => write!(f, "<none>"),
            Self::Some { stderr, .. } => stderr.fmt(f),
        }
    }
}

#[cfg(feature = "output_progress")]
impl<'state> From<&'state ShCmdState> for ItemLocationState {
    fn from(sh_cmd_state: &'state ShCmdState) -> ItemLocationState {
        match sh_cmd_state {
            Some { .. } => ItemLocationState::Exists,
            None => ItemLocationState::NotExists,
        }
    }
}
