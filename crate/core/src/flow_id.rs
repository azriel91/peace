use std::borrow::Cow;

use peace_static_check_macros::flow_id;
use serde::{Deserialize, Serialize};

/// Identifier or name of a process flow.
///
/// Examples are `"dev_env"` and `"artifact"` in the following snippet:
///
/// ```bash
/// peace dev_env discover # StatesDiscoverCmd
/// peace dev_env status   # StatesSavedDisplayCmd
/// peace dev_env deploy   # EnsureCmd
/// peace dev_env clean    # CleanCmd
///
/// peace artifact discover # StatesDiscoverCmd
/// peace artifact status   # StatesSavedDisplayCmd
/// peace artifact publish  # EnsureCmd
/// ```
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

impl FlowId {
    /// Flow ID used by the Peace framework when a command is for initializing a
    /// workspace.
    pub const fn workspace_init() -> Self {
        flow_id!("workspace_init")
    }

    /// Flow ID used by the Peace framework when a command is for initializing a
    /// profile.
    pub const fn profile_init() -> Self {
        flow_id!("profile_init")
    }
}
