use peace::{
    cfg::app_name,
    cmd_ctx::CmdCtxNpnf,
    rt_model::{output::OutputWrite, Workspace, WorkspaceSpec},
};

use crate::{
    model::{EnvManError, EnvManFlow, WorkspaceParamsKey},
    rt_model::EnvmanCmdCtxTypes,
};

/// Returns the `Workspace` for all commands.
pub fn workspace() -> Result<Workspace, EnvManError> {
    Ok(Workspace::new(
        app_name!(),
        #[cfg(not(target_arch = "wasm32"))]
        WorkspaceSpec::WorkingDir,
        #[cfg(target_arch = "wasm32")]
        WorkspaceSpec::SessionStorage,
    )?)
}

// Reads the `EnvManFlow` used for this workspace.
pub async fn env_man_flow<O>(
    output: &mut O,
    workspace: &Workspace,
) -> Result<EnvManFlow, EnvManError>
where
    O: OutputWrite,
    EnvManError: From<<O as OutputWrite>::Error>,
{
    let cmd_ctx = CmdCtxNpnf::<EnvmanCmdCtxTypes<O>>::builder()
        .with_output(output.into())
        .with_workspace(workspace.into())
        .await?;
    Ok(cmd_ctx
        .fields()
        .workspace_params()
        .get(&WorkspaceParamsKey::Flow)
        .copied()
        .expect("Expected Flow to be set for this workspace."))
}
