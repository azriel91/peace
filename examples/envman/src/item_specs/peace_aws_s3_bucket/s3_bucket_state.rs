use std::fmt;

use serde::{Deserialize, Serialize};

/// S3 bucket state.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub enum S3BucketState {
    /// S3 bucket does not exist.
    None,
    /// S3 bucket exists.
    Some {
        /// S3 bucket name.
        ///
        /// Alphanumeric characters and `_+=,.@-` are allowed.
        ///
        /// TODO: newtype + proc macro.
        name: String,
    },
}

impl fmt::Display for S3BucketState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::None => "does not exist".fmt(f),
            Self::Some { name } => write!(f, "{name} exists"),
        }
    }
}
