use peace_resources::{resources::ts::SetUp, Resources};

use crate::ParamsResolveError;

/// Specifies how to look up values for each field in the item spec's `Params`.
///
/// This trait is automatically implemented by `#[derive(Params)]`.
///
/// # Design
///
/// Nested parameters are not yet supported, though it is a possible improvement
/// by adding a `#[nested_param]` attribute to a field.
pub trait ParamsSpec {
    /// The `Params` type.
    type Params;
    /// The `Params` type, but with optional fields
    type Partial;

    /// Resolves the values to construct the item spec `Params`.
    ///
    /// This function returns an error if any value is not present in
    /// [`Resources`]. For cases where missing values are not an error, see
    /// [`resolve_partial`].
    ///
    /// [`resolve_partial`]: Self::resolve_partial
    fn resolve(&self, resources: &Resources<SetUp>) -> Result<Self::Params, ParamsResolveError>;
    /// Resolves the values to construct the item spec `Params`.
    ///
    /// Values that are not present in `Resources` will be `None`.
    fn resolve_partial(&self, resources: &Resources<SetUp>) -> Self::Partial;
}
