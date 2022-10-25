use peace::{
    cfg::{item_spec_id, FlowId, ItemSpecId, Profile},
    resources::{
        resources::ts::{
            Cleaned, CleanedDry, Ensured, EnsuredDry, SetUp, WithStateDiffs, WithStates,
            WithStatesCurrentAndDesired, WithStatesDesired,
        },
        Resources,
    },
    rt::cmds::{
        CleanCmd, DiffCmd, EnsureCmd, StatesCurrentDisplayCmd, StatesDesiredDisplayCmd,
        StatesDiscoverCmd,
    },
    rt_model::{
        CmdContext, ItemSpecGraph, ItemSpecGraphBuilder, OutputWrite, Workspace, WorkspaceSpec,
    },
};

pub use crate::{
    download_args::{DownloadArgs, DownloadCommand},
    file_download_clean_op_spec::FileDownloadCleanOpSpec,
    file_download_ensure_op_spec::FileDownloadEnsureOpSpec,
    file_download_error::FileDownloadError,
    file_download_item_spec::FileDownloadItemSpec,
    file_download_params::FileDownloadParams,
    file_download_profile_init::FileDownloadProfileInit,
    file_download_state::FileDownloadState,
    file_download_state_current_fn_spec::FileDownloadStateCurrentFnSpec,
    file_download_state_desired_fn_spec::FileDownloadStateDesiredFnSpec,
    file_download_state_diff::FileDownloadStateDiff,
    file_download_state_diff_fn_spec::FileDownloadStateDiffFnSpec,
};

mod download_args;
mod file_download_clean_op_spec;
mod file_download_ensure_op_spec;
mod file_download_error;
mod file_download_item_spec;
mod file_download_params;
mod file_download_profile_init;
mod file_download_state;
mod file_download_state_current_fn_spec;
mod file_download_state_desired_fn_spec;
mod file_download_state_diff;
mod file_download_state_diff_fn_spec;

#[cfg(target_arch = "wasm32")]
pub use file_download_item_spec_graph::FileDownloadItemSpecGraph;

#[cfg(target_arch = "wasm32")]
mod file_download_item_spec_graph;
#[cfg(target_arch = "wasm32")]
mod wasm;

#[derive(Debug)]
pub struct WorkspaceAndGraph {
    workspace: Workspace,
    item_spec_graph: ItemSpecGraph<FileDownloadError>,
}

/// Returns a default workspace and the Download item spec graph.
#[cfg(not(target_arch = "wasm32"))]
pub async fn workspace_and_graph_setup(
    workspace_spec: WorkspaceSpec,
    profile: Profile,
    flow_id: FlowId,
) -> Result<WorkspaceAndGraph, FileDownloadError> {
    let workspace = Workspace::new(workspace_spec, profile, flow_id)?;

    let item_spec_graph = {
        let mut item_spec_graph_builder = ItemSpecGraphBuilder::<FileDownloadError>::new();
        item_spec_graph_builder.add_fn(FileDownloadItemSpec::new(item_spec_id!("file")).into());
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
) -> Result<WorkspaceAndGraph, FileDownloadError> {
    let workspace = Workspace::new(workspace_spec, profile, flow_id)?;
    let item_spec_graph = {
        let mut item_spec_graph_builder = ItemSpecGraphBuilder::<FileDownloadError>::new();
        item_spec_graph_builder.add_fn(FileDownloadItemSpec::new(item_spec_id!("file")).into());
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
    file_download_profile_init: Option<FileDownloadProfileInit>,
) -> Result<CmdContext<'ctx, FileDownloadError, O, SetUp>, FileDownloadError>
where
    O: OutputWrite<FileDownloadError>,
{
    let WorkspaceAndGraph {
        workspace,
        item_spec_graph,
    } = workspace_and_graph;
    CmdContext::builder(workspace, item_spec_graph, output)
        .with_profile_init(file_download_profile_init)
        .await
}

pub async fn fetch<O>(
    cmd_context: CmdContext<'_, FileDownloadError, O, SetUp>,
) -> Result<Resources<WithStatesCurrentAndDesired>, FileDownloadError>
where
    O: OutputWrite<FileDownloadError>,
{
    let CmdContext { resources, .. } = StatesDiscoverCmd::exec(cmd_context).await?;
    Ok(resources)
}

pub async fn status<O>(
    cmd_context: CmdContext<'_, FileDownloadError, O, SetUp>,
) -> Result<Resources<WithStates>, FileDownloadError>
where
    O: OutputWrite<FileDownloadError>,
{
    let CmdContext { resources, .. } = StatesCurrentDisplayCmd::exec(cmd_context).await?;
    Ok(resources)
}

pub async fn desired<O>(
    cmd_context: CmdContext<'_, FileDownloadError, O, SetUp>,
) -> Result<Resources<WithStatesDesired>, FileDownloadError>
where
    O: OutputWrite<FileDownloadError>,
{
    let CmdContext { resources, .. } = StatesDesiredDisplayCmd::exec(cmd_context).await?;
    Ok(resources)
}

pub async fn diff<O>(
    cmd_context: CmdContext<'_, FileDownloadError, O, SetUp>,
) -> Result<Resources<WithStateDiffs>, FileDownloadError>
where
    O: OutputWrite<FileDownloadError>,
{
    let CmdContext { resources, .. } = DiffCmd::exec(cmd_context).await?;
    Ok(resources)
}

pub async fn ensure_dry<O>(
    cmd_context: CmdContext<'_, FileDownloadError, O, SetUp>,
) -> Result<Resources<EnsuredDry>, FileDownloadError>
where
    O: OutputWrite<FileDownloadError>,
{
    let CmdContext { resources, .. } = EnsureCmd::exec_dry(cmd_context).await?;
    Ok(resources)
}

pub async fn ensure<O>(
    cmd_context: CmdContext<'_, FileDownloadError, O, SetUp>,
) -> Result<Resources<Ensured>, FileDownloadError>
where
    O: OutputWrite<FileDownloadError>,
{
    let CmdContext { resources, .. } = EnsureCmd::exec(cmd_context).await?;
    Ok(resources)
}

pub async fn clean_dry<O>(
    cmd_context: CmdContext<'_, FileDownloadError, O, SetUp>,
) -> Result<Resources<CleanedDry>, FileDownloadError>
where
    O: OutputWrite<FileDownloadError>,
{
    let CmdContext { resources, .. } = CleanCmd::exec_dry(cmd_context).await?;
    Ok(resources)
}

pub async fn clean<O>(
    cmd_context: CmdContext<'_, FileDownloadError, O, SetUp>,
) -> Result<Resources<Cleaned>, FileDownloadError>
where
    O: OutputWrite<FileDownloadError>,
{
    let CmdContext { resources, .. } = CleanCmd::exec(cmd_context).await?;
    Ok(resources)
}

/// Read up to 1 kB in memory.
pub const IN_MEMORY_CONTENTS_MAX: u64 = 1024;
