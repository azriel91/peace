/// Data stored by `CmdCtxBuilder` while building a
/// `CmdCtx<SingleProfileSingleFlow>`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SingleProfileSingleFlowBuilder<ProfileSelection, FlowIdSelection> {
    /// The profile this command operates on.
    pub(crate) profile_selection: ProfileSelection,
    /// Identifier or name of the chosen process flow.
    pub(crate) flow_id_selection: FlowIdSelection,
}
