use std::fmt::Debug;

use peace_resources::{resources::ts::SetUp, Resources};

use crate::{ParamsResolveError, ValueResolutionCtx};

/// Runtime logic of how to look up values for each field in this struct.
///
/// This trait is automatically implemented by `#[derive(Params)]` on an
/// `ItemSpec::Params`, as well as manual implementations for standard library
/// types.
pub trait ValueSpecRt {
    /// The original value type. `MyParamsValueSpec::ValueType` is `MyParams`.
    type ValueType: Clone + Debug + Send + Sync + 'static;

    /// Resolves the value from resources.
    ///
    /// This function returns an error if any value is not present in
    /// [`Resources`]. For cases where missing values are not an error, see
    /// [`try_resolve`].
    ///
    /// [`try_resolve`]: Self::try_resolve
    fn resolve(
        &self,
        resources: &Resources<SetUp>,
        value_resolution_ctx: &mut ValueResolutionCtx,
    ) -> Result<Self::ValueType, ParamsResolveError>;

    /// Resolves the value from resources, returning `None` if it is not
    /// present.
    fn try_resolve(
        &self,
        resources: &Resources<SetUp>,
        value_resolution_ctx: &mut ValueResolutionCtx,
    ) -> Result<Option<Self::ValueType>, ParamsResolveError>;
}
