use std::{fmt::Debug, hash::Hash};

use serde::{de::DeserializeOwned, Serialize};

/// Marker trait for a parameter key type.
///
/// This trait is automatically implemented for types that are `Clone + Debug +
/// Eq + Hash + Deserialize + Serialize`.
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
    Clone + Debug + Eq + Hash + DeserializeOwned + Serialize + Send + Sync + 'static
{
}

impl<T> ParamsKey for T where
    T: Clone + Debug + Eq + Hash + DeserializeOwned + Serialize + Send + Sync + 'static
{
}
