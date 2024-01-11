/// Trait so that a single type parameter can be used in `CmdCtx` and `Scopes`.
///
/// The associated types linked to the concrete type can all be queried through
/// this trait.
pub trait CmdCtxTypeParams {
    /// Output to write progress or outcome to.
    type Output;
    /// Error type of the automation software.
    type AppError;
    /// Workspace parameters key type.
    type WorkspaceParamsKey;
    /// Profile parameters key type.
    type ProfileParamsKey;
    /// Flow parameters key type.
    type FlowParamsKey;
}
