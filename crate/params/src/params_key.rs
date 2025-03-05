use std::{fmt::Debug, hash::Hash};

use serde::{de::DeserializeOwned, Serialize};

/// Marker trait for a parameter key type.
///
/// This trait is automatically implemented for types that are `Copy + Debug +
/// Eq + Hash
/// + Deserialize + Serialize`.
///
/// # Examples
///
/// ```rust,ignore
/// use serde::{Deserialize, Serialize};
///
/// #[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize, Serialize)]
/// pub enum WorkspaceParam {
///     UserEmail,
///     Profile,
/// }
///
/// impl CmdCtxTypes for MyCmdCtxTypes {
///     // ..
///     type WorkspaceParamsKey = WorkspaceParam;
/// }
/// ```
pub trait ParamsKey:
    Copy + Debug + Eq + Hash + DeserializeOwned + Serialize + Send + Sync + 'static
{
}

impl<T> ParamsKey for T where
    T: Copy + Debug + Eq + Hash + DeserializeOwned + Serialize + Send + Sync + 'static
{
}
