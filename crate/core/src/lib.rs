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
    flow_id::{FlowId, FlowIdInvalidFmt},
    item_id::{ItemId, ItemIdInvalidFmt},
    profile::{Profile, ProfileInvalidFmt},
};

mod app_name;
mod flow_id;
mod item_id;
mod profile;

#[macro_use]
extern crate id_newtype;

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
/// crate::core_id_newtype!(
///     MyIdType,           // Name of the ID type
///     MyIdTypeInvalidFmt, // Name of the invalid value error
///     my_id_type,         // Name of the static check macro
///     tag,                // The `peace_fmt::Presentable` method to style the ID
/// );
/// ```
macro_rules! core_id_newtype {
    ($ty_name:ident, $ty_err_name:ident, $macro_name:ident, $presentable_method:ident) => {
        id_newtype!($ty_name, $ty_err_name, $macro_name);

        #[peace_fmt::async_trait(?Send)]
        impl peace_fmt::Presentable for $ty_name {
            async fn present<'output, PR>(&self, presenter: &mut PR) -> Result<(), PR::Error>
            where
                PR: peace_fmt::Presenter<'output>,
            {
                presenter.$presentable_method(self.as_str()).await
            }
        }
    };
}

pub(crate) use core_id_newtype;
