use std::borrow::Cow;

use serde::{Deserialize, Serialize};

/// Unique identifier for an `ItemId`, `Cow<'static, str>` newtype.
///
/// Must begin with a letter or underscore, and contain only letters, numbers,
/// and underscores.
///
/// # Examples
///
/// The following are all examples of valid `ItemId`s:
///
/// ```rust
/// # use peace_core::{item_id, ItemId};
/// #
/// let _snake = item_id!("snake_case");
/// let _camel = item_id!("camelCase");
/// let _pascal = item_id!("PascalCase");
/// ```
#[derive(Clone, Debug, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub struct ItemId(Cow<'static, str>);

crate::id_newtype!(ItemId, ItemIdInvalidFmt, item_id, code_inline);
