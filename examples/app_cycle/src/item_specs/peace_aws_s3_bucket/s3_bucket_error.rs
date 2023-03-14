#[cfg(feature = "error_reporting")]
use peace::miette;

use aws_sdk_s3::{
    self,
    error::{CreateBucketError, DeleteBucketError, HeadBucketError},
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

    /// Failed to decode URL-encoded S3 bucket document.
    #[error("Failed to decode URL-encoded s3_bucket document.")]
    S3BucketDocumentNonUtf8 {
        /// S3Bucket friendly name.
        s3_bucket_name: String,
        /// The URL encoded document from the AWS `get_s3_bucket_version`
        /// call.
        url_encoded_document: String,
        /// Underlying error.
        #[source]
        error: std::string::FromUtf8Error,
    },

    /// Failed to create S3 bucket.
    #[error("Failed to create S3 bucket.")]
    S3BucketCreateError {
        /// S3Bucket friendly name.
        s3_bucket_name: String,
        /// Underlying error.
        #[source]
        error: SdkError<CreateBucketError>,
    },

    /// Failed to discover S3 bucket.
    #[error("Failed to discover S3 bucket.")]
    S3BucketGetError {
        /// Expected S3 bucket friendly name.
        s3_bucket_name: String,
        /// Underlying error.
        #[source]
        error: SdkError<HeadBucketError>,
    },

    /// S3Bucket existed when listing policies, but did not exist when
    /// retrieving details.
    #[error("S3Bucket details failed to be retrieved.")]
    S3BucketNotFoundAfterList {
        /// Expected S3 bucket friendly name.
        s3_bucket_name: String,
    },

    /// Failed to delete S3 bucket.
    #[error("Failed to delete S3 bucket.")]
    S3BucketDeleteError {
        /// S3Bucket friendly name.
        s3_bucket_name: String,
        /// Underlying error.
        #[source]
        error: SdkError<DeleteBucketError>,
    },

    /// User changed the S3 bucket name, but AWS does not support
    /// changing this.
    #[error("S3Bucket name cannot be modified, as it is not supported by AWS.")]
    S3BucketModificationNotSupported {
        /// Current name of the s3 bucket.
        s3_bucket_name_current: String,
        /// Desired name of the s3 bucket.
        s3_bucket_name_desired: String,
    },
}
