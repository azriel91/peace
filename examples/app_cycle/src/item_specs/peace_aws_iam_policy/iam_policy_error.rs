#[cfg(feature = "error_reporting")]
use peace::miette::{self, SourceSpan};

use aws_sdk_iam::{
    error::{
        CreatePolicyError, CreatePolicyVersionError, DeletePolicyError, DeletePolicyVersionError,
        GetPolicyError, GetPolicyVersionError, ListPoliciesError, ListPolicyVersionsError,
    },
    types::SdkError,
};

/// Error while managing instance profile state.
#[cfg_attr(feature = "error_reporting", derive(peace::miette::Diagnostic))]
#[derive(Debug, thiserror::Error)]
pub enum IamPolicyError {
    /// Policy name or path was attempted to be modified.
    #[error("Policy name or path modification is not supported.")]
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

    /// Failed to list policies.
    #[error("Failed to list policies.")]
    #[cfg_attr(
        feature = "error_reporting",
        diagnostic(help("Make sure you are connected to the internet and try again."))
    )]
    PoliciesListError {
        /// Path prefix used to list policies
        path: String,
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
        error: SdkError<ListPoliciesError>,
    },

    /// Failed to decode URL-encoded policy document.
    #[error("Failed to decode URL-encoded policy document.")]
    PolicyDocumentNonUtf8 {
        /// Policy friendly name.
        policy_name: String,
        /// Policy path.
        policy_path: String,
        /// The URL encoded document from the AWS `get_policy_version` call.
        url_encoded_document: String,
        /// Underlying error.
        #[source]
        error: std::string::FromUtf8Error,
    },

    /// Failed to create policy.
    #[error("Failed to create policy.")]
    #[cfg_attr(
        feature = "error_reporting",
        diagnostic(help("Make sure you are connected to the internet and try again."))
    )]
    PolicyCreateError {
        /// Policy friendly name.
        policy_name: String,
        /// Policy path.
        policy_path: String,
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
        error: SdkError<CreatePolicyError>,
    },

    /// Failed to discover policy.
    #[error("Failed to discover policy.")]
    #[cfg_attr(
        feature = "error_reporting",
        diagnostic(help("Make sure you are connected to the internet and try again."))
    )]
    PolicyGetError {
        /// Expected policy friendly name.
        policy_name: String,
        /// Policy path.
        policy_path: String,
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
        error: SdkError<GetPolicyError>,
    },

    /// Failed to create policy version.
    #[error("Failed to create policy version.")]
    #[cfg_attr(
        feature = "error_reporting",
        diagnostic(help("Make sure you are connected to the internet and try again."))
    )]
    PolicyVersionCreateError {
        /// Policy friendly name.
        policy_name: String,
        /// Policy path.
        policy_path: String,
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
        error: SdkError<CreatePolicyVersionError>,
    },

    /// Failed to delete policy version.
    #[error("Failed to delete policy version.")]
    #[cfg_attr(
        feature = "error_reporting",
        diagnostic(help("Make sure you are connected to the internet and try again."))
    )]
    PolicyVersionDeleteError {
        /// Policy friendly name.
        policy_name: String,
        /// Policy path.
        policy_path: String,
        /// Version ID of the policy version to delete.
        version: String,
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
        error: SdkError<DeletePolicyVersionError>,
    },

    /// Failed to get policy version.
    #[error("Failed to get policy version.")]
    #[cfg_attr(
        feature = "error_reporting",
        diagnostic(help("Make sure you are connected to the internet and try again."))
    )]
    PolicyVersionGetError {
        /// Policy friendly name.
        policy_name: String,
        /// Policy path.
        policy_path: String,
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
        error: SdkError<GetPolicyVersionError>,
    },

    /// Failed to discover policy.
    #[error("Failed to discover policy.")]
    #[cfg_attr(
        feature = "error_reporting",
        diagnostic(help("Make sure you are connected to the internet and try again."))
    )]
    PolicyVersionsListError {
        /// Expected policy friendly name.
        policy_name: String,
        /// Policy path.
        policy_path: String,
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
        error: SdkError<ListPolicyVersionsError>,
    },

    /// Policy existed when listing policies, but did not exist when retrieving
    /// details.
    #[error("Policy details failed to be retrieved.")]
    #[cfg_attr(
        feature = "error_reporting",
        diagnostic(help("Someone may have deleted it, please try again."))
    )]
    PolicyNotFoundAfterList {
        /// Expected policy friendly name.
        policy_name: String,
        /// Policy path.
        policy_path: String,
        /// Policy stable ID.
        policy_id: String,
        /// Policy ARN.
        policy_arn: String,
        /// Error description from AWS error.
        #[cfg(feature = "error_reporting")]
        #[source_code]
        aws_desc: String,
        /// Span of the description to highlight.
        #[cfg(feature = "error_reporting")]
        #[label]
        aws_desc_span: SourceSpan,
    },

    /// Failed to delete policy.
    #[error("Failed to delete policy.")]
    #[cfg_attr(
        feature = "error_reporting",
        diagnostic(help("Make sure you are connected to the internet and try again."))
    )]
    PolicyDeleteError {
        /// Policy friendly name.
        policy_name: String,
        /// Policy path.
        policy_path: String,
        /// Policy stable ID.
        policy_id: String,
        /// Policy ARN.
        policy_arn: String,
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
        error: SdkError<DeletePolicyError>,
    },

    /// User changed the policy name or path, but AWS does not support changing
    /// this.
    #[error("Policy name or path cannot be modified, as it is not supported by AWS.")]
    PolicyModificationNotSupported {
        /// Name diff.
        name_diff: Option<(String, String)>,
        /// Path diff.
        path_diff: Option<(String, String)>,
    },
}
