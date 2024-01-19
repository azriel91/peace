use peace_rt_model::{output::OutputWrite, params::ParamsKeys};
use peace_value_traits::AppError;

/// Trait so that a single type parameter can be used in `CmdCtx` and `Scopes`.
///
/// The associated types linked to the concrete type can all be queried through
/// this trait.
pub trait CmdCtxTypeParams {
    /// Output to write progress or outcome to.
    type Output;
    /// Error type of the automation software.
    type AppError;
    /// Parameter key types for workspace params, profile params, and flow
    /// params.
    type ParamsKeys;
}

/// Trait so that a single type parameter can be used in `CmdCtx` and `Scopes`.
///
/// The associated types linked to the concrete type can all be queried through
/// this trait.
pub trait CmdCtxTypeParamsConstrained:
    CmdCtxTypeParams<
        AppError = <Self as CmdCtxTypeParamsConstrained>::AppError,
        Output = <Self as CmdCtxTypeParamsConstrained>::Output,
        ParamsKeys = <Self as CmdCtxTypeParamsConstrained>::ParamsKeys,
    >
{
    /// Output to write progress or outcome to.
    type Output: OutputWrite<<Self as CmdCtxTypeParamsConstrained>::AppError> + 'static;
    /// Error type of the automation software.
    type AppError: AppError;
    /// Parameter key types for workspace params, profile params, and flow
    /// params.
    type ParamsKeys: ParamsKeys;
}

impl<T> CmdCtxTypeParamsConstrained for T
where
    T: CmdCtxTypeParams,
    T::AppError: AppError,
    T::ParamsKeys: ParamsKeys,
    T::Output: OutputWrite<T::AppError> + 'static,
{
    type AppError = T::AppError;
    type Output = T::Output;
    type ParamsKeys = T::ParamsKeys;
}
