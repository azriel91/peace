use std::fmt;

use serde::{Deserialize, Serialize};

/// Diff between current (dest) and desired (src) state.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub enum IamRoleStateDiff {
    /// Role would be added.
    Added,
    /// Role would be removed.
    Removed,
    /// Role would be replaced.
    ///
    /// AWS doesn't support modifying a role in place, so this item spec must be
    /// cleaned and re-created.
    Modified {
        /// Whether the name has been changed.
        name_diff: Option<(String, String)>,
        /// Whether the path has been changed.
        path_diff: Option<(String, String)>,
    },
    /// Role exists and is up to date.
    InSyncExists,
    /// Role does not exist, which is desired.
    InSyncDoesNotExist,
}

impl fmt::Display for IamRoleStateDiff {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IamRoleStateDiff::Added => {
                write!(f, "will be created.")
            }
            IamRoleStateDiff::Removed => {
                write!(f, "will be removed.")
            }
            IamRoleStateDiff::Modified {
                name_diff,
                path_diff,
            } => match (name_diff, path_diff) {
                (None, None) => {
                    unreachable!(
                        "Modified is only valid when either name or path has changed.\n\
                        This is a bug."
                    )
                }
                (None, Some(_)) => todo!(),
                (Some(_), None) => todo!(),
                (Some(_), Some(_)) => todo!(),
            },
            IamRoleStateDiff::InSyncExists => {
                write!(f, "exists and is up to date.")
            }
            IamRoleStateDiff::InSyncDoesNotExist => {
                write!(f, "does not exist as intended.")
            }
        }
    }
}
