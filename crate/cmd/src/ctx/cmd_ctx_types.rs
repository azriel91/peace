use std::{fmt::Debug, marker::PhantomData};

use peace_rt_model::{output::OutputWrite, params::ParamsKeys};
use peace_value_traits::AppError;

/// Trait so that a single type parameter can be used in `CmdCtx` and `Scopes`.
///
/// The associated types linked to the concrete type can all be queried through
/// this trait.
pub trait CmdCtxTypes {
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
pub trait CmdCtxTypesConstrained:
    CmdCtxTypes<
        AppError = <Self as CmdCtxTypesConstrained>::AppError,
        Output = <Self as CmdCtxTypesConstrained>::Output,
        ParamsKeys = <Self as CmdCtxTypesConstrained>::ParamsKeys,
    > + Debug
    + Unpin
{
    /// Error type of the automation software.
    type AppError: AppError + From<peace_rt_model::Error>;
    /// Output to write progress or outcome to.
    type Output: OutputWrite;
    /// Parameter key types for workspace params, profile params, and flow
    /// params.
    type ParamsKeys: ParamsKeys;
}

impl<T> CmdCtxTypesConstrained for T
where
    T: CmdCtxTypes + Debug + Unpin,
    T::AppError: AppError + From<peace_rt_model::Error>,
    T::Output: OutputWrite,
    T::ParamsKeys: ParamsKeys,
{
    type AppError = T::AppError;
    type Output = T::Output;
    type ParamsKeys = T::ParamsKeys;
}

/// Concrete struct to collect `CmdCtxTypes`.
#[derive(Debug)]
pub struct CmdCtxTypesCollector<AppError, Output, ParamsKeys>(
    pub PhantomData<(AppError, Output, ParamsKeys)>,
);

impl<AppError, Output, ParamsKeysT> CmdCtxTypes
    for CmdCtxTypesCollector<AppError, Output, ParamsKeysT>
where
    AppError: Debug,
    ParamsKeysT: ParamsKeys,
{
    type AppError = AppError;
    type Output = Output;
    type ParamsKeys = ParamsKeysT;
}
