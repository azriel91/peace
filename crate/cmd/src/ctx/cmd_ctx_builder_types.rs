use std::marker::PhantomData;

use peace_rt_model::params::{KeyUnknown, ParamsKeys, ParamsKeysImpl};

use crate::scopes::type_params::{
    FlowNotSelected, FlowParamsNone, ProfileNotSelected, ProfileParamsNone, WorkspaceParamsNone,
};

/// Trait so that a single type parameter can be used in `CmdCtx` and `Scopes`.
///
/// The associated types linked to the concrete type can all be queried through
/// this trait.
pub trait CmdCtxBuilderTypes {
    /// Error type of the automation software.
    type AppError;
    /// Output to write progress or outcome to.
    type Output;
    /// Parameter key types for workspace params, profile params, and flow
    /// params.
    type ParamsKeys: ParamsKeys;
    /// Whether workspace params keys have been selected.
    ///
    /// One of:
    ///
    /// * [`WorkspaceParamsNone`]
    /// * [`WorkspaceParamsSome`]
    ///
    /// [`WorkspaceParamsNone`]: crate::scopes::type_params::WorkspaceParamsNone
    /// [`WorkspaceParamsSome`]: crate::scopes::type_params::WorkspaceParamsSome
    type WorkspaceParamsSelection;
    /// Whether profile params keys have been selected.
    ///
    /// One of:
    ///
    /// * [`ProfileParamsNone`]
    /// * [`ProfileParamsSome`]
    /// * [`ProfileParamsSomeMulti`]
    ///
    /// Only applicable to `SingleProfile*` scopes.
    ///
    /// [`ProfileParamsNone`]: crate::scopes::type_params::ProfileParamsNone
    /// [`ProfileParamsSome`]: crate::scopes::type_params::ProfileParamsSome
    /// [`ProfileParamsSomeMulti`]: crate::scopes::type_params::ProfileParamsSomeMulti
    type ProfileParamsSelection;
    /// Whether flow params keys have been selected.
    ///
    /// One of:
    ///
    /// * [`FlowParamsNone`]
    /// * [`FlowParamsSome`]
    /// * [`FlowParamsSomeMulti`]
    ///
    /// Only applicable to `*SingleFlow` scopes.
    ///
    /// [`FlowParamsNone`]: crate::scopes::type_params::FlowParamsNone
    /// [`FlowParamsSome`]: crate::scopes::type_params::FlowParamsSome
    /// [`FlowParamsSomeMulti`]: crate::scopes::type_params::FlowParamsSomeMulti
    type FlowParamsSelection;
    /// The profile this command operates on.
    ///
    /// Only applicable to `SingleProfile*` scopes.
    type ProfileSelection;
    /// Identifier or name of the chosen process flow.
    ///
    /// Only applicable to `*SingleFlow` scopes.
    type FlowSelection;
}

/// Concrete struct to collect `CmdCtxBuilderTypes`.
#[derive(Debug)]
pub struct CmdCtxBuilderTypesCollector<
    AppError,
    Output,
    ParamsKeys,
    WorkspaceParamsSelection,
    ProfileParamsSelection,
    FlowParamsSelection,
    ProfileSelection,
    FlowSelection,
>(
    #[allow(clippy::type_complexity)]
    pub  PhantomData<(
        AppError,
        Output,
        ParamsKeys,
        WorkspaceParamsSelection,
        ProfileParamsSelection,
        FlowParamsSelection,
        ProfileSelection,
        FlowSelection,
    )>,
);

/// `CmdCtxBuilderTypesCollector` with `Output` and `AppError` needing to
/// be specified.
///
/// The remainder of the type arguments use *none* type values.
pub type CmdCtxTypesCollectorEmpty<AppError, Output> = CmdCtxBuilderTypesCollector<
    AppError,
    Output,
    ParamsKeysImpl<KeyUnknown, KeyUnknown, KeyUnknown>,
    WorkspaceParamsNone,
    ProfileParamsNone,
    FlowParamsNone,
    ProfileNotSelected,
    FlowNotSelected,
>;

impl<
        AppError,
        Output,
        ParamsKeysT,
        WorkspaceParamsSelection,
        ProfileParamsSelection,
        FlowParamsSelection,
        ProfileSelection,
        FlowSelection,
    > CmdCtxBuilderTypes
    for CmdCtxBuilderTypesCollector<
        AppError,
        Output,
        ParamsKeysT,
        WorkspaceParamsSelection,
        ProfileParamsSelection,
        FlowParamsSelection,
        ProfileSelection,
        FlowSelection,
    >
where
    ParamsKeysT: ParamsKeys,
{
    type AppError = AppError;
    type FlowParamsSelection = FlowParamsSelection;
    type FlowSelection = FlowSelection;
    type Output = Output;
    type ParamsKeys = ParamsKeysT;
    type ProfileParamsSelection = ProfileParamsSelection;
    type ProfileSelection = ProfileSelection;
    type WorkspaceParamsSelection = WorkspaceParamsSelection;
}
