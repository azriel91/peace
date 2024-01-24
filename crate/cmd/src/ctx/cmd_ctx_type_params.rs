use std::{fmt::Debug, marker::PhantomData};

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
    type AppError: Debug;
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
    /// Output to write progress or outcome to.
    type Output: OutputWrite<<Self as CmdCtxTypeParamsConstrained>::AppError> + 'static;
    /// Error type of the automation software.
    type AppError: AppError + From<peace_rt_model::Error>;
    /// Parameter key types for workspace params, profile params, and flow
    /// params.
    type ParamsKeys: ParamsKeys;
}

impl<T> CmdCtxTypeParamsConstrained for T
where
    T: CmdCtxTypeParams + Debug + Unpin + 'static,
    T::AppError: AppError + From<peace_rt_model::Error>,
    T::ParamsKeys: ParamsKeys,
    T::Output: OutputWrite<T::AppError> + 'static,
{
    type AppError = T::AppError;
    type Output = T::Output;
    type ParamsKeys = T::ParamsKeys;
}

/// Concrete struct to collect `CmdCtxTypeParams`.
#[derive(Debug)]
pub struct CmdCtxTypeParamsCollector<Output, AppError, ParamsKeys>(
    pub PhantomData<(Output, AppError, ParamsKeys)>,
);

impl<Output, AppError, ParamsKeysT> CmdCtxTypeParams
    for CmdCtxTypeParamsCollector<Output, AppError, ParamsKeysT>
where
    AppError: Debug,
    ParamsKeysT: ParamsKeys,
{
    type AppError = AppError;
    type Output = Output;
    type ParamsKeys = ParamsKeysT;
}
