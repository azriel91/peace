use std::borrow::Cow;

use serde::{Deserialize, Serialize};

/// Unique identifier for an `ItemSpecId`, `Cow<'static, str>` newtype.
///
/// Must begin with a letter or underscore, and contain only letters, numbers,
/// and underscores.
///
/// # Examples
///
/// The following are all examples of valid `ItemSpecId`s:
///
/// ```rust
/// # use peace_core::{item_spec_id, ItemSpecId};
/// #
/// let _snake = item_spec_id!("snake_case");
/// let _camel = item_spec_id!("camelCase");
/// let _pascal = item_spec_id!("PascalCase");
/// ```
#[derive(Clone, Debug, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub struct ItemSpecId(Cow<'static, str>);

crate::id_newtype!(ItemSpecId, ItemSpecIdInvalidFmt, item_spec_id);
