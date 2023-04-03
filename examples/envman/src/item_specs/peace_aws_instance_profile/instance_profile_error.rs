#[cfg(feature = "error_reporting")]
use peace::miette::{self, SourceSpan};

use aws_sdk_iam::{
    error::{
        AddRoleToInstanceProfileError, CreateInstanceProfileError, DeleteInstanceProfileError,
        GetInstanceProfileError, RemoveRoleFromInstanceProfileError,
    },
    types::SdkError,
};

/// Error while managing instance profile state.
#[cfg_attr(feature = "error_reporting", derive(peace::miette::Diagnostic))]
#[derive(Debug, thiserror::Error)]
pub enum InstanceProfileError {
    /// Instance profile name or path was attempted to be modified.
    #[error("Instance profile name or path modification is not supported.")]
    NameOrPathModificationNotSupported {
        /// Whether the name has been changed.
        name_diff: Option<(String, String)>,
        /// Whether the path has been changed.
        path_diff: Option<(String, String)>,
    },

    /// A `peace` runtime error occurred.
    #[error("A `peace` runtime error occurred.")]
    PeaceRtError(
        #[cfg_attr(feature = "error_reporting", diagnostic_source)]
        #[source]
        #[from]
        peace::rt_model::Error,
    ),

    /// Failed to decode URL-encoded instance profile document.
    #[error("Failed to decode URL-encoded instance_profile document.")]
    InstanceProfileDocumentNonUtf8 {
        /// InstanceProfile friendly name.
        instance_profile_name: String,
        /// InstanceProfile path.
        instance_profile_path: String,
        /// The URL encoded document from the AWS `get_instance_profile_version`
        /// call.
        url_encoded_document: String,
        /// Underlying error.
        #[source]
        error: std::string::FromUtf8Error,
    },

    /// Failed to create instance profile.
    #[error("Failed to create instance profile.")]
    #[cfg_attr(
        feature = "error_reporting",
        diagnostic(help("Make sure you are connected to the internet and try again."))
    )]
    InstanceProfileCreateError {
        /// InstanceProfile friendly name.
        instance_profile_name: String,
        /// InstanceProfile path.
        instance_profile_path: String,
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
        error: SdkError<CreateInstanceProfileError>,
    },

    /// Failed to discover instance profile.
    #[error("Failed to discover instance profile.")]
    #[cfg_attr(
        feature = "error_reporting",
        diagnostic(help("Make sure you are connected to the internet and try again."))
    )]
    InstanceProfileGetError {
        /// Expected instance profile friendly name.
        instance_profile_name: String,
        /// InstanceProfile path.
        instance_profile_path: String,
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
        error: SdkError<GetInstanceProfileError>,
    },

    /// InstanceProfile existed when listing policies, but did not exist when
    /// retrieving details.
    #[error("InstanceProfile details failed to be retrieved.")]
    InstanceProfileNotFoundAfterList {
        /// Expected instance profile friendly name.
        instance_profile_name: String,
        /// InstanceProfile path.
        instance_profile_path: String,
        /// InstanceProfile stable ID.
        instance_profile_id: String,
        /// InstanceProfile ARN.
        instance_profile_arn: String,
    },

    /// Failed to delete instance profile.
    #[error("Failed to delete instance profile.")]
    #[cfg_attr(
        feature = "error_reporting",
        diagnostic(help("Make sure you are connected to the internet and try again."))
    )]
    InstanceProfileDeleteError {
        /// InstanceProfile friendly name.
        instance_profile_name: String,
        /// InstanceProfile path.
        instance_profile_path: String,
        /// InstanceProfile stable ID.
        instance_profile_id: String,
        /// InstanceProfile ARN.
        instance_profile_arn: String,
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
        error: SdkError<DeleteInstanceProfileError>,
    },

    /// Failed to add role to instance profile.
    #[error("Failed to add role to instance profile: {role_name}.")]
    #[cfg_attr(
        feature = "error_reporting",
        diagnostic(help("Make sure you are connected to the internet and try again."))
    )]
    InstanceProfileRoleAddError {
        /// InstanceProfile friendly name.
        instance_profile_name: String,
        /// InstanceProfile path.
        instance_profile_path: String,
        /// Role friendly name.
        role_name: String,
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
        error: SdkError<AddRoleToInstanceProfileError>,
    },

    /// Failed to remove role from instance profile.
    #[error("Failed to remove role from instance profile.")]
    #[cfg_attr(
        feature = "error_reporting",
        diagnostic(help("Make sure you are connected to the internet and try again."))
    )]
    InstanceProfileRoleRemoveError {
        /// InstanceProfile friendly name.
        instance_profile_name: String,
        /// InstanceProfile path.
        instance_profile_path: String,
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
        error: SdkError<RemoveRoleFromInstanceProfileError>,
    },

    /// User changed the instance profile name or path, but AWS does not support
    /// changing this.
    #[error("InstanceProfile name or path cannot be modified, as it is not supported by AWS.")]
    InstanceProfileModificationNotSupported {
        /// Name diff.
        name_diff: Option<(String, String)>,
        /// Path diff.
        path_diff: Option<(String, String)>,
    },
}
