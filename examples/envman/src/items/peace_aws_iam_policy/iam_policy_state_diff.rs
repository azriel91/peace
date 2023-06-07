use std::fmt;

use serde::{Deserialize, Serialize};

/// Diff between current (dest) and goal (src) state.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub enum IamPolicyStateDiff {
    /// Policy would be added.
    Added,
    /// Policy would be removed.
    Removed,
    /// Policy would be replaced.
    ///
    /// AWS' SDK doesn't support modifying a policy's name or path.
    NameOrPathModified {
        /// Whether the name has been changed.
        name_diff: Option<(String, String)>,
        /// Whether the path has been changed.
        path_diff: Option<(String, String)>,
    },
    /// The policy has been modified.
    DocumentModified {
        /// Current policy document.
        document_current: String,
        /// Goal policy document.
        document_goal: String,
    },
    /// Policy exists and is up to date.
    InSyncExists,
    /// Policy does not exist, which is goal.
    InSyncDoesNotExist,
}

impl fmt::Display for IamPolicyStateDiff {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IamPolicyStateDiff::Added => {
                write!(f, "will be created.")
            }
            IamPolicyStateDiff::Removed => {
                write!(f, "will be removed.")
            }
            IamPolicyStateDiff::DocumentModified {
                document_current: _,
                document_goal: _,
            } => write!(f, "policy will be updated."),
            IamPolicyStateDiff::NameOrPathModified {
                name_diff,
                path_diff,
            } => match (name_diff, path_diff) {
                (None, None) => {
                    unreachable!(
                        "Modified is only valid when either name or path has changed.\n\
                        This is a bug."
                    )
                }
                (None, Some((path_current, path_goal))) => {
                    write!(f, "path changed from {path_current} to {path_goal}")
                }
                (Some((name_current, name_goal)), None) => {
                    write!(f, "name changed from {name_current} to {name_goal}")
                }
                (Some((name_current, name_goal)), Some((path_current, path_goal))) => write!(
                    f,
                    "name and path changed from {name_current}:{path_current} to {name_goal}:{path_goal}"
                ),
            },
            IamPolicyStateDiff::InSyncExists => {
                write!(f, "exists and is up to date.")
            }
            IamPolicyStateDiff::InSyncDoesNotExist => {
                write!(f, "does not exist as intended.")
            }
        }
    }
}
