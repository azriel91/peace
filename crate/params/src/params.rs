use std::fmt::Debug;

use serde::{de::DeserializeOwned, Serialize};

use crate::ParamsSpecBuilder;

/// Input parameters to an item spec.
///
/// This trait is automatically implemented by `#[derive(Params)]`.
pub trait Params {
    type Spec: Clone + Debug + Serialize + Send + Sync + 'static;
    type SpecBuilder: ParamsSpecBuilder<Output = Self::Spec>;
    type Partial: Clone + Debug + Serialize + DeserializeOwned + Send + Sync + 'static;
}
