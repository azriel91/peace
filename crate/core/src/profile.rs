use std::borrow::Cow;

use peace_static_check_macros::profile;
use serde::{Deserialize, Serialize};

/// Identifier or namespace to distinguish execution environments.
///
/// Example suitable identifiers are:
///
/// * `"dev_user1"`
/// * `"dev_user2"`
/// * `"prod_customer1"`
///
/// Must begin with a letter or underscore, and contain only letters, numbers,
/// and underscores.
///
/// # Examples
///
/// The following are all examples of valid `Profile`s:
///
/// ```rust
/// # use peace_core::{profile, Profile};
/// #
/// let _snake = profile!("snake_case");
/// let _camel = profile!("camelCase");
/// let _pascal = profile!("PascalCase");
/// ```
#[derive(Clone, Debug, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub struct Profile(Cow<'static, str>);

crate::id_newtype!(Profile, ProfileInvalidFmt, profile, tag);

impl Profile {
    /// Profile used by the Peace framework when a command is for initializing
    /// the workspace.
    pub const fn workspace_init() -> Self {
        profile!("workspace_init")
    }
}
