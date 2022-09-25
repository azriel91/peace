use peace::{
    cfg::{FlowId, Profile},
    resources::{
        resources_type_state::{
            Ensured, EnsuredDry, SetUp, WithStateDiffs, WithStates, WithStatesCurrentAndDesired,
            WithStatesDesired,
        },
        Resources,
    },
    rt::cmds::{
        DiffCmd, EnsureCmd, StatesCurrentDisplayCmd, StatesDesiredDisplayCmd, StatesDiscoverCmd,
    },
    rt_model::{
        CmdContext, ItemSpecGraph, ItemSpecGraphBuilder, OutputWrite, Workspace, WorkspaceSpec,
    },
};

pub use crate::{
    download_args::{DownloadArgs, DownloadCommand},
    download_clean_op_spec::DownloadCleanOpSpec,
    download_ensure_op_spec::DownloadEnsureOpSpec,
    download_error::DownloadError,
    download_item_spec::DownloadItemSpec,
    download_params::DownloadParams,
    download_profile_init::DownloadProfileInit,
    download_state_current_fn_spec::DownloadStateCurrentFnSpec,
    download_state_desired_fn_spec::DownloadStateDesiredFnSpec,
    download_state_diff_fn_spec::DownloadStateDiffFnSpec,
    file_state::FileState,
    file_state_diff::FileStateDiff,
};

mod download_args;
mod download_clean_op_spec;
mod download_ensure_op_spec;
mod download_error;
mod download_item_spec;
mod download_params;
mod download_profile_init;
mod download_state_current_fn_spec;
mod download_state_desired_fn_spec;
mod download_state_diff_fn_spec;
mod file_state;
mod file_state_diff;

#[cfg(target_arch = "wasm32")]
pub use download_item_spec_graph::DownloadItemSpecGraph;

#[cfg(target_arch = "wasm32")]
mod download_item_spec_graph;
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
        item_spec_graph_builder.add_fn(DownloadItemSpec.into());
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
        item_spec_graph_builder.add_fn(DownloadItemSpec.into());
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
    download_profile_init: Option<DownloadProfileInit>,
) -> Result<CmdContext<'ctx, DownloadError, O, SetUp>, DownloadError>
where
    O: OutputWrite<DownloadError>,
{
    let WorkspaceAndGraph {
        workspace,
        item_spec_graph,
    } = workspace_and_graph;
    CmdContext::builder(workspace, item_spec_graph, output)
        .with_profile_init(download_profile_init)
        .await
}

pub async fn init<O>(
    cmd_context: CmdContext<'_, DownloadError, O, SetUp>,
) -> Result<Resources<SetUp>, DownloadError>
where
    O: OutputWrite<DownloadError>,
{
    let CmdContext { resources, .. } = cmd_context;
    Ok(resources)
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
) -> Result<Resources<WithStates>, DownloadError>
where
    O: OutputWrite<DownloadError>,
{
    let CmdContext { resources, .. } = StatesCurrentDisplayCmd::exec(cmd_context).await?;
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
) -> Result<Resources<WithStateDiffs>, DownloadError>
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

/// Read up to 1 kB in memory.
pub const IN_MEMORY_CONTENTS_MAX: u64 = 1024;
