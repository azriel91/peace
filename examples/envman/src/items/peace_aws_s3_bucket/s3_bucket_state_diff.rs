use std::fmt;

use serde::{Deserialize, Serialize};

/// Diff between current (dest) and desired (src) state.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub enum S3BucketStateDiff {
    /// S3Bucket would be added.
    Added,
    /// S3Bucket would be removed.
    Removed,
    /// S3Bucket renamed.
    ///
    /// AWS' SDK doesn't support modifying an S3 bucket's name.
    NameModified {
        /// Current bucket name.
        s3_bucket_name_current: String,
        /// Desired bucket name.
        s3_bucket_name_desired: String,
    },
    /// S3Bucket exists and is up to date.
    InSyncExists,
    /// S3Bucket does not exist, which is desired.
    InSyncDoesNotExist,
}

impl fmt::Display for S3BucketStateDiff {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            S3BucketStateDiff::Added => {
                write!(f, "will be created.")
            }
            S3BucketStateDiff::Removed => {
                write!(f, "will be removed.")
            }
            S3BucketStateDiff::NameModified {
                s3_bucket_name_current,
                s3_bucket_name_desired,
            } => write!(
                f,
                "name has changed from {s3_bucket_name_current} to {s3_bucket_name_desired}"
            ),
            S3BucketStateDiff::InSyncExists => {
                write!(f, "exists and is up to date.")
            }
            S3BucketStateDiff::InSyncDoesNotExist => {
                write!(f, "does not exist as intended.")
            }
        }
    }
}
