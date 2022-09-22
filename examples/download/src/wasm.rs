use std::{collections::HashMap, path::PathBuf};

use peace::{
    cfg::{flow_id, profile, FlowId, Profile},
    rt_model::{InMemoryTextOutput, WorkspaceSpec},
};
use url::Url;
use wasm_bindgen::prelude::*;

pub use crate::{
    cmd_context, desired, diff, ensure, ensure_dry, fetch, status, workspace_and_graph_setup,
    workspace_init, DownloadArgs, DownloadCleanOpSpec, DownloadCommand, DownloadEnsureOpSpec,
    DownloadError, DownloadItemSpec, DownloadParams, DownloadStateCurrentFnSpec,
    DownloadStateDesiredFnSpec, DownloadStateDiffFnSpec, FileState, FileStateDiff,
    WorkspaceAndGraph,
};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen(getter_with_clone)]
pub struct WorkspaceAndContent {
    workspace_and_graph: WorkspaceAndGraph,
    content: std::collections::HashMap<PathBuf, String>,
    pub output: String,
}

#[wasm_bindgen]
impl WorkspaceAndContent {
    /// Returns the content of the hashmap.
    #[wasm_bindgen]
    pub fn contents(&self) -> Result<JsValue, JsValue> {
        JsValue::from_serde(&self.content).map_err(into_js_err_value)
    }
}

#[wasm_bindgen]
pub async fn wasm_init(url: String, name: String) -> Result<WorkspaceAndContent, JsValue> {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    workspace_init(
        WorkspaceSpec::SessionStorage,
        profile!("default"),
        flow_id!("file"),
        Url::parse(&url).expect("Failed to parse URL."),
        std::path::PathBuf::from(name),
    )
    .await
    .map(|workspace_and_graph| async move {
        let content = HashMap::new();
        let output = String::new();

        Ok(WorkspaceAndContent {
            workspace_and_graph,
            content,
            output,
        })
    })
    .map_err(into_js_err_value)?
    .await
}

#[wasm_bindgen]
pub async fn wasm_fetch(
    workspace_and_content: WorkspaceAndContent,
) -> Result<WorkspaceAndContent, JsValue> {
    let WorkspaceAndContent {
        workspace_and_graph,
        content,
        output: _,
    } = workspace_and_content;
    let mut in_memory_text_output = InMemoryTextOutput::new();
    let mut cmd_context = cmd_context(&workspace_and_graph, &mut in_memory_text_output)
        .await
        .map_err(into_js_err_value)?;
    let resources = cmd_context.resources_mut();
    resources.insert(content);

    let mut resources = fetch(cmd_context).await.map_err(into_js_err_value)?;
    let output = in_memory_text_output.into_inner();

    let content = resources
        .remove::<std::collections::HashMap<PathBuf, String>>()
        .ok_or(JsValue::from_str(
            "Resources did not contain content HashMap.",
        ))?;
    Ok(WorkspaceAndContent {
        workspace_and_graph,
        content,
        output,
    })
}

#[wasm_bindgen]
pub async fn wasm_status(
    workspace_and_content: WorkspaceAndContent,
) -> Result<WorkspaceAndContent, JsValue> {
    let WorkspaceAndContent {
        workspace_and_graph,
        content,
        output: _,
    } = workspace_and_content;
    let mut in_memory_text_output = InMemoryTextOutput::new();
    let mut cmd_context = cmd_context(&workspace_and_graph, &mut in_memory_text_output)
        .await
        .map_err(into_js_err_value)?;
    let resources = cmd_context.resources_mut();
    resources.insert(content);

    let mut resources = status(cmd_context).await.map_err(into_js_err_value)?;
    let output = in_memory_text_output.into_inner();

    let content = resources
        .remove::<std::collections::HashMap<PathBuf, String>>()
        .ok_or(JsValue::from_str(
            "Resources did not contain content HashMap.",
        ))?;
    Ok(WorkspaceAndContent {
        workspace_and_graph,
        content,
        output,
    })
}

