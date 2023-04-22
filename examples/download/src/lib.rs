use peace::{
    cfg::{app_name, item_spec_id, AppName, FlowId, ItemSpecId, Profile},
    cmd::{ctx::CmdCtx, scopes::SingleProfileSingleFlow},
    resources::resources::ts::SetUp,
    rt::cmds::{
        sub::StatesSavedReadCmd, CleanCmd, DiffCmd, EnsureCmd, StatesDesiredDisplayCmd,
        StatesDiscoverCmd, StatesSavedDisplayCmd,
    },
    rt_model::{
        output::OutputWrite,
        params::{KeyUnknown, ParamsKeysImpl},
        Flow, ItemSpecGraphBuilder, Workspace, WorkspaceSpec,
    },
};
use peace_item_specs::file_download::{FileDownloadItemSpec, FileDownloadParams};

#[cfg(not(target_arch = "wasm32"))]
pub use crate::download_args::{DownloadArgs, DownloadCommand};
pub use crate::{download_error::DownloadError, file_id::FileId};

#[cfg(not(target_arch = "wasm32"))]
mod download_args;
mod download_error;
mod file_id;

#[cfg(target_arch = "wasm32")]
mod wasm;

#[derive(Debug)]
pub struct WorkspaceAndFlow {
    workspace: Workspace,
    flow: Flow<DownloadError>,
}

const FILE_ITEM_SPEC_ID: ItemSpecId = item_spec_id!("file");

/// Returns a default workspace and the Download item spec graph.
#[cfg(not(target_arch = "wasm32"))]
pub async fn workspace_and_flow_setup(
    workspace_spec: WorkspaceSpec,
    flow_id: FlowId,
) -> Result<WorkspaceAndFlow, DownloadError> {
    let workspace = Workspace::new(app_name!(), workspace_spec)?;

    let item_spec_graph = {
        let mut item_spec_graph_builder = ItemSpecGraphBuilder::<DownloadError>::new();
        item_spec_graph_builder
            .add_fn(FileDownloadItemSpec::<FileId>::new(FILE_ITEM_SPEC_ID).into());
        item_spec_graph_builder.build()
    };
    let flow = Flow::new(flow_id, item_spec_graph);

    let workspace_and_flow = WorkspaceAndFlow { workspace, flow };
    Ok(workspace_and_flow)
}

/// Returns a default workspace and the Download item spec graph.
#[cfg(target_arch = "wasm32")]
pub async fn workspace_and_flow_setup(
    workspace_spec: WorkspaceSpec,
    flow_id: FlowId,
) -> Result<WorkspaceAndFlow, DownloadError> {
    let workspace = Workspace::new(app_name!(), workspace_spec)?;
    let item_spec_graph = {
        let mut item_spec_graph_builder = ItemSpecGraphBuilder::<DownloadError>::new();
        item_spec_graph_builder
            .add_fn(FileDownloadItemSpec::<FileId>::new(item_spec_id!("file")).into());
        item_spec_graph_builder.build()
    };
    let flow = Flow::new(flow_id, item_spec_graph);

    let workspace_and_flow = WorkspaceAndFlow { workspace, flow };
    Ok(workspace_and_flow)
}

pub type DownloadCmdCtx<'ctx, O> = CmdCtx<
    SingleProfileSingleFlow<
        'ctx,
        DownloadError,
        O,
        ParamsKeysImpl<KeyUnknown, KeyUnknown, KeyUnknown>,
        SetUp,
    >,
>;

/// Returns a `CmdCtx` initialized from the workspace and item spec graph
pub async fn cmd_ctx<'ctx, O>(
    workspace_and_flow: &'ctx WorkspaceAndFlow,
    profile: Profile,
    output: &'ctx mut O,
    file_download_params: Option<FileDownloadParams<FileId>>,
) -> Result<DownloadCmdCtx<'ctx, O>, DownloadError>
where
    O: OutputWrite<DownloadError>,
{
    let WorkspaceAndFlow { workspace, flow } = workspace_and_flow;
    let mut cmd_ctx_builder = CmdCtx::builder_single_profile_single_flow(output, workspace)
        .with_profile(profile)
        .with_flow(flow);

    if let Some(file_download_params) = file_download_params {
        cmd_ctx_builder = cmd_ctx_builder.with_item_spec_params::<FileDownloadItemSpec<FileId>>(
            FILE_ITEM_SPEC_ID,
            file_download_params,
        );
    }

    cmd_ctx_builder.await
}

