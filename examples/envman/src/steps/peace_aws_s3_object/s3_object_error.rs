use std::{num::ParseIntError, path::PathBuf};

use aws_sdk_s3::{
    error::SdkError,
    operation::{
        delete_object::DeleteObjectError, head_object::HeadObjectError,
        list_objects::ListObjectsError, put_object::PutObjectError,
    },
};
#[cfg(feature = "error_reporting")]
use peace::miette::{self, SourceSpan};

/// Error while managing S3 object state.
#[cfg_attr(feature = "error_reporting", derive(peace::miette::Diagnostic))]
#[derive(Debug, thiserror::Error)]
pub enum S3ObjectError {
    /// Bucket for S3 object was attempted to be modified.
    #[error("S3 object bucket modification is not supported.")]
    BucketModificationNotSupported {
        /// Current S3 bucket for the object.
        bucket_name_current: String,
        /// Goal S3 bucket for the object.
        bucket_name_goal: String,
    },

    /// Failed to check file to upload existence.
    #[error("Failed to check file to upload existence: {}.", file_path.display())]
    ObjectFileExists {
        /// Path to the file to be uploaded.
        file_path: PathBuf,
        /// S3 bucket name.
        bucket_name: String,
        /// S3 object key.
        object_key: String,
        /// Underlying error.
        #[source]
        error: std::io::Error,
    },

    /// Failed to open file to upload.
    #[error("Failed to open file to upload.")]
    ObjectFileOpen {
        /// Path to the file to be uploaded.
        file_path: PathBuf,
        /// S3 bucket name.
        bucket_name: String,
        /// S3 object key.
        object_key: String,
        /// Underlying error.
        #[source]
        error: std::io::Error,
    },

    /// Error occurred reading file to upload.
    #[error("Error occurred reading file to upload.")]
    ObjectFileRead {
        /// Path to the file to be uploaded.
        file_path: PathBuf,
        /// S3 bucket name.
        bucket_name: String,
        /// S3 object key.
        object_key: String,
        /// Underlying error.
        #[source]
        error: std::io::Error,
    },

    /// Error occurred opening file to stream.
    #[error("Error occurred opening file to stream.")]
    ObjectFileStream {
        /// Path to the file to be uploaded.
        file_path: PathBuf,
        /// S3 bucket name.
        bucket_name: String,
        /// S3 object key.
        object_key: String,
        /// Underlying error.
        #[source]
        error: aws_smithy_types::byte_stream::error::Error,
    },

    /// Content MD5 hex string failed to be parsed as bytes.
    #[error("Content MD5 hex string failed to be parsed as bytes.")]
    ObjectContentMd5HexstrParse {
        /// Path to the file to be uploaded.
        file_path: PathBuf,
        /// S3 bucket name.
        bucket_name: String,
        /// S3 object key.
        object_key: String,
        /// Content MD5 string that was attempted to be parsed.
        content_md5_hexstr: String,
        /// Underlying error.
        #[source]
        error: ParseIntError,
    },

    /// Failed to base64 decode object ETag.
    #[error(
        "Failed to base64 decode object ETag. Bucket: {bucket_name}, object: {object_key}, etag: {content_md5_b64}."
    )]
    ObjectETagB64Decode {
        /// S3 bucket name.
        bucket_name: String,
        /// S3 object key.
        object_key: String,
        /// ETag that should represent base64 encoded MD5 hash.
        ///
        /// This was the value that was attempted to be parsed.
        content_md5_b64: String,
        error: base64::DecodeError,
    },

    /// S3 object key was attempted to be modified.
    #[error("S3 object key modification is not supported.")]
    ObjectKeyModificationNotSupported {
        /// S3 bucket name.
        bucket_name: String,
        /// Current key of the s3 object.
        object_key_current: String,
        /// Goal key of the s3 object.
        object_key_goal: String,
    },

    /// A `peace` runtime error occurred.
    #[error("A `peace` runtime error occurred.")]
    PeaceRtError(
        #[cfg_attr(feature = "error_reporting", diagnostic_source)]
        #[source]
        #[from]
        peace::rt_model::Error,
    ),

    /// Failed to list S3 objects.
    #[error("Failed to list S3 objects to discover: `{object_key}`.")]
    #[cfg_attr(
        feature = "error_reporting",
        diagnostic(help("Make sure you are connected to the internet and try again."))
    )]
    S3ObjectListError {
        /// S3 bucket name.
        bucket_name: String,
        /// S3 object key.
        object_key: String,
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
        error: SdkError<ListObjectsError>,
    },

    /// Failed to upload S3 object.
    #[error("Failed to upload S3 object: `{object_key}`.")]
    #[cfg_attr(
        feature = "error_reporting",
        diagnostic(help("Make sure you are connected to the internet and try again."))
    )]
    S3ObjectUploadError {
        /// S3 bucket name.
        bucket_name: String,
        /// S3 object key.
        object_key: String,
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
        error: SdkError<PutObjectError>,
    },

    /// Failed to discover S3 object.
    #[error("Failed to discover S3 object: `{object_key}`.")]
    #[cfg_attr(
        feature = "error_reporting",
        diagnostic(help("Make sure you are connected to the internet and try again."))
    )]
    S3ObjectGetError {
        /// Expected S3 object friendly name.
        object_key: String,
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
        error: SdkError<HeadObjectError>,
    },

    /// Failed to delete S3 object.
    #[error("Failed to delete S3 object: `{object_key}`.")]
    #[cfg_attr(
        feature = "error_reporting",
        diagnostic(help("Make sure you are connected to the internet and try again."))
    )]
    S3ObjectDeleteError {
        /// S3 bucket name.
        bucket_name: String,
        /// S3 object key.
        object_key: String,
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
        error: SdkError<DeleteObjectError>,
    },

    /// User changed the S3 object name, but AWS does not support
    /// changing this.
    #[error(
        "S3Object name cannot be modified, as it is not supported by AWS: current: `{object_key_current}`, goal: `{object_key_goal}`."
    )]
    S3ObjectModificationNotSupported {
        /// Current name of the s3 object.
        object_key_current: String,
        /// Goal name of the s3 object.
        object_key_goal: String,
    },
}
