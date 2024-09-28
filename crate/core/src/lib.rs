//! Low level data types for the peace automation framework.
//!
//! This crate exists because:
//!
//! * `peace_cfg` has a dependency on `peace_resource_rt` for `Resources`, used
//!   in `Item::setup`.
//! * `peace_resource_rt` has a dependency on `ItemId`, as uses `TypeMap<ItemId,
//!   _>` for the `States` maps.
//!
//!     When [peace#67] is implemented, the `progress` module can be moved out
//!     of `peace_core` into `peace_cfg`.
//!
//! [peace#67]: https://github.com/azriel91/peace/issues/67

// Re-exports
pub use peace_static_check_macros::{app_name, flow_id, item_id, profile};

pub use crate::{
    app_name::{AppName, AppNameInvalidFmt},
    apply_check::ApplyCheck,
    flow_id::{FlowId, FlowIdInvalidFmt},
    item_id::{ItemId, ItemIdInvalidFmt},
    profile::{Profile, ProfileInvalidFmt},
};

#[cfg(feature = "output_progress")]
pub mod progress;

mod app_name;
mod apply_check;
mod flow_id;
mod item_id;
mod profile;

/// Implements common behaviour for an ID type.
///
/// The implemented behaviour includes:
///
/// * `IdType::new`
/// * `IdType::new_unchecked`
/// * `IdType::is_valid_id`
/// * `std::ops::Deref`
/// * `std::ops::DerefMut`
/// * `std::fmt::Display`
/// * `std::str::FromStr`
/// * `TryFrom<String>`
/// * `TryFrom<&'static str>`
/// * `peace_fmt::Presentable`
///
/// A separate error type is also generated, which indicates an invalid value
/// when the ID type is instantiated with `new`.
///
/// # Usage
///
/// ```rust
/// use std::borrow::Cow;
///
/// // replace this with your ID type's macro
/// use peace_static_check_macros::my_id_type;
/// use serde::{Deserialize, Serialize};
///
/// // Rename your ID type
/// #[derive(Clone, Debug, Hash, PartialEq, Eq, Deserialize, Serialize)]
/// pub struct MyIdType(Cow<'static, str>);
///
/// crate::id_newtype!(
///     MyIdType,           // Name of the ID type
///     MyIdTypeInvalidFmt, // Name of the invalid value error
///     my_id_type,         // Name of the static check macro
///     tag,                // The `peace_fmt::Presentable` method to style the ID
/// );
/// ```
macro_rules! id_newtype {
    ($ty_name:ident, $ty_err_name:ident, $macro_name:ident, $presentable_method:ident) => {
        impl $ty_name {
            #[doc = concat!("Returns a new `", stringify!($ty_name), "` if the given `&str` is valid.")]
            ///
            #[doc = concat!("Most users should use the [`", stringify!($macro_name), "!`] macro as this provides")]
            /// compile time checks and returns a `const` value.
            ///
            #[doc = concat!("[`", stringify!($macro_name), "!`]: peace_static_check_macros::profile")]
            pub fn new(s: &'static str) -> Result<Self, $ty_err_name<'static>> {
                Self::try_from(s)
            }

            #[doc = concat!("Returns a new `", stringify!($ty_name), "`.")]
            ///
            #[doc = concat!("Most users should use the [`", stringify!($macro_name), "!`] macro as this provides")]
            /// compile time checks and returns a `const` value.
            ///
            #[doc = concat!("[`", stringify!($macro_name), "!`]: peace_static_check_macros::profile")]
            #[doc(hidden)]
            pub const fn new_unchecked(s: &'static str) -> Self {
                Self(std::borrow::Cow::Borrowed(s))
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

            /// Returns the inner `Cow<'static, str>`.
            pub fn into_inner(self) -> Cow<'static, str> {
                self.0
            }

            /// Returns the `&str` held by this ID.
            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl std::ops::Deref for $ty_name {
            type Target = std::borrow::Cow<'static, str>;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl std::fmt::Display for $ty_name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }

        #[peace_fmt::async_trait(?Send)]
        impl peace_fmt::Presentable for $ty_name {
            async fn present<'output, PR>(&self, presenter: &mut PR) -> Result<(), PR::Error>
            where
                PR: peace_fmt::Presenter<'output>
            {
                presenter.$presentable_method(self).await
            }
        }

        impl TryFrom<String> for $ty_name {
            type Error = $ty_err_name<'static>;

            fn try_from(s: String) -> Result<$ty_name, $ty_err_name<'static>> {
                if Self::is_valid_id(&s) {
                    Ok($ty_name(std::borrow::Cow::Owned(s)))
                } else {
                    let s = std::borrow::Cow::Owned(s);
                    Err($ty_err_name::new(s))
                }
            }
        }

        impl TryFrom<&'static str> for $ty_name {
            type Error = $ty_err_name<'static>;

            fn try_from(s: &'static str) -> Result<$ty_name, $ty_err_name<'static>> {
                if Self::is_valid_id(s) {
                    Ok($ty_name(std::borrow::Cow::Borrowed(s)))
                } else {
                    let s = std::borrow::Cow::Borrowed(s);
                    Err($ty_err_name::new(s))
                }
            }
        }

        impl std::str::FromStr for $ty_name {
            type Err = $ty_err_name<'static>;

            fn from_str(s: &str) -> Result<$ty_name, $ty_err_name<'static>> {
                if Self::is_valid_id(s) {
                    Ok($ty_name(std::borrow::Cow::Owned(String::from(s))))
                } else {
                    let s = std::borrow::Cow::Owned(String::from(s));
                    Err($ty_err_name::new(s))
                }
            }
        }

        #[doc = concat!("Error indicating `", stringify!($ty_name), "` provided is not in the correct format.")]
        #[derive(Clone, Debug, PartialEq, Eq)]
        pub struct $ty_err_name<'s> {
            /// String that was provided for the `$ty_name`.
            value: std::borrow::Cow<'s, str>,
        }

        impl<'s> $ty_err_name<'s> {
            #[doc = concat!("Returns a new `", stringify!($ty_err_name), "` error.")]
            pub fn new(value: std::borrow::Cow<'s, str>) -> Self {
                Self { value }
            }

            #[doc = concat!("Returns the value that failed to be parsed as a [`", stringify!($ty_name), "`].")]
            pub fn value(&self) -> &std::borrow::Cow<'s, str> {
                &self.value
            }
        }

        impl<'s> std::fmt::Display for $ty_err_name<'s> {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(
                    f,
                    "`{value}` is not a valid `{ty_name}`.\n\
                    `{ty_name}`s must begin with a letter or underscore, and contain only letters, numbers, or underscores.",
                    ty_name = stringify!($ty_name),
                    value = self.value
                )
            }
        }

        impl<'s> std::error::Error for $ty_err_name<'s> {}
    };
}

pub(crate) use id_newtype;
