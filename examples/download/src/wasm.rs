use std::path::PathBuf;

use peace::resources::Resources;
use url::Url;

pub use crate::{
    desired, diff, ensure, ensure_dry, setup_graph, status, DownloadArgs, DownloadCleanOpSpec,
    DownloadCommand, DownloadEnsureOpSpec, DownloadError, DownloadFullSpec, DownloadParams,
    DownloadStateCurrentFnSpec, DownloadStateDesiredFnSpec, DownloadStateDiffFnSpec, FileState,
    FileStateDiff,
};

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen(getter_with_clone)]
pub struct GraphAndContent(
    peace::rt_model::FullSpecGraph<DownloadError>,
    std::collections::HashMap<PathBuf, String>,
);

#[wasm_bindgen]
impl GraphAndContent {
    /// Returns the content of the hashmap.
    #[wasm_bindgen]
    pub fn contents(&self) -> Result<JsValue, JsValue> {
        JsValue::from_serde(&self.1).map_err(|e| JsValue::from_str(&format!("{e}")))
    }
}

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

pub async fn stdout_write(s: &str) -> Result<(), DownloadError> {
    log(s);
    Ok(())
}
