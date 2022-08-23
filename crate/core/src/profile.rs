use std::{borrow::Cow, convert::TryFrom, fmt, ops::Deref, str::FromStr};

use serde::{Deserialize, Serialize};

use crate::ProfileInvalidFmt;

/// Unique identifier for a `Profile`, `Cow<'static, str>` newtype.
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

impl Profile {
    /// Returns a `Profile` if the given `&str` is valid.
    ///
    /// Most users should use the [`profile!`] macro as this provides
    /// compile time checks and returns a `const` value.
    ///
    /// [`profile!`]: peace_profile_macro::profile
    pub fn new(s: &'static str) -> Result<Self, ProfileInvalidFmt> {
        Self::try_from(s)
    }

    /// Returns a `Profile`.
    ///
    /// Most users should use the [`profile!`] macro as this provides
    /// compile time checks and returns a `const` value.
    ///
    /// [`profile!`]: peace_profile_macro::profile
    #[doc(hidden)]
    pub const fn new_unchecked(s: &'static str) -> Self {
        Self(Cow::Borrowed(s))
    }

    /// Returns whether the provided `&str` is a valid station identifier.
    pub fn is_valid_id(proposed_id: &str) -> bool {
        let mut chars = proposed_id.chars();
        let first_char = chars.next();
        let first_char_valid = first_char
            .map(|c| c.is_ascii_alphabetic() || c == '_')
            .unwrap_or(false);
        let remainder_chars_valid =
            chars.all(|c| c.is_ascii_alphabetic() || c == '_' || c.is_ascii_digit());

        first_char_valid && remainder_chars_valid
    }
}

impl Deref for Profile {
    type Target = Cow<'static, str>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl fmt::Display for Profile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TryFrom<String> for Profile {
    type Error = ProfileInvalidFmt<'static>;

    fn try_from(s: String) -> Result<Profile, ProfileInvalidFmt<'static>> {
        if Self::is_valid_id(&s) {
            Ok(Profile(Cow::Owned(s)))
        } else {
            let s = Cow::Owned(s);
            Err(ProfileInvalidFmt::new(s))
        }
    }
}

impl TryFrom<&'static str> for Profile {
    type Error = ProfileInvalidFmt<'static>;

    fn try_from(s: &'static str) -> Result<Profile, ProfileInvalidFmt<'static>> {
        if Self::is_valid_id(s) {
            Ok(Profile(Cow::Borrowed(s)))
        } else {
            let s = Cow::Borrowed(s);
            Err(ProfileInvalidFmt::new(s))
        }
    }
}

impl FromStr for Profile {
    type Err = ProfileInvalidFmt<'static>;

    fn from_str(s: &str) -> Result<Profile, ProfileInvalidFmt<'static>> {
        if Self::is_valid_id(s) {
            Ok(Profile(Cow::Owned(String::from(s))))
        } else {
            let s = Cow::Owned(String::from(s));
            Err(ProfileInvalidFmt::new(s))
        }
    }
}
