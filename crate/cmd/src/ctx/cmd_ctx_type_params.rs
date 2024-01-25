use std::{fmt::Debug, marker::PhantomData};

use peace_rt_model::{output::OutputWrite, params::ParamsKeys};
use peace_value_traits::AppError;

/// Trait so that a single type parameter can be used in `CmdCtx` and `Scopes`.
///
/// The associated types linked to the concrete type can all be queried through
/// this trait.
pub trait CmdCtxTypeParams {
    /// Error type of the automation software.
    type AppError: Debug;
    /// Output to write progress or outcome to.
    type Output;
    /// Parameter key types for workspace params, profile params, and flow
    /// params.
    type ParamsKeys: ParamsKeys;
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
    > + Debug
    + Unpin
    + 'static
{
    /// Error type of the automation software.
    type AppError: AppError + From<peace_rt_model::Error>;
    /// Output to write progress or outcome to.
    type Output: OutputWrite<<Self as CmdCtxTypeParamsConstrained>::AppError> + 'static;
    /// Parameter key types for workspace params, profile params, and flow
    /// params.
    type ParamsKeys: ParamsKeys;
}

impl<T> CmdCtxTypeParamsConstrained for T
where
    T: CmdCtxTypeParams + Debug + Unpin + 'static,
    T::AppError: AppError + From<peace_rt_model::Error>,
    T::Output: OutputWrite<T::AppError> + 'static,
    T::ParamsKeys: ParamsKeys,
{
    type AppError = T::AppError;
    type Output = T::Output;
    type ParamsKeys = T::ParamsKeys;
}

/// Concrete struct to collect `CmdCtxTypeParams`.
#[derive(Debug)]
pub struct CmdCtxTypeParamsCollector<AppError, Output, ParamsKeys>(
    pub PhantomData<(AppError, Output, ParamsKeys)>,
);

impl<AppError, Output, ParamsKeysT> CmdCtxTypeParams
    for CmdCtxTypeParamsCollector<AppError, Output, ParamsKeysT>
where
    AppError: Debug,
    ParamsKeysT: ParamsKeys,
{
    type AppError = AppError;
    type Output = Output;
    type ParamsKeys = ParamsKeysT;
}
