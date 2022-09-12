use std::borrow::Cow;

use serde::{Deserialize, Serialize};

/// Unique identifier for a `FlowId`, `Cow<'static, str>` newtype.
///
/// Must begin with a letter or underscore, and contain only letters, numbers,
/// and underscores.
///
/// # Examples
///
/// The following are all examples of valid `FlowId`s:
///
/// ```rust
/// # use peace_core::{flow_id, FlowId};
/// #
/// let _snake = flow_id!("snake_case");
/// let _camel = flow_id!("camelCase");
/// let _pascal = flow_id!("PascalCase");
/// ```
#[derive(Clone, Debug, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub struct FlowId(Cow<'static, str>);

crate::id_newtype!(FlowId, FlowIdInvalidFmt, flow_id);
