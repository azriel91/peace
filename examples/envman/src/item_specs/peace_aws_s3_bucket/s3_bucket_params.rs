use std::marker::PhantomData;

use derivative::Derivative;
use peace::params::Params;
use serde::{Deserialize, Serialize};

/// S3Bucket item parameters.
///
/// The `Id` type parameter is needed for each S3 bucket params to be a
/// distinct type.
///
/// # Type Parameters
///
/// * `Id`: A zero-sized type used to distinguish different S3 bucket parameters
///   from each other.
#[derive(Derivative, Params, PartialEq, Eq, Deserialize, Serialize)]
#[derivative(Clone, Debug)]
pub struct S3BucketParams<Id> {
    /// Name for both the S3 bucket and role.
    ///
    /// Alphanumeric characters and `_+=,.@-` are allowed.
    ///
    /// TODO: newtype + proc macro.
    name: String,
    /// Marker for unique S3 bucket parameters type.
    marker: PhantomData<Id>,
}

impl<Id> S3BucketParams<Id> {
    pub fn new(name: String) -> Self {
        Self {
            name,
            marker: PhantomData,
        }
    }

    /// Returns the name for both the S3 bucket and role.
    ///
    /// Alphanumeric characters and `_+=,.@-` are allowed.
    pub fn name(&self) -> &str {
        self.name.as_ref()
    }
}
