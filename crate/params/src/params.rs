use std::fmt::Debug;

use serde::{de::DeserializeOwned, Serialize};

use crate::FieldWiseSpecRt;

/// Input parameters to an item spec.
///
/// This trait is automatically implemented by `#[derive(Value)]`.
pub trait Params {
    /// Convenience associated type for `ValueSpec<Self>`.
    type Spec: Clone + Debug + Serialize + DeserializeOwned + Send + Sync + 'static;
    /// The `Value` type, but with optional fields.
    type Partial: Clone + Debug + Default + Send + Sync + 'static;
    /// The `Value` type, but each field is wrapped with
    /// `ValueSpecFieldless<T>`.
    ///
    /// Specifies how to look up values for each field in the `Value`.
    type FieldWiseSpec: FieldWiseSpecRt<ValueType = Self, Partial = Self::Partial>
        + Clone
        + Debug
        + Serialize
        + DeserializeOwned
        + Send
        + Sync
        + 'static;

    /// Builder to return the `FieldWiseSpec` type.
    type FieldWiseBuilder;

    /// Returns a builder to construct the `FieldWise` spec.
    fn field_wise_spec() -> Self::FieldWiseBuilder;
}
