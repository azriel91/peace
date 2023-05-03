use std::marker::PhantomData;

use derivative::Derivative;
use peace::params::Params;
use serde::{Deserialize, Serialize};

/// IamRole item parameters.
///
/// The `Id` type parameter is needed for each instance profile params to be a
/// distinct type.
///
/// # Type Parameters
///
/// * `Id`: A zero-sized type used to distinguish different instance profile
///   parameters from each other.
#[derive(Derivative, Params, PartialEq, Eq, Deserialize, Serialize)]
#[derivative(Clone, Debug)]
pub struct IamRoleParams<Id> {
    /// Name for both the instance profile and role.
    ///
    /// Alphanumeric characters and `_+=,.@-` are allowed.
    ///
    /// TODO: newtype + proc macro.
    name: String,
    /// Namespace for both the instance profile and role.
    ///
    /// String that begins and ends with a forward slash.
    ///
    /// e.g. `/demo/`
    #[serde(default = "path_default")]
    path: String,
    /// Managed policy ARN to attach to the role.
    managed_policy_arn: String,
    /// Marker for unique instance profile parameters type.
    marker: PhantomData<Id>,
}

fn path_default() -> String {
    String::from("/")
}

impl<Id> IamRoleParams<Id> {
    /// Returns the name for both the instance profile and role.
    ///
    /// Alphanumeric characters and `_+=,.@-` are allowed.
    pub fn name(&self) -> &str {
        self.name.as_ref()
    }

    /// Returns the namespace for both the instance profile and role.
    ///
    /// String that begins and ends with a forward slash.
    ///
    /// e.g. `/demo/`
    pub fn path(&self) -> &str {
        self.path.as_ref()
    }

    /// Returns the ARN of the managed policy to attach.
    pub fn managed_policy_arn(&self) -> &str {
        self.managed_policy_arn.as_ref()
    }
}
