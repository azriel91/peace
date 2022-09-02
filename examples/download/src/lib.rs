use std::path::PathBuf;

use peace::{
    cfg::{FlowId, Profile},
    resources::{
        resources_type_state::{
            Ensured, EnsuredDry, SetUp, WithStateDiffs, WithStates, WithStatesDesired,
        },
        Resources, StateDiffs, StatesCurrent, StatesDesired, StatesEnsured, StatesEnsuredDry,
    },
    rt::{DiffCmd, EnsureCmd, StateCurrentCmd, StateDesiredCmd},
    rt_model::{CmdContext, ItemSpecGraph, ItemSpecGraphBuilder, Workspace, WorkspaceSpec},
};
use tokio::io::{AsyncWrite, AsyncWriteExt};
use url::Url;

pub use crate::{
    download_args::{DownloadArgs, DownloadCommand},
    download_clean_op_spec::DownloadCleanOpSpec,
    download_ensure_op_spec::DownloadEnsureOpSpec,
    download_error::DownloadError,
    download_item_spec::DownloadItemSpec,
    download_params::DownloadParams,
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
pub async fn setup_workspace_and_graph(
    workspace_spec: WorkspaceSpec,
    profile: Profile,
    flow_id: FlowId,
    url: Url,
    dest: PathBuf,
) -> Result<WorkspaceAndGraph, DownloadError> {
    let workspace = Workspace::init(workspace_spec, profile, flow_id).await?;
    let item_spec_graph = {
        let mut item_spec_graph_builder = ItemSpecGraphBuilder::<DownloadError>::new();
        item_spec_graph_builder.add_fn(DownloadItemSpec::new(url, dest).into());
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
pub async fn setup_workspace_and_graph(
    workspace_spec: WorkspaceSpec,
    profile: Profile,
    flow_id: FlowId,
    url: Url,
    dest: PathBuf,
) -> Result<WorkspaceAndGraph, DownloadError> {
    let workspace = Workspace::init(workspace_spec, profile, flow_id).await?;
    let item_spec_graph = {
        let mut item_spec_graph_builder = ItemSpecGraphBuilder::<DownloadError>::new();
        item_spec_graph_builder.add_fn(DownloadItemSpec::new(url, dest).into());
        item_spec_graph_builder.build()
    };

    let workspace_and_graph = WorkspaceAndGraph {
        workspace,
        item_spec_graph,
    };
    Ok(workspace_and_graph)
}

/// Returns a `CmdContext` initialized from the workspace and item spec graph
pub async fn cmd_context(
    workspace_and_graph: &WorkspaceAndGraph,
) -> Result<CmdContext<'_, SetUp, DownloadError>, DownloadError> {
    let WorkspaceAndGraph {
        workspace,
        item_spec_graph,
    } = workspace_and_graph;
    CmdContext::init(workspace, item_spec_graph).await
}

pub async fn status<W>(
    output: W,
    cmd_context: CmdContext<'_, SetUp, DownloadError>,
) -> Result<Resources<WithStates>, DownloadError>
where
    W: AsyncWrite + Unpin,
{
    let CmdContext { resources, .. } = StateCurrentCmd::exec(cmd_context).await?;
    let states_serialized = {
        let states = resources.borrow::<StatesCurrent>();
        serde_yaml::to_string(&*states).map_err(DownloadError::StatesSerialize)?
    };

    output_write(output, &states_serialized).await?;
    Ok(resources)
}

pub async fn desired<W>(
    output: W,
    cmd_context: CmdContext<'_, SetUp, DownloadError>,
) -> Result<Resources<WithStatesDesired>, DownloadError>
where
    W: AsyncWrite + Unpin,
{
    let CmdContext { resources, .. } = StateDesiredCmd::exec(cmd_context).await?;
    let states_desired_serialized = {
        let states_desired = resources.borrow::<StatesDesired>();
        serde_yaml::to_string(&*states_desired).map_err(DownloadError::StatesDesiredSerialize)?
    };

    output_write(output, &states_desired_serialized).await?;
    Ok(resources)
}

pub async fn diff<W>(
    output: W,
    cmd_context: CmdContext<'_, SetUp, DownloadError>,
) -> Result<Resources<WithStateDiffs>, DownloadError>
where
    W: AsyncWrite + Unpin,
{
    let CmdContext { resources, .. } = DiffCmd::exec(cmd_context).await?;
    let state_diffs_serialized = {
        let state_diffs = resources.borrow::<StateDiffs>();
        serde_yaml::to_string(&*state_diffs).map_err(DownloadError::StateDiffsSerialize)?
    };

    output_write(output, &state_diffs_serialized).await?;
    Ok(resources)
}

pub async fn ensure_dry<W>(
    output: W,
    cmd_context: CmdContext<'_, SetUp, DownloadError>,
) -> Result<Resources<EnsuredDry>, DownloadError>
where
    W: AsyncWrite + Unpin,
{
    let CmdContext { resources, .. } = EnsureCmd::exec_dry(cmd_context).await?;
    let states_ensured_dry_serialized = {
        let states_ensured_dry = resources.borrow::<StatesEnsuredDry>();
        serde_yaml::to_string(&*states_ensured_dry).map_err(DownloadError::StateDiffsSerialize)?
    };

    output_write(output, &states_ensured_dry_serialized).await?;
    Ok(resources)
}

pub async fn ensure<W>(
    output: W,
    cmd_context: CmdContext<'_, SetUp, DownloadError>,
) -> Result<Resources<Ensured>, DownloadError>
where
    W: AsyncWrite + Unpin,
{
    let CmdContext { resources, .. } = EnsureCmd::exec(cmd_context).await?;
    let states_ensured_serialized = {
        let states_ensured = resources.borrow::<StatesEnsured>();
        serde_yaml::to_string(&*states_ensured).map_err(DownloadError::StateDiffsSerialize)?
    };

    output_write(output, &states_ensured_serialized).await?;
    Ok(resources)
}

pub async fn output_write<W>(mut output: W, s: &str) -> Result<(), DownloadError>
where
    W: AsyncWrite + Unpin,
{
    output
        .write_all(s.as_bytes())
        .await
        .map_err(DownloadError::StdoutWrite)
}

/// Read up to 1 kB in memory.
pub const IN_MEMORY_CONTENTS_MAX: u64 = 1024;
