/// Trait so that a single type parameter can be used in `CmdCtx` and `Scopes`.
///
/// The associated types linked to the concrete type can all be queried through
/// this trait.
pub trait CmdCtxBuilderTypeParams {
    /// Whether the command works with zero, one, or multiple profiles, and zero
    /// or one flow.
    type ScopeBuilder;
    /// Output to write progress or outcome to.
    type Output;
    /// Error type of the automation software.
    type AppError;
    /// Parameter key types for workspace params, profile params, and flow
    /// params.
    type ParamsKeys;
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
