use peace::{
    cfg::app_name,
    cmd_ctx::{CmdCtxSpnf, CmdCtxSpsf, ProfileSelection},
    flow_rt::Flow,
    rt_model::{output::OutputWrite, Workspace, WorkspaceSpec},
};

use crate::{
    model::{EnvManError, WorkspaceParamsKey},
    rt_model::EnvmanCmdCtxTypes,
};

/// Alias to simplify naming the `CmdCtx` type.
pub type EnvManCmdCtx<'ctx, O> = CmdCtxSpsf<'ctx, EnvmanCmdCtxTypes<O>>;

/// Alias to simplify naming the `CmdCtx` type.
#[allow(dead_code)] // TODO: maybe remove after refactoring cmd_ctx
pub type EnvManCmdCtx1p0f<'ctx, O> = CmdCtxSpnf<'ctx, EnvmanCmdCtxTypes<O>>;

/// Returns a `CmdCtx<SingleProfileSingleFlow<'ctx,
/// EnvmanCmdCtxTypes<O>>>`.
#[allow(dead_code)] // TODO: maybe remove after refactoring cmd_ctx
pub async fn envman_cmd_ctx_1p1f<O>(
    output: &mut O,
    flow: Flow<EnvManError>,
) -> Result<EnvManCmdCtx<'_, O>, EnvManError>
where
    O: OutputWrite,
    EnvManError: From<<O as OutputWrite>::Error>,
{
    let workspace = Workspace::new(
        app_name!(),
        #[cfg(not(target_arch = "wasm32"))]
        WorkspaceSpec::WorkingDir,
        #[cfg(target_arch = "wasm32")]
        WorkspaceSpec::SessionStorage,
    )?;

    let profile_key = WorkspaceParamsKey::Profile;
    CmdCtxSpsf::<EnvmanCmdCtxTypes<O>>::builder()
        .with_output(output.into())
        .with_workspace(workspace.into())
        .with_profile_selection(ProfileSelection::FromWorkspaceParam(profile_key.into()))
        .with_flow(flow.into())
        .await
}

/// Returns a `CmdCtx<SingleProfileSingleFlow<'ctx,
/// EnvmanCmdCtxTypes<Output>>>`.
#[allow(dead_code)] // TODO: maybe remove after refactoring cmd_ctx
pub async fn envman_cmd_ctx_1p0f<O>(output: &mut O) -> Result<EnvManCmdCtx1p0f<'_, O>, EnvManError>
where
    O: OutputWrite,
    EnvManError: From<<O as OutputWrite>::Error>,
{
    let workspace = Workspace::new(
        app_name!(),
        #[cfg(not(target_arch = "wasm32"))]
        WorkspaceSpec::WorkingDir,
        #[cfg(target_arch = "wasm32")]
        WorkspaceSpec::SessionStorage,
    )?;

    let cmd_ctx_builder = CmdCtxSpnf::<EnvmanCmdCtxTypes<O>>::builder()
        .with_output(output.into())
        .with_workspace(workspace.into());

    let profile_key = WorkspaceParamsKey::Profile;
    cmd_ctx_builder
        .with_profile_selection(ProfileSelection::FromWorkspaceParam(profile_key.into()))
        .await
}
