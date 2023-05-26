use std::fmt;

use serde::{Deserialize, Serialize};

/// Diff between current (dest) and desired (src) state.
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
        /// Desired instance profile role association.
        role_associated_desired: bool,
    },
    /// InstanceProfile exists and is up to date.
    InSyncExists,
    /// InstanceProfile does not exist, which is desired.
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
                role_associated_desired,
            } => {
                if !role_associated_current && *role_associated_desired {
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
                (None, Some((path_current, path_desired))) => {
                    write!(f, "path changed from {path_current} to {path_desired}")
                }
                (Some((name_current, name_desired)), None) => {
                    write!(f, "name changed from {name_current} to {name_desired}")
                }
                (Some((name_current, name_desired)), Some((path_current, path_desired))) => write!(
                    f,
                    "name and path changed from {name_current}:{path_current} to {name_desired}:{path_desired}"
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
