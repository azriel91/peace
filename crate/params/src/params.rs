use std::fmt::Debug;

use serde::{de::DeserializeOwned, Serialize};

/// Input parameters to an item spec.
///
/// This trait is automatically implemented by `#[derive(Params)]`.
pub trait Params {
    /// The `Params` type, but each field is wrapped with `ValueSpec<T>`.
    ///
    /// Specifies how to look up values for each field in the item spec's
    /// `Params`.
    type Spec: Clone + Debug + Serialize + DeserializeOwned + Send + Sync + 'static;
    /// The `Params` type, but with optional fields
    type Partial: Clone + Debug + Serialize + DeserializeOwned + Send + Sync + 'static;
}
