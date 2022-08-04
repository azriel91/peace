use std::path::PathBuf;

use clap::Parser;
use peace::{
    resources::{
        resources_type_state::SetUp, Resources, StateDiffs, States, StatesDesired, StatesEnsured,
        StatesEnsuredDry,
    },
    rt::{DiffCmd, EnsureCmd, StateCurrentCmd, StateDesiredCmd},
    rt_model::{FullSpecGraph, FullSpecGraphBuilder},
};
#[cfg(not(target_arch = "wasm32"))]
use tokio::io::{self, AsyncWriteExt};
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

mod download_args;
mod download_clean_op_spec;
mod download_ensure_op_spec;
mod download_error;
mod download_full_spec;
mod download_params;
mod download_state_current_fn_spec;
mod download_state_desired_fn_spec;
mod download_state_diff_fn_spec;
mod file_state;
mod file_state_diff;

#[cfg(not(target_arch = "wasm32"))]
pub fn run() -> Result<(), DownloadError> {
    let mut builder = tokio::runtime::Builder::new_current_thread();

    builder
        .thread_name("main")
        .thread_stack_size(3 * 1024 * 1024);

    #[cfg(not(target_arch = "wasm32"))]
    builder.enable_io().enable_time();

    let runtime = builder.build().map_err(DownloadError::TokioRuntimeInit)?;

    let DownloadArgs { command } = DownloadArgs::parse();
    runtime.block_on(async {
        match command {
            DownloadCommand::Status { url, dest } => {
                let (graph, resources) = setup_graph(url, dest).await?;
                status(&graph, resources).await
            }
            DownloadCommand::Desired { url, dest } => {
                let (graph, resources) = setup_graph(url, dest).await?;
                desired(&graph, resources).await
            }
            DownloadCommand::Diff { url, dest } => {
                let (graph, resources) = setup_graph(url, dest).await?;
                diff(&graph, resources).await
            }
            DownloadCommand::EnsureDry { url, dest } => {
                let (graph, resources) = setup_graph(url, dest).await?;
                ensure_dry(&graph, resources).await
            }
            DownloadCommand::Ensure { url, dest } => {
                let (graph, resources) = setup_graph(url, dest).await?;
                ensure(&graph, resources).await
            }
        }?;

        Ok::<_, DownloadError>(())
    })
}

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn wasm_status() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    let runtime = tokio::runtime::Builder::new_current_thread()
        .thread_name("main")
        .thread_stack_size(3 * 1024 * 1024)
        .build()
        .map_err(DownloadError::TokioRuntimeInit)
        .expect("Failed to initialize tokio runtime.");

    let result = runtime.block_on(async move {
        let (graph, resources) = setup_graph(
            Url::parse("https://ifconfig.me").expect("Failed to parse URL."),
            std::path::Path::new("ip.json").to_path_buf(),
        )
        .await?;
        status(&graph, resources).await?;

        Result::<(), DownloadError>::Ok(())
    });

    if let Err(e) = result {
        log(&format!("{e}"));
    }
}

async fn setup_graph(
    url: Url,
    dest: PathBuf,
) -> Result<(FullSpecGraph<DownloadError>, Resources<SetUp>), DownloadError> {
    let mut graph_builder = FullSpecGraphBuilder::<DownloadError>::new();
    graph_builder.add_fn(DownloadFullSpec::new(url, dest).into());
    let graph = graph_builder.build();
    let resources = graph.setup(Resources::new()).await?;
    Ok((graph, resources))
}

async fn status(
    graph: &FullSpecGraph<DownloadError>,
    resources: Resources<SetUp>,
) -> Result<(), DownloadError> {
    let resources = StateCurrentCmd::exec(graph, resources).await?;
    let states = resources.borrow::<States>();
    let states_serialized =
        serde_yaml::to_string(&*states).map_err(DownloadError::StatesSerialize)?;

    stdout_write(&states_serialized).await
}

async fn desired(
    graph: &FullSpecGraph<DownloadError>,
    resources: Resources<SetUp>,
) -> Result<(), DownloadError> {
    let resources = StateDesiredCmd::exec(graph, resources).await?;
    let states_desired = resources.borrow::<StatesDesired>();
    let states_desired_serialized =
        serde_yaml::to_string(&*states_desired).map_err(DownloadError::StatesDesiredSerialize)?;

    stdout_write(&states_desired_serialized).await
}

async fn diff(
    graph: &FullSpecGraph<DownloadError>,
    resources: Resources<SetUp>,
) -> Result<(), DownloadError> {
    let resources = DiffCmd::exec(graph, resources).await?;
    let state_diffs = resources.borrow::<StateDiffs>();
    let state_diffs_serialized =
        serde_yaml::to_string(&*state_diffs).map_err(DownloadError::StateDiffsSerialize)?;

    stdout_write(&state_diffs_serialized).await
}

async fn ensure_dry(
    graph: &FullSpecGraph<DownloadError>,
    resources: Resources<SetUp>,
) -> Result<(), DownloadError> {
    let resources = EnsureCmd::exec_dry(graph, resources).await?;
    let states_ensured_dry = resources.borrow::<StatesEnsuredDry>();
    let states_ensured_dry_serialized =
        serde_yaml::to_string(&*states_ensured_dry).map_err(DownloadError::StateDiffsSerialize)?;

    stdout_write(&states_ensured_dry_serialized).await
}

async fn ensure(
    graph: &FullSpecGraph<DownloadError>,
    resources: Resources<SetUp>,
) -> Result<(), DownloadError> {
    let resources = EnsureCmd::exec(graph, resources).await?;
    let states_ensured = resources.borrow::<StatesEnsured>();
    let states_ensured_serialized =
        serde_yaml::to_string(&*states_ensured).map_err(DownloadError::StateDiffsSerialize)?;

    stdout_write(&states_ensured_serialized).await
}

#[cfg(not(target_arch = "wasm32"))]
async fn stdout_write(s: &str) -> Result<(), DownloadError> {
    let mut stdout = io::stdout();
    stdout
        .write_all(s.as_bytes())
        .await
        .map_err(DownloadError::StdoutWrite)
}

#[cfg(target_arch = "wasm32")]
async fn stdout_write(s: &str) -> Result<(), DownloadError> {
    log(s);
    Ok(())
}

/// Read up to 1 kB in memory.
pub const IN_MEMORY_CONTENTS_MAX: u64 = 1024;
