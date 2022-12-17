use peace::{
    cfg::{item_spec_id, FlowId, ItemSpecId, Profile},
    resources::{
        resources::ts::{
            Cleaned, CleanedDry, Ensured, EnsuredDry, SetUp, WithStatePreviousDiffs,
            WithStatesCurrentAndDesired, WithStatesDesired, WithStatesPrevious,
        },
        Resources,
    },
    rt::cmds::{
        CleanCmd, DiffCmd, EnsureCmd, StatesDesiredDisplayCmd, StatesDiscoverCmd,
        StatesPreviousDisplayCmd,
    },
    rt_model::{
        CmdContext, ItemSpecGraph, ItemSpecGraphBuilder, OutputWrite, Workspace, WorkspaceSpec,
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
    profile: Profile,
    flow_id: FlowId,
) -> Result<WorkspaceAndGraph, DownloadError> {
    let workspace = Workspace::new(workspace_spec, profile, flow_id)?;

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
    profile: Profile,
    flow_id: FlowId,
) -> Result<WorkspaceAndGraph, DownloadError> {
    let workspace = Workspace::new(workspace_spec, profile, flow_id)?;
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

/// Returns a `CmdContext` initialized from the workspace and item spec graph
pub async fn cmd_context<'ctx, O>(
    workspace_and_graph: &'ctx WorkspaceAndGraph,
    output: &'ctx mut O,
    file_download_params: Option<FileDownloadParams<FileId>>,
) -> Result<CmdContext<'ctx, DownloadError, O, SetUp>, DownloadError>
where
    O: OutputWrite<DownloadError>,
{
    let WorkspaceAndGraph {
        workspace,
        item_spec_graph,
    } = workspace_and_graph;
    CmdContext::builder(workspace, item_spec_graph, output)
        .with_profile_init(file_download_params)
        .await
}

pub async fn fetch<O>(
    cmd_context: CmdContext<'_, DownloadError, O, SetUp>,
) -> Result<Resources<WithStatesCurrentAndDesired>, DownloadError>
where
    O: OutputWrite<DownloadError>,
{
    let CmdContext { resources, .. } = StatesDiscoverCmd::exec(cmd_context).await?;
    Ok(resources)
}

pub async fn status<O>(
    cmd_context: CmdContext<'_, DownloadError, O, SetUp>,
) -> Result<Resources<WithStatesPrevious>, DownloadError>
where
    O: OutputWrite<DownloadError>,
{
    let CmdContext { resources, .. } = StatesPreviousDisplayCmd::exec(cmd_context).await?;
    Ok(resources)
}

pub async fn desired<O>(
    cmd_context: CmdContext<'_, DownloadError, O, SetUp>,
) -> Result<Resources<WithStatesDesired>, DownloadError>
where
    O: OutputWrite<DownloadError>,
{
    let CmdContext { resources, .. } = StatesDesiredDisplayCmd::exec(cmd_context).await?;
    Ok(resources)
}

pub async fn diff<O>(
    cmd_context: CmdContext<'_, DownloadError, O, SetUp>,
) -> Result<Resources<WithStatePreviousDiffs>, DownloadError>
where
    O: OutputWrite<DownloadError>,
{
    let CmdContext { resources, .. } = DiffCmd::exec(cmd_context).await?;
    Ok(resources)
}

pub async fn ensure_dry<O>(
    cmd_context: CmdContext<'_, DownloadError, O, SetUp>,
) -> Result<Resources<EnsuredDry>, DownloadError>
where
    O: OutputWrite<DownloadError>,
{
    let CmdContext { resources, .. } = EnsureCmd::exec_dry(cmd_context).await?;
    Ok(resources)
}

pub async fn ensure<O>(
    cmd_context: CmdContext<'_, DownloadError, O, SetUp>,
) -> Result<Resources<Ensured>, DownloadError>
where
    O: OutputWrite<DownloadError>,
{
    let CmdContext { resources, .. } = EnsureCmd::exec(cmd_context).await?;
    Ok(resources)
}

pub async fn clean_dry<O>(
    cmd_context: CmdContext<'_, DownloadError, O, SetUp>,
) -> Result<Resources<CleanedDry>, DownloadError>
where
    O: OutputWrite<DownloadError>,
{
    let CmdContext { resources, .. } = CleanCmd::exec_dry(cmd_context).await?;
    Ok(resources)
}

pub async fn clean<O>(
    cmd_context: CmdContext<'_, DownloadError, O, SetUp>,
) -> Result<Resources<Cleaned>, DownloadError>
where
    O: OutputWrite<DownloadError>,
{
    let CmdContext { resources, .. } = CleanCmd::exec(cmd_context).await?;
    Ok(resources)
}
