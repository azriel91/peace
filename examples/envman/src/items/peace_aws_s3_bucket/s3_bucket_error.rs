use aws_sdk_s3::{
    error::SdkError,
    operation::{
        create_bucket::CreateBucketError, delete_bucket::DeleteBucketError,
        head_bucket::HeadBucketError, list_buckets::ListBucketsError,
    },
    types::error::{BucketAlreadyExists, BucketAlreadyOwnedByYou},
};
#[cfg(feature = "error_reporting")]
use peace::miette::{self, SourceSpan};

/// Error while managing S3 bucket state.
#[cfg_attr(feature = "error_reporting", derive(peace::miette::Diagnostic))]
#[derive(Debug, thiserror::Error)]
pub enum S3BucketError {
    /// S3 bucket name was attempted to be modified.
    #[error("S3 bucket name modification is not supported.")]
    NameModificationNotSupported {
        /// Current name of the s3 bucket.
        s3_bucket_name_current: String,
        /// Goal name of the s3 bucket.
        s3_bucket_name_goal: String,
    },

    /// Failed to list S3 buckets.
    #[error("Failed to discover: `{s3_bucket_name}`.")]
    #[cfg_attr(
        feature = "error_reporting",
        diagnostic(help("Make sure you are connected to the internet and try again."))
    )]
    S3BucketListError {
        /// S3Bucket friendly name.
        s3_bucket_name: String,
        /// Error description from AWS error.
        #[cfg(feature = "error_reporting")]
        #[source_code]
        aws_desc: String,
        /// Span of the description to highlight.
        #[cfg(feature = "error_reporting")]
        #[label]
        aws_desc_span: SourceSpan,
        /// Underlying error.
        #[source]
        error: Box<SdkError<ListBucketsError>>,
    },

    /// Failed to create S3 bucket as someone else owns the name.
    #[error("Failed to create S3 bucket: `{s3_bucket_name}`.")]
    #[cfg_attr(
        feature = "error_reporting",
        diagnostic(help(
            "Someone else owns the S3 bucket name.\n\
            \n\
            Please use a different profile name by running:\n\
            ```\n\
            ./envman switch <profile_name> --create --type development username/repo <version>\n\
            ```\n\
            "
        ))
    )]
    S3BucketOwnedBySomeoneElseError {
        /// S3Bucket friendly name.
        s3_bucket_name: String,
        /// Underlying error.
        #[source]
        error: Box<BucketAlreadyExists>,
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
        error: Box<BucketAlreadyOwnedByYou>,
    },

    /// Failed to create S3 bucket.
    #[error("Failed to create S3 bucket: `{s3_bucket_name}`.")]
    #[cfg_attr(
        feature = "error_reporting",
        diagnostic(help("Make sure you are connected to the internet and try again."))
    )]
    S3BucketCreateError {
        /// S3Bucket friendly name.
        s3_bucket_name: String,
        /// Error description from AWS error.
        #[cfg(feature = "error_reporting")]
        #[source_code]
        aws_desc: String,
        /// Span of the description to highlight.
        #[cfg(feature = "error_reporting")]
        #[label]
        aws_desc_span: SourceSpan,
        /// Underlying error.
        #[source]
        error: Box<SdkError<CreateBucketError>>,
    },

    /// Failed to discover S3 bucket.
    #[error("Failed to discover S3 bucket: `{s3_bucket_name}`.")]
    #[cfg_attr(
        feature = "error_reporting",
        diagnostic(help("Make sure you are connected to the internet and try again."))
    )]
    S3BucketGetError {
        /// Expected S3 bucket friendly name.
        s3_bucket_name: String,
        /// Error description from AWS error.
        #[cfg(feature = "error_reporting")]
        #[source_code]
        aws_desc: String,
        /// Span of the description to highlight.
        #[cfg(feature = "error_reporting")]
        #[label]
        aws_desc_span: SourceSpan,
        /// Underlying error.
        #[source]
        error: Box<SdkError<HeadBucketError>>,
    },

    /// Failed to delete S3 bucket.
    #[error("Failed to delete S3 bucket: `{s3_bucket_name}`.")]
    #[cfg_attr(
        feature = "error_reporting",
        diagnostic(help("Make sure you are connected to the internet and try again."))
    )]
    S3BucketDeleteError {
        /// S3Bucket friendly name.
        s3_bucket_name: String,
        /// Error description from AWS error.
        #[cfg(feature = "error_reporting")]
        #[source_code]
        aws_desc: String,
        /// Span of the description to highlight.
        #[cfg(feature = "error_reporting")]
        #[label]
        aws_desc_span: SourceSpan,
        /// Underlying error.
        #[source]
        error: Box<SdkError<DeleteBucketError>>,
    },

    /// User changed the S3 bucket name, but AWS does not support
    /// changing this.
    #[error(
        "S3Bucket name cannot be modified, as it is not supported by AWS: current: `{s3_bucket_name_current}`, goal: `{s3_bucket_name_goal}`."
    )]
    S3BucketModificationNotSupported {
        /// Current name of the s3 bucket.
        s3_bucket_name_current: String,
        /// Goal name of the s3 bucket.
        s3_bucket_name_goal: String,
    },
}