pub async fn fetch<O>(cmd_ctx: &mut DownloadCmdCtx<'_, O>) -> Result<(), DownloadError>
where
    O: OutputWrite<DownloadError>,
{
    let (_states_current, _states_desired) =
        StatesDiscoverCmd::current_and_desired(cmd_ctx).await?;
    Ok(())
}

pub async fn status<O>(cmd_ctx: &mut DownloadCmdCtx<'_, O>) -> Result<(), DownloadError>
where
    O: OutputWrite<DownloadError>,
{
    // Already displayed by the command
    let _states_saved = StatesSavedDisplayCmd::exec(cmd_ctx).await?;
    Ok(())
}

pub async fn desired<O>(cmd_ctx: &mut DownloadCmdCtx<'_, O>) -> Result<(), DownloadError>
where
    O: OutputWrite<DownloadError>,
{
    // Already displayed by the command
    let _states_desired = StatesDesiredDisplayCmd::exec(cmd_ctx).await?;
    Ok(())
}

pub async fn diff<O>(cmd_ctx: &mut DownloadCmdCtx<'_, O>) -> Result<(), DownloadError>
where
    O: OutputWrite<DownloadError>,
{
    let states_diff = DiffCmd::current_and_desired(cmd_ctx).await?;
    cmd_ctx.output_mut().present(&states_diff).await?;
    Ok(())
}

pub async fn ensure_dry<O>(cmd_ctx: &mut DownloadCmdCtx<'_, O>) -> Result<(), DownloadError>
where
    O: OutputWrite<DownloadError>,
{
    let states_saved = StatesSavedReadCmd::exec(cmd_ctx).await?;
    let states_ensured_dry_outcome = EnsureCmd::exec_dry(cmd_ctx, &states_saved).await?;
    let states_ensured_dry = &states_ensured_dry_outcome.value;
    cmd_ctx.output_mut().present(states_ensured_dry).await?;
    Ok(())
}

pub async fn ensure<O>(cmd_ctx: &mut DownloadCmdCtx<'_, O>) -> Result<(), DownloadError>
where
    O: OutputWrite<DownloadError>,
{
    let states_saved = StatesSavedReadCmd::exec(cmd_ctx).await?;
    let states_ensured_outcome = EnsureCmd::exec(cmd_ctx, &states_saved).await?;
    let states_ensured = &states_ensured_outcome.value;
    cmd_ctx.output_mut().present(states_ensured).await?;
    Ok(())
}

pub async fn clean_dry<O>(cmd_ctx: &mut DownloadCmdCtx<'_, O>) -> Result<(), DownloadError>
where
    O: OutputWrite<DownloadError>,
{
    let states_saved = StatesSavedReadCmd::exec(cmd_ctx).await?;
    let states_cleaned_dry_outcome = CleanCmd::exec_dry(cmd_ctx, &states_saved).await?;
    let states_cleaned_dry = &states_cleaned_dry_outcome.value;
    cmd_ctx.output_mut().present(states_cleaned_dry).await?;
    Ok(())
}

pub async fn clean<O>(cmd_ctx: &mut DownloadCmdCtx<'_, O>) -> Result<(), DownloadError>
where
    O: OutputWrite<DownloadError>,
{
    let states_saved = StatesSavedReadCmd::exec(cmd_ctx).await?;
    let states_cleaned_outcome = CleanCmd::exec(cmd_ctx, &states_saved).await?;
    let states_cleaned = &states_cleaned_outcome.value;
    cmd_ctx.output_mut().present(states_cleaned).await?;
    Ok(())
}
