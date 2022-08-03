use std::path::Path;

use clap::Parser;
use peace::{
    resources::{resources_type_state::SetUp, Resources, StateDiffs, States, StatesDesired},
    rt::{DiffCmd, StateCurrentCmd, StateDesiredCmd},
    rt_model::{FullSpecGraph, FullSpecGraphBuilder},
};
use peace_resources::StatesEnsured;
use peace_rt::EnsureCmd;
use tokio::io::{self, AsyncWriteExt, Stdout};
use url::Url;

pub use crate::{
    download_args::{DownloadArgs, DownloadCommand},
    download_clean_op_spec::DownloadCleanOpSpec,
    download_ensure_op_spec::DownloadEnsureOpSpec,
    download_error::DownloadError,
    download_full_spec::DownloadFullSpec,
    download_params::DownloadParams,
    download_state_current_fn_spec::DownloadStateCurrentFnSpec,
    download_state_desired_fn_spec::DownloadStateDesiredFnSpec,
    download_state_diff_fn_spec::DownloadStateDiffFnSpec,
    file_state::FileState,
    file_state_diff::FileStateDiff,
};

#[path = "download/download_args.rs"]
mod download_args;
#[path = "download/download_clean_op_spec.rs"]
mod download_clean_op_spec;
#[path = "download/download_ensure_op_spec.rs"]
mod download_ensure_op_spec;
#[path = "download/download_error.rs"]
mod download_error;
#[path = "download/download_full_spec.rs"]
mod download_full_spec;
#[path = "download/download_params.rs"]
mod download_params;
#[path = "download/download_state_current_fn_spec.rs"]
mod download_state_current_fn_spec;
#[path = "download/download_state_desired_fn_spec.rs"]
mod download_state_desired_fn_spec;
#[path = "download/download_state_diff_fn_spec.rs"]
mod download_state_diff_fn_spec;
#[path = "download/file_state.rs"]
mod file_state;
#[path = "download/file_state_diff.rs"]
mod file_state_diff;

fn main() -> Result<(), DownloadError> {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .thread_name("main")
        .thread_stack_size(3 * 1024 * 1024)
        .enable_io()
        .enable_time()
        .build()
        .map_err(DownloadError::TokioRuntimeInit)?;

    let download_args = DownloadArgs::parse();
    runtime.block_on(async {
        let url =
            Url::parse("https://api.my-ip.io/ip.json").expect("Expected download URL to be valid.");
        let dest = Path::new("all.json").to_path_buf();

        let mut graph_builder = FullSpecGraphBuilder::<DownloadError>::new();
        graph_builder.add_fn(DownloadFullSpec::new(url, dest).into());

        let graph = graph_builder.build();

        let resources = graph.setup(Resources::new()).await?;

        match download_args.command {
            DownloadCommand::Status => status(&graph, resources).await,
            DownloadCommand::Desired => desired(&graph, resources).await,
            DownloadCommand::Diff => diff(&graph, resources).await,
            DownloadCommand::Ensure => ensure(&graph, resources).await,
        }?;

        Ok::<_, DownloadError>(())
    })
}

async fn status(
    graph: &FullSpecGraph<DownloadError>,
    resources: Resources<SetUp>,
) -> Result<(), DownloadError> {
    let resources = StateCurrentCmd::exec(graph, resources).await?;
    let states = resources.borrow::<States>();
    let states_serialized =
        serde_yaml::to_string(&*states).map_err(DownloadError::StatesSerialize)?;

    let mut stdout = io::stdout();
    stdout_write(&mut stdout, states_serialized.as_bytes()).await
}

async fn desired(
    graph: &FullSpecGraph<DownloadError>,
    resources: Resources<SetUp>,
) -> Result<(), DownloadError> {
    let resources = StateDesiredCmd::exec(graph, resources).await?;
    let states_desired = resources.borrow::<StatesDesired>();
    let states_desired_serialized =
        serde_yaml::to_string(&*states_desired).map_err(DownloadError::StatesDesiredSerialize)?;

    let mut stdout = io::stdout();
    stdout_write(&mut stdout, states_desired_serialized.as_bytes()).await
}

async fn diff(
    graph: &FullSpecGraph<DownloadError>,
    resources: Resources<SetUp>,
) -> Result<(), DownloadError> {
    let resources = DiffCmd::exec(graph, resources).await?;
    let state_diffs = resources.borrow::<StateDiffs>();
    let state_diffs_serialized =
        serde_yaml::to_string(&*state_diffs).map_err(DownloadError::StateDiffsSerialize)?;

    let mut stdout = io::stdout();
    stdout_write(&mut stdout, state_diffs_serialized.as_bytes()).await
}

async fn ensure(
    graph: &FullSpecGraph<DownloadError>,
    resources: Resources<SetUp>,
) -> Result<(), DownloadError> {
    let resources = EnsureCmd::exec(graph, resources).await?;
    let states_ensured = resources.borrow::<StatesEnsured>();
    let states_ensured_serialized =
        serde_yaml::to_string(&*states_ensured).map_err(DownloadError::StateDiffsSerialize)?;

    let mut stdout = io::stdout();
    stdout_write(&mut stdout, states_ensured_serialized.as_bytes()).await
}

async fn stdout_write(stdout: &mut Stdout, bytes: &[u8]) -> Result<(), DownloadError> {
    stdout
        .write_all(bytes)
        .await
        .map_err(DownloadError::StdoutWrite)
}

/// Read up to 1 kB in memory.
pub const IN_MEMORY_CONTENTS_MAX: u64 = 1024;
