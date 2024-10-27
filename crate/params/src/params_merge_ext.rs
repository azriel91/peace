use crate::Params;

/// Trait for merging `ParamsPartial` onto a `Params` object.
///
/// This is automatically implemented by [`#[derive(Params)]`].
///
/// [`#[derive(Params)]`]: peace_params_derive::Params
pub trait ParamsMergeExt: Params {
    /// Moves the values from `Self::Partial` onto this `Params` object.
    fn merge(&mut self, params_partial: <Self as Params>::Partial);
}
