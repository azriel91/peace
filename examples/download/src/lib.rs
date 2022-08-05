use std::path::PathBuf;

#[cfg(not(target_arch = "wasm32"))]
use clap::Parser;
use peace::{
    resources::{
        resources_type_state::{
            Ensured, EnsuredDry, SetUp, WithStateDiffs, WithStates, WithStatesDesired,
        },
        Resources, StateDiffs, States, StatesDesired, StatesEnsured, StatesEnsuredDry,
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
    let runtime = tokio::runtime::Builder::new_current_thread()
        .thread_name("main")
        .thread_stack_size(3 * 1024 * 1024)
        .enable_io()
        .enable_time()
        .build()
        .map_err(DownloadError::TokioRuntimeInit)?;

    let DownloadArgs { command } = DownloadArgs::parse();
    runtime.block_on(async {
        match command {
            DownloadCommand::Status { url, dest } => {
                let graph = setup_graph(url, dest).await?;
                let resources = graph.setup(Resources::new()).await?;
                status(&graph, resources).await?;
            }
            DownloadCommand::Desired { url, dest } => {
                let graph = setup_graph(url, dest).await?;
                let resources = graph.setup(Resources::new()).await?;
                desired(&graph, resources).await?;
            }
            DownloadCommand::Diff { url, dest } => {
                let graph = setup_graph(url, dest).await?;
                let resources = graph.setup(Resources::new()).await?;
                diff(&graph, resources).await?;
            }
            DownloadCommand::EnsureDry { url, dest } => {
                let graph = setup_graph(url, dest).await?;
                let resources = graph.setup(Resources::new()).await?;
                ensure_dry(&graph, resources).await?;
            }
            DownloadCommand::Ensure { url, dest } => {
                let graph = setup_graph(url, dest).await?;
                let resources = graph.setup(Resources::new()).await?;
                ensure(&graph, resources).await?;
            }
        }

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
#[wasm_bindgen(getter_with_clone)]
pub struct GraphAndContent(
    peace::rt_model::FullSpecGraph<DownloadError>,
    std::collections::HashMap<PathBuf, String>,
);

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub async fn wasm_setup(url: String, name: String) -> Result<GraphAndContent, JsValue> {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    setup_graph(
        Url::parse(&url).expect("Failed to parse URL."),
        std::path::PathBuf::from(name),
    )
    .await
    .map(|graph| async move {
        let mut resources = graph
            .setup(Resources::new())
            .await
            .map_err(|e| JsValue::from_str(&format!("{e}")))?;
        let content = resources
            .remove::<std::collections::HashMap<PathBuf, String>>()
            .ok_or(JsValue::from_str(
                "Resources did not contain content HashMap.",
            ))?;
        Ok(GraphAndContent(graph, content))
    })
    .map_err(|e| JsValue::from_str(&format!("{e}")))?
    .await
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub async fn wasm_status(graph_and_content: GraphAndContent) -> Result<GraphAndContent, JsValue> {
    let GraphAndContent(graph, content) = graph_and_content;
    let mut resources = graph
        .setup(Resources::new())
        .await
        .map_err(|e| JsValue::from_str(&format!("{e}")))?;
    resources.insert(content);

    let mut resources = status(&graph, resources.into())
        .await
        .map_err(|e| JsValue::from_str(&format!("{e}")))?;

    let content = resources
        .remove::<std::collections::HashMap<PathBuf, String>>()
        .ok_or(JsValue::from_str(
            "Resources did not contain content HashMap.",
        ))?;
    Ok(GraphAndContent(graph, content))
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub async fn wasm_desired(graph_and_content: GraphAndContent) -> Result<GraphAndContent, JsValue> {
    let GraphAndContent(graph, content) = graph_and_content;
    let mut resources = graph
        .setup(Resources::new())
        .await
        .map_err(|e| JsValue::from_str(&format!("{e}")))?;
    resources.insert(content);

    let mut resources = desired(&graph, resources.into())
        .await
        .map_err(|e| JsValue::from_str(&format!("{e}")))?;

    let content = resources
        .remove::<std::collections::HashMap<PathBuf, String>>()
        .ok_or(JsValue::from_str(
            "Resources did not contain content HashMap.",
        ))?;
    Ok(GraphAndContent(graph, content))
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub async fn wasm_diff(graph_and_content: GraphAndContent) -> Result<GraphAndContent, JsValue> {
    let GraphAndContent(graph, content) = graph_and_content;
    let mut resources = graph
        .setup(Resources::new())
        .await
        .map_err(|e| JsValue::from_str(&format!("{e}")))?;
    resources.insert(content);

    let mut resources = diff(&graph, resources.into())
        .await
        .map_err(|e| JsValue::from_str(&format!("{e}")))?;

    let content = resources
        .remove::<std::collections::HashMap<PathBuf, String>>()
        .ok_or(JsValue::from_str(
            "Resources did not contain content HashMap.",
        ))?;
    Ok(GraphAndContent(graph, content))
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub async fn wasm_ensure_dry(
    graph_and_content: GraphAndContent,
) -> Result<GraphAndContent, JsValue> {
    let GraphAndContent(graph, content) = graph_and_content;
    let mut resources = graph
        .setup(Resources::new())
        .await
        .map_err(|e| JsValue::from_str(&format!("{e}")))?;
    resources.insert(content);

    let mut resources = ensure_dry(&graph, resources.into())
        .await
        .map_err(|e| JsValue::from_str(&format!("{e}")))?;

    let content = resources
        .remove::<std::collections::HashMap<PathBuf, String>>()
        .ok_or(JsValue::from_str(
            "Resources did not contain content HashMap.",
        ))?;
    Ok(GraphAndContent(graph, content))
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub async fn wasm_ensure(graph_and_content: GraphAndContent) -> Result<GraphAndContent, JsValue> {
    let GraphAndContent(graph, content) = graph_and_content;
    let mut resources = graph
        .setup(Resources::new())
        .await
        .map_err(|e| JsValue::from_str(&format!("{e}")))?;
    resources.insert(content);

    let mut resources = ensure(&graph, resources.into())
        .await
        .map_err(|e| JsValue::from_str(&format!("{e}")))?;

    let content = resources
        .remove::<std::collections::HashMap<PathBuf, String>>()
        .ok_or(JsValue::from_str(
            "Resources did not contain content HashMap.",
        ))?;
    Ok(GraphAndContent(graph, content))
}

async fn setup_graph(
    url: Url,
    dest: PathBuf,
) -> Result<FullSpecGraph<DownloadError>, DownloadError> {
    let mut graph_builder = FullSpecGraphBuilder::<DownloadError>::new();
    graph_builder.add_fn(DownloadFullSpec::new(url, dest).into());
    let graph = graph_builder.build();
    Ok(graph)
}

async fn status(
    graph: &FullSpecGraph<DownloadError>,
    resources: Resources<SetUp>,
) -> Result<Resources<WithStates>, DownloadError> {
    let resources = StateCurrentCmd::exec(graph, resources).await?;
    let states_serialized = {
        let states = resources.borrow::<States>();
        serde_yaml::to_string(&*states).map_err(DownloadError::StatesSerialize)?
    };

    stdout_write(&states_serialized).await?;
    Ok(resources)
}

async fn desired(
    graph: &FullSpecGraph<DownloadError>,
    resources: Resources<SetUp>,
) -> Result<Resources<WithStatesDesired>, DownloadError> {
    let resources = StateDesiredCmd::exec(graph, resources).await?;
    let states_desired_serialized = {
        let states_desired = resources.borrow::<StatesDesired>();
        serde_yaml::to_string(&*states_desired).map_err(DownloadError::StatesDesiredSerialize)?
    };

    stdout_write(&states_desired_serialized).await?;
    Ok(resources)
}

async fn diff(
    graph: &FullSpecGraph<DownloadError>,
    resources: Resources<SetUp>,
) -> Result<Resources<WithStateDiffs>, DownloadError> {
    let resources = DiffCmd::exec(graph, resources).await?;
    let state_diffs_serialized = {
        let state_diffs = resources.borrow::<StateDiffs>();
        serde_yaml::to_string(&*state_diffs).map_err(DownloadError::StateDiffsSerialize)?
    };

    stdout_write(&state_diffs_serialized).await?;
    Ok(resources)
}

#[cfg_attr(target_arch = "wasm32", allow(dead_code))]
async fn ensure_dry(
    graph: &FullSpecGraph<DownloadError>,
    resources: Resources<SetUp>,
) -> Result<Resources<EnsuredDry>, DownloadError> {
    let resources = EnsureCmd::exec_dry(graph, resources).await?;
    let states_ensured_dry_serialized = {
        let states_ensured_dry = resources.borrow::<StatesEnsuredDry>();
        serde_yaml::to_string(&*states_ensured_dry).map_err(DownloadError::StateDiffsSerialize)?
    };

    stdout_write(&states_ensured_dry_serialized).await?;
    Ok(resources)
}

#[cfg_attr(target_arch = "wasm32", allow(dead_code))]
async fn ensure(
    graph: &FullSpecGraph<DownloadError>,
    resources: Resources<SetUp>,
) -> Result<Resources<Ensured>, DownloadError> {
    let resources = EnsureCmd::exec(graph, resources).await?;
    let states_ensured_serialized = {
        let states_ensured = resources.borrow::<StatesEnsured>();
        serde_yaml::to_string(&*states_ensured).map_err(DownloadError::StateDiffsSerialize)?
    };

    stdout_write(&states_ensured_serialized).await?;
    Ok(resources)
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
