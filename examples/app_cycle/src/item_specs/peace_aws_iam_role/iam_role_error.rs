#[cfg(feature = "error_reporting")]
use peace::miette;

use aws_sdk_iam::{
    error::{CreateRoleError, DeleteRoleError, GetRoleError},
    types::SdkError,
};

/// Error while managing instance profile state.
#[cfg_attr(feature = "error_reporting", derive(peace::miette::Diagnostic))]
#[derive(Debug, thiserror::Error)]
pub enum IamRoleError {
    /// A `peace` runtime error occurred.
    #[error("A `peace` runtime error occurred.")]
    PeaceRtError(
        #[cfg_attr(feature = "error_reporting", diagnostic_source)]
        #[source]
        #[from]
        peace::rt_model::Error,
    ),

    /// Failed to create role.
    #[error("Failed to create role.")]
    RoleCreateError {
        /// Role friendly name.
        role_name: String,
        /// Underlying error.
        #[source]
        error: SdkError<CreateRoleError>,
    },

    /// Failed to discover role.
    #[error("Failed to discover role.")]
    RoleGetError {
        /// Expected role friendly name.
        role_name: String,
        /// Underlying error.
        #[source]
        error: SdkError<GetRoleError>,
    },

    /// Failed to delete role.
    #[error("Failed to delete role.")]
    RoleDeleteError {
        /// Role friendly name.
        role_name: String,
        /// Role stable ID.
        role_id: String,
        /// Role ARN.
        role_arn: String,
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
