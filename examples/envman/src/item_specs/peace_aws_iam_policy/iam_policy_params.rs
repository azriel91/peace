use std::marker::PhantomData;

use derivative::Derivative;
use serde::{Deserialize, Serialize};

/// IamPolicy item parameters.
///
/// The `Id` type parameter is needed for each instance profile params to be a
/// distinct type.
///
/// # Type Parameters
///
/// * `Id`: A zero-sized type used to distinguish different instance profile
///   parameters from each other.
#[derive(Derivative, PartialEq, Eq, Deserialize, Serialize)]
#[derivative(Clone, Debug)]
pub struct IamPolicyParams<Id> {
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
    /// Policy document to use.
    policy_document: String,
    /// Marker for unique instance profile parameters type.
    marker: PhantomData<Id>,
}

fn path_default() -> String {
    String::from("/")
}

impl<Id> IamPolicyParams<Id> {
    pub fn new(name: String, path: String, policy_document: String) -> Self {
        Self {
            name,
            path,
            policy_document,
            marker: PhantomData,
        }
    }

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

    /// Returns the policy document to use.
    ///
    /// e.g.
    ///
    /// ```json
    /// {
    ///     "Version": "2012-10-17",
    ///     "Statement": [
    ///         {
    ///             "Effect": "Allow",
    ///             "Action": [
    ///                 "cloudformation:Describe*",
    ///                 "cloudformation:List*",
    ///                 "cloudformation:Get*"
    ///             ],
    ///             "Resource": "*"
    ///         }
    ///     ]
    /// }
    /// ```
    pub fn policy_document(&self) -> &str {
        self.policy_document.as_ref()
    }
}
