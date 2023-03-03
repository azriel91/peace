use peace::{
    cfg::{app_name, item_spec_id, AppName, FlowId, ItemSpecId, Profile},
    cmd::{ctx::CmdCtx, scopes::SingleProfileSingleFlow},
    resources::resources::ts::SetUp,
    rt::cmds::{
        CleanCmd, DiffCmd, EnsureCmd, StatesDesiredDisplayCmd, StatesDiscoverCmd,
        StatesSavedDisplayCmd,
    },
    rt_model::{
        output::OutputWrite,
        params::{KeyKnown, KeyUnknown, ParamsKeysImpl},
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
            .add_fn(FileDownloadItemSpec::<FileId>::new(item_spec_id!("file")).into());
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
        ParamsKeysImpl<KeyUnknown, KeyKnown<String>, KeyUnknown>,
        SetUp,
    >,
>;

/// Returns a `CmdCtx` initialized from the workspace and item spec graph
pub async fn cmd_context<'ctx, O>(
    workspace_and_flow: &'ctx WorkspaceAndFlow,
    profile: Profile,
    output: &'ctx mut O,
    file_download_params: Option<FileDownloadParams<FileId>>,
) -> Result<DownloadCmdCtx<'ctx, O>, DownloadError>
where
    O: OutputWrite<DownloadError>,
{
    let WorkspaceAndFlow { workspace, flow } = workspace_and_flow;
    CmdCtx::builder_single_profile_single_flow(output, workspace)
        .with_profile(profile)
        .with_flow(flow)
        .with_profile_param_value("file_download_params".to_string(), file_download_params)
        .await
}

pub async fn fetch<O>(cmd_context: DownloadCmdCtx<'_, O>) -> Result<(), DownloadError>
where
    O: OutputWrite<DownloadError>,
{
    StatesDiscoverCmd::exec(cmd_context).await?;
    Ok(())
}

pub async fn status<O>(cmd_context: DownloadCmdCtx<'_, O>) -> Result<(), DownloadError>
where
    O: OutputWrite<DownloadError>,
{
    StatesSavedDisplayCmd::exec(cmd_context).await?;
    Ok(())
}

pub async fn desired<O>(cmd_context: DownloadCmdCtx<'_, O>) -> Result<(), DownloadError>
where
    O: OutputWrite<DownloadError>,
{
    StatesDesiredDisplayCmd::exec(cmd_context).await?;
    Ok(())
}

pub async fn diff<O>(cmd_context: DownloadCmdCtx<'_, O>) -> Result<(), DownloadError>
where
    O: OutputWrite<DownloadError>,
{
    DiffCmd::exec(cmd_context).await?;
    Ok(())
}

pub async fn ensure_dry<O>(cmd_context: DownloadCmdCtx<'_, O>) -> Result<(), DownloadError>
where
    O: OutputWrite<DownloadError>,
{
    EnsureCmd::exec_dry(cmd_context).await?;
    Ok(())
}

pub async fn ensure<O>(cmd_context: DownloadCmdCtx<'_, O>) -> Result<(), DownloadError>
where
    O: OutputWrite<DownloadError>,
{
    EnsureCmd::exec(cmd_context).await?;
    Ok(())
}

pub async fn clean_dry<O>(cmd_context: DownloadCmdCtx<'_, O>) -> Result<(), DownloadError>
where
    O: OutputWrite<DownloadError>,
{
    CleanCmd::exec_dry(cmd_context).await?;
    Ok(())
}

pub async fn clean<O>(cmd_context: DownloadCmdCtx<'_, O>) -> Result<(), DownloadError>
where
    O: OutputWrite<DownloadError>,
{
    CleanCmd::exec(cmd_context).await?;
    Ok(())
}