#[wasm_bindgen]
pub async fn wasm_desired(
    workspace_and_content: WorkspaceAndContent,
) -> Result<WorkspaceAndContent, JsValue> {
    let WorkspaceAndContent {
        workspace_and_graph,
        content,
        output: _,
    } = workspace_and_content;
    let mut in_memory_text_output = InMemoryTextOutput::new();
    let mut cmd_context = cmd_context(&workspace_and_graph, &mut in_memory_text_output)
        .await
        .map_err(into_js_err_value)?;
    let resources = cmd_context.resources_mut();
    resources.insert(content);

    let mut resources = desired(cmd_context).await.map_err(into_js_err_value)?;
    let output = in_memory_text_output.into_inner();

    let content = resources
        .remove::<std::collections::HashMap<PathBuf, String>>()
        .ok_or(JsValue::from_str(
            "Resources did not contain content HashMap.",
        ))?;
    Ok(WorkspaceAndContent {
        workspace_and_graph,
        content,
        output,
    })
}

#[wasm_bindgen]
pub async fn wasm_diff(
    workspace_and_content: WorkspaceAndContent,
) -> Result<WorkspaceAndContent, JsValue> {
    let WorkspaceAndContent {
        workspace_and_graph,
        content,
        output: _,
    } = workspace_and_content;
    let mut in_memory_text_output = InMemoryTextOutput::new();
    let mut cmd_context = cmd_context(&workspace_and_graph, &mut in_memory_text_output)
        .await
        .map_err(into_js_err_value)?;
    let resources = cmd_context.resources_mut();
    resources.insert(content);

    let mut resources = diff(cmd_context).await.map_err(into_js_err_value)?;
    let output = in_memory_text_output.into_inner();

    let content = resources
        .remove::<std::collections::HashMap<PathBuf, String>>()
        .ok_or(JsValue::from_str(
            "Resources did not contain content HashMap.",
        ))?;
    Ok(WorkspaceAndContent {
        workspace_and_graph,
        content,
        output,
    })
}

#[wasm_bindgen]
pub async fn wasm_ensure_dry(
    workspace_and_content: WorkspaceAndContent,
) -> Result<WorkspaceAndContent, JsValue> {
    let WorkspaceAndContent {
        workspace_and_graph,
        content,
        output: _,
    } = workspace_and_content;
    let mut in_memory_text_output = InMemoryTextOutput::new();
    let mut cmd_context = cmd_context(&workspace_and_graph, &mut in_memory_text_output)
        .await
        .map_err(into_js_err_value)?;
    let resources = cmd_context.resources_mut();
    resources.insert(content);

    let mut resources = ensure_dry(cmd_context).await.map_err(into_js_err_value)?;
    let output = in_memory_text_output.into_inner();

    let content = resources
        .remove::<std::collections::HashMap<PathBuf, String>>()
        .ok_or(JsValue::from_str(
            "Resources did not contain content HashMap.",
        ))?;
    Ok(WorkspaceAndContent {
        workspace_and_graph,
        content,
        output,
    })
}

#[wasm_bindgen]
pub async fn wasm_ensure(
    workspace_and_content: WorkspaceAndContent,
) -> Result<WorkspaceAndContent, JsValue> {
    let WorkspaceAndContent {
        workspace_and_graph,
        content,
        output: _,
    } = workspace_and_content;
    let mut in_memory_text_output = InMemoryTextOutput::new();
    let mut cmd_context = cmd_context(&workspace_and_graph, &mut in_memory_text_output)
        .await
        .map_err(into_js_err_value)?;
    let resources = cmd_context.resources_mut();
    resources.insert(content);

    let mut resources = ensure(cmd_context).await.map_err(into_js_err_value)?;
    let output = in_memory_text_output.into_inner();

    let content = resources
        .remove::<std::collections::HashMap<PathBuf, String>>()
        .ok_or(JsValue::from_str(
            "Resources did not contain content HashMap.",
        ))?;
    Ok(WorkspaceAndContent {
        workspace_and_graph,
        content,
        output,
    })
}

fn into_js_err_value<E>(e: E) -> JsValue
where
    E: std::error::Error,
{
    use std::{error::Error, fmt::Write};

    let mut buffer = String::with_capacity(256);
    writeln!(&mut buffer, "{e}").unwrap();

    let mut source_opt: Option<&(dyn Error + 'static)> = e.source();
    while let Some(source) = source_opt {
        writeln!(&mut buffer, "Caused by:").unwrap();
        writeln!(&mut buffer, "{source}").unwrap();
        source_opt = source.source();
    }

    JsValue::from_str(&buffer)
}
