use std::fmt::Debug;

use peace_resources::{resources::ts::SetUp, Resources};
use serde::{de::DeserializeOwned, Serialize};

use crate::{AnySpecRt, ParamsResolveError, ValueResolutionCtx};

/// Runtime logic of how to look up values for each field in this struct.
///
/// This trait is automatically implemented by `#[derive(Params)]` on an
/// `Item::Params`, as well as manual implementations for standard library
/// types.
pub trait FieldWiseSpecRt: AnySpecRt {
    /// The original value type. `MyParamsFieldWiseSpec::ValueType` is
    /// `MyParams`.
    type ValueType: Clone + Debug + Serialize + DeserializeOwned + Send + Sync + 'static;
    /// The `Params` type, but with each of its fields wrapped in `Option`.
    type Partial: Clone + Debug + Default + Serialize + DeserializeOwned + Send + Sync + 'static;

    /// Resolves the values to construct the item `Params`.
    ///
    /// This function returns an error if any value is not present in
    /// [`Resources`]. For cases where missing values are not an error, see
    /// [`resolve_partial`].
    ///
    /// [`resolve_partial`]: Self::resolve_partial
    fn resolve(
        &self,
        resources: &Resources<SetUp>,
        value_resolution_ctx: &mut ValueResolutionCtx<ItemIdT>,
    ) -> Result<Self::ValueType, ParamsResolveError<ItemIdT>>;
    /// Resolves the values to construct the item `Params`.
    ///
    /// Values that are not present in `Resources` will be `None`.
    fn resolve_partial(
        &self,
        resources: &Resources<SetUp>,
        value_resolution_ctx: &mut ValueResolutionCtx<ItemIdT>,
    ) -> Result<Self::Partial, ParamsResolveError<ItemIdT>>;
}
