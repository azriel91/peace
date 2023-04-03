#[cfg(feature = "error_reporting")]
use peace::miette::{self, SourceSpan};

use aws_sdk_iam::{
    error::{
        AttachRolePolicyError, CreateRoleError, DeleteRoleError, DetachRolePolicyError,
        GetRoleError, ListAttachedRolePoliciesError,
    },
    types::SdkError,
};

/// Error while managing instance profile state.
#[cfg_attr(feature = "error_reporting", derive(peace::miette::Diagnostic))]
#[derive(Debug, thiserror::Error)]
pub enum IamRoleError {
    /// Role name or path was attempted to be modified.
    #[error("Role name or path modification is not supported.")]
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

    /// Failed to attach managed policy.
    #[error("Failed to attach managed policy.")]
    #[cfg_attr(
        feature = "error_reporting",
        diagnostic(help("Make sure you are connected to the internet and try again."))
    )]
    ManagedPolicyAttachError {
        /// Role friendly name.
        role_name: String,
        /// Role path.
        role_path: String,
        /// ARN of the managed policy.
        managed_policy_arn: String,
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
        error: SdkError<AttachRolePolicyError>,
    },

    /// Failed to detach managed policy.
    #[error("Failed to detach managed policy.")]
    #[cfg_attr(
        feature = "error_reporting",
        diagnostic(help("Make sure you are connected to the internet and try again."))
    )]
    ManagedPolicyDetachError {
        /// Role friendly name.
        role_name: String,
        /// Role path.
        role_path: String,
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
        error: SdkError<DetachRolePolicyError>,
    },

    /// Failed to list managed policies for role.
    #[error("Failed to list managed policies for role.")]
    #[cfg_attr(
        feature = "error_reporting",
        diagnostic(help("Make sure you are connected to the internet and try again."))
    )]
    ManagedPoliciesListError {
        /// Role friendly name.
        role_name: String,
        /// Role path.
        role_path: String,
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
        error: SdkError<ListAttachedRolePoliciesError>,
    },

    /// Failed to create role.
    #[error("Failed to create role.")]
    #[cfg_attr(
        feature = "error_reporting",
        diagnostic(help("Make sure you are connected to the internet and try again."))
    )]
    RoleCreateError {
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
        error: SdkError<CreateRoleError>,
    },

    /// Failed to discover role.
    #[error("Failed to discover role.")]
    #[cfg_attr(
        feature = "error_reporting",
        diagnostic(help("Make sure you are connected to the internet and try again."))
    )]
    RoleGetError {
        /// Expected role friendly name.
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
        error: SdkError<GetRoleError>,
    },

    /// Failed to delete role.
    #[error("Failed to delete role.")]
    #[cfg_attr(
        feature = "error_reporting",
        diagnostic(help("Make sure you are connected to the internet and try again."))
    )]
    RoleDeleteError {
        /// Role friendly name.
        role_name: String,
        /// Role stable ID.
        role_id: String,
        /// Role ARN.
        role_arn: String,
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
        error: SdkError<DeleteRoleError>,
    },

    /// User changed the role name or path, but AWS does not support changing
    /// this.
    #[error("Role name or path cannot be modified, as it is not supported by AWS.")]
    RoleModificationNotSupported {
        /// Name diff.
        name_diff: Option<(String, String)>,
        /// Path diff.
        path_diff: Option<(String, String)>,
    },
}
