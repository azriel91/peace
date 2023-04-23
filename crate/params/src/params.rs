use std::fmt::Debug;

use crate::ParamsSpecBuilder;

/// Input parameters to an item spec.
///
/// This trait is automatically implemented by `#[derive(Params)]`.
///
/// # Design
///
/// We can't constrain `Spec` or `Partial with `Clone + Serialize +
/// DeserializeOwned` yet, because we want to support `ValueSpec::FromMap`,
/// which is stored as a `Box<dyn Fn(..)`.
pub trait Params {
    type Spec: Debug + Send + Sync + 'static;
    type SpecBuilder: ParamsSpecBuilder<Output = Self::Spec>;
    type Partial: Debug + Send + Sync + 'static;
}
