use std::marker::PhantomData;

use derivative::Derivative;
use serde::{Deserialize, Serialize};

/// InstanceProfile item parameters.
///
/// The `Id` type parameter is needed for each instance profile params to be a
/// distinct type.
///
/// # Type Parameters
///
/// * `Id`: A zero-sized type used to distinguish different instance profile
///   parameters from each other.
#[derive(Clone, Derivative, PartialEq, Eq, Deserialize, Serialize)]
#[derivative(Debug)]
pub struct InstanceProfileParams<Id> {
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
    /// Whether or not to associate the profile with a role of the same name.
    ///
    /// The role must already exist.
    role_associate: bool,
    /// Marker for unique instance profile parameters type.
    marker: PhantomData<Id>,
}

fn path_default() -> String {
    String::from("/")
}

impl<Id> InstanceProfileParams<Id> {
    pub fn new(name: String, path: String, role_associate: bool) -> Self {
        Self {
            name,
            path,
            role_associate,
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

    /// Whether or not to associate the profile with a role of the same name.
    pub fn role_associate(&self) -> bool {
        self.role_associate
    }
}
