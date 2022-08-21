use std::{borrow::Cow, convert::TryFrom, fmt, ops::Deref, str::FromStr};

use serde::{Deserialize, Serialize};

use crate::ItemSpecIdInvalidFmt;

/// Unique identifier for an `ItemSpec`, `Cow<'static, str>` newtype.
///
/// Must begin with a letter or underscore, and contain only letters, numbers,
/// and underscores.
///
/// # Examples
///
/// The following are all examples of valid `ItemSpecId`s
///
/// ```rust
/// # use peace_cfg::{item_spec_id, ItemSpecId};
/// #
/// let _snake = item_spec_id!("snake_case");
/// let _camel = item_spec_id!("camelCase");
/// let _pascal = item_spec_id!("PascalCase");
/// ```
#[derive(Clone, Debug, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub struct ItemSpecId(Cow<'static, str>);

impl ItemSpecId {
    /// Returns an `ItemSpecId` if the given `&str` is valid.
    ///
    /// Most users should use the [`item_spec_id!`] macro as this provides
    /// compile time checks and returns a `const` value.
    ///
    /// [`item_spec_id!`]: peace_item_spec_id_macro::item_spec_id
    pub fn new(s: &'static str) -> Result<Self, ItemSpecIdInvalidFmt> {
        Self::try_from(s)
    }

    /// Returns an `ItemSpecId`.
    ///
    /// Most users should use the [`item_spec_id!`] macro as this provides
    /// compile time checks and returns a `const` value.
    ///
    /// [`item_spec_id!`]: peace_item_spec_id_macro::item_spec_id
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

impl Deref for ItemSpecId {
    type Target = Cow<'static, str>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl fmt::Display for ItemSpecId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TryFrom<String> for ItemSpecId {
    type Error = ItemSpecIdInvalidFmt<'static>;

    fn try_from(s: String) -> Result<ItemSpecId, ItemSpecIdInvalidFmt<'static>> {
        if Self::is_valid_id(&s) {
            Ok(ItemSpecId(Cow::Owned(s)))
        } else {
            let s = Cow::Owned(s);
            Err(ItemSpecIdInvalidFmt::new(s))
        }
    }
}

impl TryFrom<&'static str> for ItemSpecId {
    type Error = ItemSpecIdInvalidFmt<'static>;

    fn try_from(s: &'static str) -> Result<ItemSpecId, ItemSpecIdInvalidFmt<'static>> {
        if Self::is_valid_id(s) {
            Ok(ItemSpecId(Cow::Borrowed(s)))
        } else {
            let s = Cow::Borrowed(s);
            Err(ItemSpecIdInvalidFmt::new(s))
        }
    }
}

impl FromStr for ItemSpecId {
    type Err = ItemSpecIdInvalidFmt<'static>;

    fn from_str(s: &str) -> Result<ItemSpecId, ItemSpecIdInvalidFmt<'static>> {
        if Self::is_valid_id(s) {
            Ok(ItemSpecId(Cow::Owned(String::from(s))))
        } else {
            let s = Cow::Owned(String::from(s));
            Err(ItemSpecIdInvalidFmt::new(s))
        }
    }
}
