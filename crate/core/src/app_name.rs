use std::borrow::Cow;

use serde::{Deserialize, Serialize};

/// Name of the application that is run by end users.
///
/// This is usually the crate name of the final binary. It needs to be passed in
/// from the executable crate, as it is not possible for Peace to detect it
/// within the library itself.
///
/// Must begin with a letter or underscore, and contain only letters, numbers,
/// and underscores.
///
/// # Examples
///
/// The following are all examples of valid `AppName`s:
///
/// ```rust
/// # use peace_core::{app_name, AppName};
/// #
/// let _default = app_name!(); // defaults to calling crate name
/// let _snake = app_name!("snake_case");
/// let _camel = app_name!("camelCase");
/// let _pascal = app_name!("PascalCase");
/// ```
#[derive(Clone, Debug, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub struct AppName(Cow<'static, str>);

crate::id_newtype!(AppName, AppNameInvalidFmt, app_name);
