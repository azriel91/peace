use std::{borrow::Cow, convert::TryFrom, fmt, ops::Deref, str::FromStr};

use serde::{Deserialize, Serialize};

use crate::FullSpecIdInvalidFmt;

/// Unique identifier for a `FullSpec`, `Cow<'static, str>` newtype.
///
/// Must begin with a letter or underscore, and contain only letters, numbers,
/// and underscores.
///
/// # Examples
///
/// The following are all examples of valid `FullSpecId`s
///
/// ```rust
/// # use peace_cfg::{full_spec_id, FullSpecId};
/// #
/// let _snake = full_spec_id!("snake_case");
/// let _camel = full_spec_id!("camelCase");
/// let _pascal = full_spec_id!("PascalCase");
/// ```
#[derive(Clone, Debug, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub struct FullSpecId(Cow<'static, str>);

impl FullSpecId {
    /// Returns a `FullSpecId` if the given `&str` is valid.
    ///
    /// Most users should use the [`full_spec_id!`] macro as this provides
    /// compile time checks and returns a `const` value.
    ///
    /// [`full_spec_id!`]: peace_full_spec_id_macro::full_spec_id
    pub fn new(s: &'static str) -> Result<Self, FullSpecIdInvalidFmt> {
        Self::try_from(s)
    }

    /// Returns a `FullSpecId`.
    ///
    /// Most users should use the [`full_spec_id!`] macro as this provides
    /// compile time checks and returns a `const` value.
    ///
    /// [`full_spec_id!`]: peace_full_spec_id_macro::full_spec_id
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

impl Deref for FullSpecId {
    type Target = Cow<'static, str>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl fmt::Display for FullSpecId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TryFrom<String> for FullSpecId {
    type Error = FullSpecIdInvalidFmt<'static>;

    fn try_from(s: String) -> Result<FullSpecId, FullSpecIdInvalidFmt<'static>> {
        if Self::is_valid_id(&s) {
            Ok(FullSpecId(Cow::Owned(s)))
        } else {
            let s = Cow::Owned(s);
            Err(FullSpecIdInvalidFmt::new(s))
        }
    }
}

impl TryFrom<&'static str> for FullSpecId {
    type Error = FullSpecIdInvalidFmt<'static>;

    fn try_from(s: &'static str) -> Result<FullSpecId, FullSpecIdInvalidFmt<'static>> {
        if Self::is_valid_id(s) {
            Ok(FullSpecId(Cow::Borrowed(s)))
        } else {
            let s = Cow::Borrowed(s);
            Err(FullSpecIdInvalidFmt::new(s))
        }
    }
}

impl FromStr for FullSpecId {
    type Err = FullSpecIdInvalidFmt<'static>;

    fn from_str(s: &str) -> Result<FullSpecId, FullSpecIdInvalidFmt<'static>> {
        if Self::is_valid_id(s) {
            Ok(FullSpecId(Cow::Owned(String::from(s))))
        } else {
            let s = Cow::Owned(String::from(s));
            Err(FullSpecIdInvalidFmt::new(s))
        }
    }
}
