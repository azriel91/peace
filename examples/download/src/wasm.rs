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
pub struct GraphAndContent {
    graph: peace::rt_model::FullSpecGraph<DownloadError>,
    content: std::collections::HashMap<PathBuf, String>,
    pub output: String,
}

#[wasm_bindgen]
impl GraphAndContent {
    /// Returns the content of the hashmap.
    #[wasm_bindgen]
    pub fn contents(&self) -> Result<JsValue, JsValue> {
        JsValue::from_serde(&self.content).map_err(|e| JsValue::from_str(&format!("{e}")))
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

        let output = String::new();
        Ok(GraphAndContent {
            graph,
            content,
            output,
        })
    })
    .map_err(|e| JsValue::from_str(&format!("{e}")))?
    .await
}

#[wasm_bindgen]
pub async fn wasm_status(graph_and_content: GraphAndContent) -> Result<GraphAndContent, JsValue> {
    let GraphAndContent {
        graph,
        content,
        output: _,
    } = graph_and_content;
    let mut resources = graph
        .setup(Resources::new())
        .await
        .map_err(|e| JsValue::from_str(&format!("{e}")))?;
    resources.insert(content);

    let mut buffer = Vec::<u8>::with_capacity(256);
    let mut resources = status(&mut buffer, &graph, resources.into())
        .await
        .map_err(|e| JsValue::from_str(&format!("{e}")))?;
    let output = String::from_utf8(buffer).map_err(|e| JsValue::from_str(&format!("{e}")))?;

    let content = resources
        .remove::<std::collections::HashMap<PathBuf, String>>()
        .ok_or(JsValue::from_str(
            "Resources did not contain content HashMap.",
        ))?;
    Ok(GraphAndContent {
        graph,
        content,
        output,
    })
}

#[wasm_bindgen]
pub async fn wasm_desired(graph_and_content: GraphAndContent) -> Result<GraphAndContent, JsValue> {
    let GraphAndContent {
        graph,
        content,
        output: _,
    } = graph_and_content;
    let mut resources = graph
        .setup(Resources::new())
        .await
        .map_err(|e| JsValue::from_str(&format!("{e}")))?;
    resources.insert(content);

    let mut buffer = Vec::<u8>::with_capacity(256);
    let mut resources = desired(&mut buffer, &graph, resources.into())
        .await
        .map_err(|e| JsValue::from_str(&format!("{e}")))?;
    let output = String::from_utf8(buffer).map_err(|e| JsValue::from_str(&format!("{e}")))?;

    let content = resources
        .remove::<std::collections::HashMap<PathBuf, String>>()
        .ok_or(JsValue::from_str(
            "Resources did not contain content HashMap.",
        ))?;
    Ok(GraphAndContent {
        graph,
        content,
        output,
    })
}

#[wasm_bindgen]
pub async fn wasm_diff(graph_and_content: GraphAndContent) -> Result<GraphAndContent, JsValue> {
    let GraphAndContent {
        graph,
        content,
        output: _,
    } = graph_and_content;
    let mut resources = graph
        .setup(Resources::new())
        .await
        .map_err(|e| JsValue::from_str(&format!("{e}")))?;
    resources.insert(content);

    let mut buffer = Vec::<u8>::with_capacity(256);
    let mut resources = diff(&mut buffer, &graph, resources.into())
        .await
        .map_err(|e| JsValue::from_str(&format!("{e}")))?;
    let output = String::from_utf8(buffer).map_err(|e| JsValue::from_str(&format!("{e}")))?;

    let content = resources
        .remove::<std::collections::HashMap<PathBuf, String>>()
        .ok_or(JsValue::from_str(
            "Resources did not contain content HashMap.",
        ))?;
    Ok(GraphAndContent {
        graph,
        content,
        output,
    })
}

#[wasm_bindgen]
pub async fn wasm_ensure_dry(
    graph_and_content: GraphAndContent,
) -> Result<GraphAndContent, JsValue> {
    let GraphAndContent {
        graph,
        content,
        output: _,
    } = graph_and_content;
    let mut resources = graph
        .setup(Resources::new())
        .await
        .map_err(|e| JsValue::from_str(&format!("{e}")))?;
    resources.insert(content);

    let mut buffer = Vec::<u8>::with_capacity(256);
    let mut resources = ensure_dry(&mut buffer, &graph, resources.into())
        .await
        .map_err(|e| JsValue::from_str(&format!("{e}")))?;
    let output = String::from_utf8(buffer).map_err(|e| JsValue::from_str(&format!("{e}")))?;

    let content = resources
        .remove::<std::collections::HashMap<PathBuf, String>>()
        .ok_or(JsValue::from_str(
            "Resources did not contain content HashMap.",
        ))?;
    Ok(GraphAndContent {
        graph,
        content,
        output,
    })
}

#[wasm_bindgen]
pub async fn wasm_ensure(graph_and_content: GraphAndContent) -> Result<GraphAndContent, JsValue> {
    let GraphAndContent {
        graph,
        content,
        output: _,
    } = graph_and_content;
    let mut resources = graph
        .setup(Resources::new())
        .await
        .map_err(|e| JsValue::from_str(&format!("{e}")))?;
    resources.insert(content);

    let mut buffer = Vec::<u8>::with_capacity(256);
    let mut resources = ensure(&mut buffer, &graph, resources.into())
        .await
        .map_err(|e| JsValue::from_str(&format!("{e}")))?;
    let output = String::from_utf8(buffer).map_err(|e| JsValue::from_str(&format!("{e}")))?;

    let content = resources
        .remove::<std::collections::HashMap<PathBuf, String>>()
        .ok_or(JsValue::from_str(
            "Resources did not contain content HashMap.",
        ))?;
    Ok(GraphAndContent {
        graph,
        content,
        output,
    })
}
