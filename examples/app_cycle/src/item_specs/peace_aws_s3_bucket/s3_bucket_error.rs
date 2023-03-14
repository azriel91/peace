#[cfg(feature = "error_reporting")]
use peace::miette;

use aws_sdk_s3::{
    self,
    error::{
        BucketAlreadyExists, BucketAlreadyOwnedByYou, CreateBucketError, DeleteBucketError,
        HeadBucketError, ListBucketsError,
    },
    types::SdkError,
};

/// Error while managing S3 bucket state.
#[cfg_attr(feature = "error_reporting", derive(peace::miette::Diagnostic))]
#[derive(Debug, thiserror::Error)]
pub enum S3BucketError {
    /// S3 bucket name was attempted to be modified.
    #[error("S3 bucket name modification is not supported.")]
    NameModificationNotSupported {
        /// Current name of the s3 bucket.
        s3_bucket_name_current: String,
        /// Desired name of the s3 bucket.
        s3_bucket_name_desired: String,
    },

    /// A `peace` runtime error occurred.
    #[error("A `peace` runtime error occurred.")]
    PeaceRtError(
        #[cfg_attr(feature = "error_reporting", diagnostic_source)]
        #[source]
        #[from]
        peace::rt_model::Error,
    ),

    /// Failed to list S3 buckets.
    #[error("Failed to list S3 buckets to discover: `{s3_bucket_name}`.")]
    S3BucketListError {
        /// S3Bucket friendly name.
        s3_bucket_name: String,
        /// Underlying error.
        #[source]
        error: SdkError<ListBucketsError>,
    },

    /// Failed to create S3 bucket as someone else owns the name.
    #[error("Failed to create S3 bucket as someone else owns the name: `{s3_bucket_name}`.")]
    S3BucketOwnedBySomeoneElseError {
        /// S3Bucket friendly name.
        s3_bucket_name: String,
        /// Underlying error.
        #[source]
        error: BucketAlreadyExists,
    },

    /// Failed to create S3 bucket as you already have one with the same name.
    #[error(
        "Failed to create S3 bucket as you already have one with the same name: `{s3_bucket_name}`."
    )]
    S3BucketOwnedByYouError {
        /// S3Bucket friendly name.
        s3_bucket_name: String,
        /// Underlying error.
        #[source]
        error: BucketAlreadyOwnedByYou,
    },

    /// Failed to create S3 bucket.
    #[error("Failed to create S3 bucket: `{s3_bucket_name}`.")]
    S3BucketCreateError {
        /// S3Bucket friendly name.
        s3_bucket_name: String,
        /// Underlying error.
        #[source]
        error: SdkError<CreateBucketError>,
    },

    /// Failed to discover S3 bucket.
    #[error("Failed to discover S3 bucket: `{s3_bucket_name}`.")]
    S3BucketGetError {
        /// Expected S3 bucket friendly name.
        s3_bucket_name: String,
        /// Underlying error.
        #[source]
        error: SdkError<HeadBucketError>,
    },

    /// Failed to delete S3 bucket.
    #[error("Failed to delete S3 bucket: `{s3_bucket_name}`.")]
    S3BucketDeleteError {
        /// S3Bucket friendly name.
        s3_bucket_name: String,
        /// Underlying error.
        #[source]
        error: SdkError<DeleteBucketError>,
    },

    /// User changed the S3 bucket name, but AWS does not support
    /// changing this.
    #[error(
        "S3Bucket name cannot be modified, as it is not supported by AWS: current: `{s3_bucket_name_current}`, desired: `{s3_bucket_name_desired}`."
    )]
    S3BucketModificationNotSupported {
        /// Current name of the s3 bucket.
        s3_bucket_name_current: String,
        /// Desired name of the s3 bucket.
        s3_bucket_name_desired: String,
    },
}
