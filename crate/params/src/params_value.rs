use std::fmt::Debug;

use serde::{de::DeserializeOwned, Serialize};

/// Marker trait for a parameter value type.
///
/// This trait is automatically implemented for types that are `Clone + Debug +
/// DeserializeOwned + Serialize + Send + Sync + 'static`.
pub trait ParamsValue:
    Clone + Debug + PartialEq + DeserializeOwned + Serialize + Send + Sync + 'static
{
}

impl<T> ParamsValue for T where
    T: Clone + Debug + PartialEq + DeserializeOwned + Serialize + Send + Sync + 'static
{
}
