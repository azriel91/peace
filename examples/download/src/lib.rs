use std::path::PathBuf;

use peace::{
    resources::{
        resources_type_state::{
            Ensured, EnsuredDry, SetUp, WithStateDiffs, WithStates, WithStatesDesired,
        },
        Resources, StateDiffs, States, StatesDesired, StatesEnsured, StatesEnsuredDry,
    },
    rt::{DiffCmd, EnsureCmd, StateCurrentCmd, StateDesiredCmd},
    rt_model::{ItemSpecGraph, ItemSpecGraphBuilder},
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

pub async fn setup_graph(
    url: Url,
    dest: PathBuf,
) -> Result<ItemSpecGraph<DownloadError>, DownloadError> {
    let mut graph_builder = ItemSpecGraphBuilder::<DownloadError>::new();
    graph_builder.add_fn(DownloadItemSpec::new(url, dest).into());
    let graph = graph_builder.build();
    Ok(graph)
}

pub async fn status<W>(
    output: W,
    graph: &ItemSpecGraph<DownloadError>,
    resources: Resources<SetUp>,
) -> Result<Resources<WithStates>, DownloadError>
where
    W: AsyncWrite + Unpin,
{
    let resources = StateCurrentCmd::exec(graph, resources).await?;
    let states_serialized = {
        let states = resources.borrow::<States>();
        serde_yaml::to_string(&*states).map_err(DownloadError::StatesSerialize)?
    };

    output_write(output, &states_serialized).await?;
    Ok(resources)
}

pub async fn desired<W>(
    output: W,
    graph: &ItemSpecGraph<DownloadError>,
    resources: Resources<SetUp>,
) -> Result<Resources<WithStatesDesired>, DownloadError>
where
    W: AsyncWrite + Unpin,
{
    let resources = StateDesiredCmd::exec(graph, resources).await?;
    let states_desired_serialized = {
        let states_desired = resources.borrow::<StatesDesired>();
        serde_yaml::to_string(&*states_desired).map_err(DownloadError::StatesDesiredSerialize)?
    };

    output_write(output, &states_desired_serialized).await?;
    Ok(resources)
}

pub async fn diff<W>(
    output: W,
    graph: &ItemSpecGraph<DownloadError>,
    resources: Resources<SetUp>,
) -> Result<Resources<WithStateDiffs>, DownloadError>
where
    W: AsyncWrite + Unpin,
{
    let resources = DiffCmd::exec(graph, resources).await?;
    let state_diffs_serialized = {
        let state_diffs = resources.borrow::<StateDiffs>();
        serde_yaml::to_string(&*state_diffs).map_err(DownloadError::StateDiffsSerialize)?
    };

    output_write(output, &state_diffs_serialized).await?;
    Ok(resources)
}

pub async fn ensure_dry<W>(
    output: W,
    graph: &ItemSpecGraph<DownloadError>,
    resources: Resources<SetUp>,
) -> Result<Resources<EnsuredDry>, DownloadError>
where
    W: AsyncWrite + Unpin,
{
    let resources = EnsureCmd::exec_dry(graph, resources).await?;
    let states_ensured_dry_serialized = {
        let states_ensured_dry = resources.borrow::<StatesEnsuredDry>();
        serde_yaml::to_string(&*states_ensured_dry).map_err(DownloadError::StateDiffsSerialize)?
    };

    output_write(output, &states_ensured_dry_serialized).await?;
    Ok(resources)
}

pub async fn ensure<W>(
    output: W,
    graph: &ItemSpecGraph<DownloadError>,
    resources: Resources<SetUp>,
) -> Result<Resources<Ensured>, DownloadError>
where
    W: AsyncWrite + Unpin,
{
    let resources = EnsureCmd::exec(graph, resources).await?;
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
