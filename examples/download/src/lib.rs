use peace::{
    cfg::{app_name, item_spec_id, AppName, FlowId, ItemSpecId, Profile},
    cmd::{ctx::CmdCtx, scopes::SingleProfileSingleFlow},
    resources::resources::ts::SetUp,
    rt::cmds::{
        CleanCmd, DiffCmd, EnsureCmd, StatesDesiredDisplayCmd, StatesDiscoverCmd,
        StatesSavedDisplayCmd,
    },
    rt_model::{
        cmd_context_params::{KeyKnown, KeyUnknown, ParamsKeysImpl},
        output::OutputWrite,
        Flow, ItemSpecGraph, ItemSpecGraphBuilder, Workspace, WorkspaceSpec,
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
pub struct WorkspaceAndGraph {
    workspace: Workspace,
    item_spec_graph: ItemSpecGraph<DownloadError>,
}

/// Returns a default workspace and the Download item spec graph.
#[cfg(not(target_arch = "wasm32"))]
pub async fn workspace_and_graph_setup(
    workspace_spec: WorkspaceSpec,
) -> Result<WorkspaceAndGraph, DownloadError> {
    let workspace = Workspace::new(app_name!(), workspace_spec)?;

    let item_spec_graph = {
        let mut item_spec_graph_builder = ItemSpecGraphBuilder::<DownloadError>::new();
        item_spec_graph_builder
            .add_fn(FileDownloadItemSpec::<FileId>::new(item_spec_id!("file")).into());
        item_spec_graph_builder.build()
    };

    let workspace_and_graph = WorkspaceAndGraph {
        workspace,
        item_spec_graph,
    };
    Ok(workspace_and_graph)
}

/// Returns a default workspace and the Download item spec graph.
#[cfg(target_arch = "wasm32")]
pub async fn workspace_and_graph_setup(
    workspace_spec: WorkspaceSpec,
) -> Result<WorkspaceAndGraph, DownloadError> {
    let workspace = Workspace::new(app_name!(), workspace_spec)?;
    let item_spec_graph = {
        let mut item_spec_graph_builder = ItemSpecGraphBuilder::<DownloadError>::new();
        item_spec_graph_builder
            .add_fn(FileDownloadItemSpec::<FileId>::new(item_spec_id!("file")).into());
        item_spec_graph_builder.build()
    };

    let workspace_and_graph = WorkspaceAndGraph {
        workspace,
        item_spec_graph,
    };
    Ok(workspace_and_graph)
}

pub type DownloadCmdCtx<'ctx, O> = CmdCtx<
    'ctx,
    O,
    SingleProfileSingleFlow<
        DownloadError,
        ParamsKeysImpl<KeyUnknown, KeyKnown<String>, KeyUnknown>,
        SetUp,
    >,
    ParamsKeysImpl<KeyUnknown, KeyKnown<String>, KeyUnknown>,
>;

/// Returns a `CmdCtx` initialized from the workspace and item spec graph
pub async fn cmd_context<'ctx, O>(
    workspace_and_graph: &'ctx WorkspaceAndGraph,
    profile: Profile,
    flow_id: FlowId,
    output: &'ctx mut O,
    file_download_params: Option<FileDownloadParams<FileId>>,
) -> Result<DownloadCmdCtx<'ctx, O>, DownloadError>
where
    O: OutputWrite<DownloadError>,
{
    let WorkspaceAndGraph {
        workspace,
        item_spec_graph,
    } = workspace_and_graph;
    CmdCtx::builder_single_profile_single_flow(output, workspace)
        .with_profile(profile)
        .with_flow(Flow::new(flow_id, item_spec_graph.clone()))
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
