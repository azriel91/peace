use std::fmt;

use serde::{Deserialize, Serialize};

/// Diff between current (dest) and goal (src) state.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub enum S3ObjectStateDiff {
    /// S3 object would be added.
    Added,
    /// S3 object would be removed.
    Removed,
    /// S3 bucket for the object renamed.
    ///
    /// We could do an AWS `copy_object` then remove the current.
    BucketNameModified {
        /// Current bucket name.
        bucket_name_current: String,
        /// Goal bucket name.
        bucket_name_goal: String,
    },
    /// S3 object renamed.
    ///
    /// We could do an AWS `copy_object` then remove the current if the content
    /// hasn't changed.
    ObjectKeyModified {
        /// Current object key.
        object_key_current: String,
        /// Goal object key.
        object_key_goal: String,
    },
    /// S3 object content has changed.
    ObjectContentModified {
        /// Current MD5 hex string of object content.
        content_md5_hexstr_current: Option<String>,
        /// Goal MD5 hex string of object content.
        content_md5_hexstr_goal: Option<String>,
    },
    /// S3 object exists and is up to date.
    InSyncExists,
    /// S3 object does not exist, which is goal.
    InSyncDoesNotExist,
}

impl fmt::Display for S3ObjectStateDiff {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            S3ObjectStateDiff::Added => {
                write!(f, "will be created.")
            }
            S3ObjectStateDiff::Removed => {
                write!(f, "will be removed.")
            }
            S3ObjectStateDiff::BucketNameModified {
                bucket_name_current,
                bucket_name_goal,
            } => write!(
                f,
                "bucket name has changed from {bucket_name_current} to {bucket_name_goal}"
            ),
            S3ObjectStateDiff::ObjectKeyModified {
                object_key_current,
                object_key_goal,
            } => write!(
                f,
                "object key has changed from {object_key_current} to {object_key_goal}"
            ),
            S3ObjectStateDiff::ObjectContentModified {
                content_md5_hexstr_current,
                content_md5_hexstr_goal,
            } => {
                let content_md5_hexstr_current =
                    content_md5_hexstr_current.as_deref().unwrap_or("<none>");
                let content_md5_hexstr_goal =
                    content_md5_hexstr_goal.as_deref().unwrap_or("<none>");

                write!(
                    f,
                    "object content has changed from {content_md5_hexstr_current} to {content_md5_hexstr_goal}"
                )
            }
            S3ObjectStateDiff::InSyncExists => {
                write!(f, "exists and is up to date.")
            }
            S3ObjectStateDiff::InSyncDoesNotExist => {
                write!(f, "does not exist as intended.")
            }
        }
    }
}
