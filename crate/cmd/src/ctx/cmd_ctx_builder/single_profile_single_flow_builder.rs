/// Data stored by `CmdCtxBuilder` while building a
/// `CmdCtx<SingleProfileSingleFlow>`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SingleProfileSingleFlowBuilder<
    ProfileSelection,
    FlowIdSelection,
    WorkspaceParamsSelection,
> {
    /// The profile this command operates on.
    pub(crate) profile_selection: ProfileSelection,
    /// Identifier or name of the chosen process flow.
    pub(crate) flow_id_selection: FlowIdSelection,
    /// Workspace parameters.
    pub(crate) workspace_params_selection: WorkspaceParamsSelection,
}
