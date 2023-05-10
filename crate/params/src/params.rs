use std::fmt::Debug;

use serde::{de::DeserializeOwned, Serialize};

use crate::FieldWiseSpecRt;

/// Input parameters to an item spec.
///
/// This trait is automatically implemented by `#[derive(Params)]`.
pub trait Params {
    /// Convenience associated type for `ValueSpec<Self>`.
    type Spec: Clone + Debug + Serialize + DeserializeOwned + Send + Sync + 'static;
    /// The `Params` type, but with optional fields.
    type Partial: Clone + Debug + Default + Send + Sync + 'static;
    /// The `Params` type, but each field is wrapped with `ValueSpec<T>`.
    ///
    /// Specifies how to look up values for each field in the item spec's
    /// `Params`.
    type FieldWiseSpec: FieldWiseSpecRt<ValueType = Self, Partial = Self::Partial>
        + Clone
        + Debug
        + Serialize
        + DeserializeOwned
        + Send
        + Sync
        + 'static;
}
