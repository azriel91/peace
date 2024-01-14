use peace_rt_model::{output::OutputWrite, params::ParamsKeys};
use peace_value_traits::AppError;

/// Trait so that a single type parameter can be used in `CmdCtx` and `Scopes`.
///
/// The associated types linked to the concrete type can all be queried through
/// this trait.
pub trait CmdCtxTypeParams {
    /// Whether the command works with zero, one, or multiple profiles, and zero
    /// or one flow.
    type Scope;
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
pub trait CmdCtxTypeParamsConstrained {
    /// Whether the command works with zero, one, or multiple profiles, and zero
    /// or one flow.
    type Scope;
    /// Output to write progress or outcome to.
    type Output: OutputWrite<Self::AppError> + 'static;
    /// Error type of the automation software.
    type AppError: AppError;
    /// Parameter key types for workspace params, profile params, and flow
    /// params.
    type ParamsKeys: ParamsKeys;
}
