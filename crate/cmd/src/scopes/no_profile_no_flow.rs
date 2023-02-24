use peace_rt_model::cmd_context_params::{KeyMaybe, ParamsKeys, WorkspaceParams};

/// A command that only works with workspace parameters.
///
/// ```bash
/// path/to/repo/.peace/envman
/// |- 📝 workspace_params.yaml    # ✅ can read or write `WorkspaceParams`
/// |
/// |- 🌏 ..                       # ❌ cannot read or write `Profile` information
/// ```
///
/// ## Capabilities
///
/// This kind of command can:
///
/// * Read or write workspace parameters.
///
/// This kind of command cannot:
///
/// * Read or write profile parameters -- see `SingleProfileNoFlow` or
///   `MultiProfileNoFlow`.
/// * Read or write flow parameters -- see `MultiProfileNoFlow`.
/// * Read or write flow state -- see `SingleProfileSingleFlow` or
///   `MultiProfileSingleFlow`.
#[derive(Debug)]
pub struct NoProfileNoFlow<PKeys>
where
    PKeys: ParamsKeys + 'static,
{
    /// Workspace params.
    workspace_params: WorkspaceParams<<PKeys::WorkspaceParamsKMaybe as KeyMaybe>::Key>,
}

impl<PKeys> NoProfileNoFlow<PKeys>
where
    PKeys: ParamsKeys + 'static,
{
    pub fn new(
        workspace_params: WorkspaceParams<<PKeys::WorkspaceParamsKMaybe as KeyMaybe>::Key>,
    ) -> Self {
        Self { workspace_params }
    }
}
