use std::fmt;

use serde::{Deserialize, Serialize};

/// Diff between current (dest) and desired (src) state.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub enum BlankStateDiff {
    /// Value was added.
    Added {
        /// The new value.
        value: u32,
    },
    /// Value
    OutOfSync {
        /// Difference between the current and desired values.
        diff: i64,
    },
    InSync {
        /// The current value.
        value: u32,
    },
}

impl fmt::Display for BlankStateDiff {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BlankStateDiff::Added { value } => write!(f, "`{value}` newly added."),
            BlankStateDiff::OutOfSync { diff } => {
                write!(f, "Current value differs to desired value by: `{diff}`.")
            }
            BlankStateDiff::InSync { value } => write!(f, "Value already in sync: `{value}`."),
        }
    }
}
