use peace::{
    cfg::app_name,
    cmd::ctx::CmdCtx,
    rt_model::{output::OutputWrite, Workspace, WorkspaceSpec},
};

use crate::model::{EnvManError, EnvManFlow, WorkspaceParamsKey};

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
    O: OutputWrite<EnvManError>,
{
    let cmd_ctx_builder =
        CmdCtx::builder_no_profile_no_flow::<EnvManError, O>(output.into(), workspace.into());
    crate::cmds::ws_params_augment!(cmd_ctx_builder);
    let cmd_ctx = cmd_ctx_builder.await?;
    Ok(cmd_ctx
        .workspace_params()
        .get(&WorkspaceParamsKey::Flow)
        .copied()
        .expect("Expected Flow to be set for this workspace."))
}
