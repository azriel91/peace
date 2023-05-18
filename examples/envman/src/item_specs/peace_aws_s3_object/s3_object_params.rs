use std::{
    marker::PhantomData,
    path::{Path, PathBuf},
};

use derivative::Derivative;
use peace::params::Params;
use serde::{Deserialize, Serialize};

/// S3Object item parameters.
///
/// The `Id` type parameter is needed for each S3 object params to be a
/// distinct type.
///
/// # Type Parameters
///
/// * `Id`: A zero-sized type used to distinguish different S3 object parameters
///   from each other.
#[derive(Derivative, Params, PartialEq, Eq, Deserialize, Serialize)]
#[derivative(Clone, Debug)]
#[serde(bound = "")]
pub struct S3ObjectParams<Id> {
    /// Path to the file to upload.
    file_path: PathBuf,
    /// Name of the bucket to insert the S3 object into.
    bucket_name: String,
    /// Key for the S3 object.
    object_key: String,
    /// Marker for unique S3 object parameters type.
    marker: PhantomData<Id>,
}

impl<Id> S3ObjectParams<Id> {
    pub fn new(file_path: PathBuf, bucket_name: String, object_key: String) -> Self {
        Self {
            file_path,
            bucket_name,
            object_key,
            marker: PhantomData,
        }
    }

    /// Returns the path to the file to upload.
    pub fn file_path(&self) -> &Path {
        &self.file_path
    }

    /// Returns the bucket_name to put the S3 object into.
    pub fn bucket_name(&self) -> &str {
        self.bucket_name.as_ref()
    }

    /// Returns the key for the S3 object.
    pub fn object_key(&self) -> &str {
        self.object_key.as_ref()
    }
}
