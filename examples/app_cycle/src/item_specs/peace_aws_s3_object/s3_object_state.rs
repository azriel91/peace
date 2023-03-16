use std::fmt;

use peace::cfg::state::Generated;
use serde::{Deserialize, Serialize};

/// S3 object state.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub enum S3ObjectState {
    /// S3 object does not exist.
    None,
    /// S3 object exists.
    Some {
        /// S3 bucket to insert the object into,
        bucket_name: String,
        /// S3 object key.
        object_key: String,
        /// MD5 hex string of the content.
        content_md5_hexstr: Option<String>,
        /// ETag served by S3.
        e_tag: Generated<String>,
    },
}

impl fmt::Display for S3ObjectState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::None => "does not exist".fmt(f),
            Self::Some {
                bucket_name,
                object_key,
                content_md5_hexstr,
                e_tag: _,
            } => {
                if let Some(content_md5_hexstr) = content_md5_hexstr {
                    write!(
                        f,
                        "{bucket_name}/{object_key} with MD5 sum: {content_md5_hexstr}"
                    )
                } else {
                    write!(f, "{bucket_name}/{object_key} (MD5 unknown)")
                }
            }
        }
    }
}
