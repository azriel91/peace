use std::path::PathBuf;

use peace::{
    cfg::Profile,
    resources::{
        resources_type_state::{
            Ensured, EnsuredDry, SetUp, WithStateDiffs, WithStates, WithStatesDesired,
        },
        StateDiffs, States, StatesDesired, StatesEnsured, StatesEnsuredDry,
    },
    rt::{DiffCmd, EnsureCmd, StateCurrentCmd, StateDesiredCmd},
    rt_model::{ItemSpecGraphBuilder, Workspace, WorkspaceSpec},
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

pub async fn setup_workspace(
    workspace_spec: &WorkspaceSpec,
    profile: Profile,
    url: Url,
    dest: PathBuf,
) -> Result<Resources<SetUp, DownloadError>, DownloadError> {
    let mut graph_builder = ItemSpecGraphBuilder::<DownloadError>::new();
    graph_builder.add_fn(DownloadItemSpec::new(url, dest).into());
    let graph = graph_builder.build();

    let workspace = Workspace::try_new(workspace_spec, profile, graph).await?;
    Ok(workspace)
}

pub async fn status<W>(
    output: W,
    workspace: Workspace<SetUp, DownloadError>,
) -> Result<Resources<WithStates, DownloadError>, DownloadError>
where
    W: AsyncWrite + Unpin,
{
    let workspace = StateCurrentCmd::exec(workspace).await?;
    let resources = workspace.resources();
    let states_serialized = {
        let states = resources.borrow::<States>();
        serde_yaml::to_string(&*states).map_err(DownloadError::StatesSerialize)?
    };

    output_write(output, &states_serialized).await?;
    Ok(workspace)
}

pub async fn desired<W>(
    output: W,
    workspace: Workspace<SetUp, DownloadError>,
) -> Result<Resources<WithStatesDesired, DownloadError>, DownloadError>
where
    W: AsyncWrite + Unpin,
{
    let workspace = StateDesiredCmd::exec(workspace).await?;
    let resources = workspace.resources();
    let states_desired_serialized = {
        let states_desired = resources.borrow::<StatesDesired>();
        serde_yaml::to_string(&*states_desired).map_err(DownloadError::StatesDesiredSerialize)?
    };

    output_write(output, &states_desired_serialized).await?;
    Ok(workspace)
}

pub async fn diff<W>(
    output: W,
    workspace: Workspace<SetUp, DownloadError>,
) -> Result<Resources<WithStateDiffs, DownloadError>, DownloadError>
where
    W: AsyncWrite + Unpin,
{
    let workspace = DiffCmd::exec(workspace).await?;
    let resources = workspace.resources();
    let state_diffs_serialized = {
        let state_diffs = resources.borrow::<StateDiffs>();
        serde_yaml::to_string(&*state_diffs).map_err(DownloadError::StateDiffsSerialize)?
    };

    output_write(output, &state_diffs_serialized).await?;
    Ok(workspace)
}

pub async fn ensure_dry<W>(
    output: W,
    workspace: Workspace<SetUp, DownloadError>,
) -> Result<Resources<EnsuredDry, DownloadError>, DownloadError>
where
    W: AsyncWrite + Unpin,
{
    let workspace = EnsureCmd::exec_dry(workspace).await?;
    let resources = workspace.resources();
    let states_ensured_dry_serialized = {
        let states_ensured_dry = resources.borrow::<StatesEnsuredDry>();
        serde_yaml::to_string(&*states_ensured_dry).map_err(DownloadError::StateDiffsSerialize)?
    };

    output_write(output, &states_ensured_dry_serialized).await?;
    Ok(workspace)
}

pub async fn ensure<W>(
    output: W,
    workspace: Workspace<SetUp, DownloadError>,
) -> Result<Resources<Ensured, DownloadError>, DownloadError>
where
    W: AsyncWrite + Unpin,
{
    let workspace = EnsureCmd::exec(workspace).await?;
    let resources = workspace.resources();
    let states_ensured_serialized = {
        let states_ensured = resources.borrow::<StatesEnsured>();
        serde_yaml::to_string(&*states_ensured).map_err(DownloadError::StateDiffsSerialize)?
    };

    output_write(output, &states_ensured_serialized).await?;
    Ok(workspace)
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
