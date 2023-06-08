use std::fmt;

use serde::{Deserialize, Serialize};

/// Diff between current (dest) and goal (src) state.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub enum InstanceProfileStateDiff {
    /// InstanceProfile would be added.
    Added,
    /// InstanceProfile would be removed.
    Removed,
    /// InstanceProfile would be replaced.
    ///
    /// AWS' SDK doesn't support modifying a instance profile's name or path.
    NameOrPathModified {
        /// Whether the name has been changed.
        name_diff: Option<(String, String)>,
        /// Whether the path has been changed.
        path_diff: Option<(String, String)>,
    },
    /// The instance profile role association has been modified.
    RoleAssociatedModified {
        /// Current instance profile role association.
        role_associated_current: bool,
        /// Goal instance profile role association.
        role_associated_goal: bool,
    },
    /// InstanceProfile exists and is up to date.
    InSyncExists,
    /// InstanceProfile does not exist, which is goal.
    InSyncDoesNotExist,
}

impl fmt::Display for InstanceProfileStateDiff {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InstanceProfileStateDiff::Added => {
                write!(f, "will be created.")
            }
            InstanceProfileStateDiff::Removed => {
                write!(f, "will be removed.")
            }
            InstanceProfileStateDiff::RoleAssociatedModified {
                role_associated_current,
                role_associated_goal,
            } => {
                if !role_associated_current && *role_associated_goal {
                    write!(f, "role will be disassociated from instance profile.")
                } else {
                    write!(f, "role will be associated with instance profile.")
                }
            }
            InstanceProfileStateDiff::NameOrPathModified {
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
            InstanceProfileStateDiff::InSyncExists => {
                write!(f, "exists and is up to date.")
            }
            InstanceProfileStateDiff::InSyncDoesNotExist => {
                write!(f, "does not exist as intended.")
            }
        }
    }
}
