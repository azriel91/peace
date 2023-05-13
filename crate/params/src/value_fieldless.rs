use std::fmt::Debug;

use serde::{de::DeserializeOwned, Serialize};

/// Field of an `ItemSpec::Params`.
///
/// This trait is automatically implemented by `#[derive(Value)]`.
///
/// This is *like* the [`Params`] trait, except it does not have the `FieldWise`
/// resolution functionality.
///
/// [`Params`]: crate::Params
pub trait ValueFieldless {
    /// Convenience associated type for `ValueSpec<Self>`.
    type Spec: Clone + Debug + Serialize + DeserializeOwned + Send + Sync + 'static;
    /// The `Value` type, but with optional fields.
    type Partial: Clone + Debug + Default + Send + Sync + 'static;
}
