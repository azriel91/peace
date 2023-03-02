use peace::{
    cfg::{flow_id, profile, FlowId, Profile},
    rt_model::{InMemoryTextOutput, WorkspaceSpec},
};
use peace_item_specs::file_download::{FileDownloadParams, StorageForm};
use url::Url;
use wasm_bindgen::prelude::*;

pub use crate::{
    clean, clean_dry, cmd_context, desired, diff, ensure, ensure_dry, fetch, status,
    workspace_and_flow_setup, DownloadError, WorkspaceAndFlow,
};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen(getter_with_clone)]
pub struct WorkspaceAndOutput {
    workspace_and_flow: WorkspaceAndFlow,
    pub output: String,
}

#[wasm_bindgen]
pub async fn wasm_init(url: String, name: String) -> Result<WorkspaceAndOutput, JsValue> {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    let workspace_and_output =
        workspace_and_flow_setup(WorkspaceSpec::SessionStorage, flow_id!("file"))
            .await
            .map(|workspace_and_flow| async move {
                let output = String::new();

                Result::<_, JsValue>::Ok(WorkspaceAndOutput {
                    workspace_and_flow,
                    output,
                })
            })
            .map_err(into_js_err_value)?
            .await?;

    // Store init params in storage.
    let file_download_params = {
        let url = Url::parse(&url).expect("Failed to parse URL.");
        let dest = std::path::PathBuf::from(name);
        FileDownloadParams::new(url, dest, StorageForm::Text)
    };

    // Building the command context currently serializes the init params to storage.
    let WorkspaceAndOutput {
        workspace_and_flow,
        output: _,
    } = workspace_and_output;
    let mut in_memory_text_output = InMemoryTextOutput::new();
    let _cmd_context = cmd_context(
        &workspace_and_flow,
        profile!("default"),
        &mut in_memory_text_output,
        Some(file_download_params),
    )
    .await
    .map_err(into_js_err_value)?;

    let output = in_memory_text_output.into_inner();

    Ok(WorkspaceAndOutput {
        workspace_and_flow,
        output,
    })
}

#[wasm_bindgen]
pub async fn wasm_fetch(
    workspace_and_output: WorkspaceAndOutput,
) -> Result<WorkspaceAndOutput, JsValue> {
    let WorkspaceAndOutput {
        workspace_and_flow,
        output: _,
    } = workspace_and_output;
    let mut in_memory_text_output = InMemoryTextOutput::new();
    let cmd_context = cmd_context(
        &workspace_and_flow,
        profile!("default"),
        &mut in_memory_text_output,
        None,
    )
    .await
    .map_err(into_js_err_value)?;

    fetch(cmd_context).await.map_err(into_js_err_value)?;
    let output = in_memory_text_output.into_inner();

    Ok(WorkspaceAndOutput {
        workspace_and_flow,
        output,
    })
}

#[wasm_bindgen]
pub async fn wasm_status(
    workspace_and_output: WorkspaceAndOutput,
) -> Result<WorkspaceAndOutput, JsValue> {
    let WorkspaceAndOutput {
        workspace_and_flow,
        output: _,
    } = workspace_and_output;
    let mut in_memory_text_output = InMemoryTextOutput::new();
    let cmd_context = cmd_context(
        &workspace_and_flow,
        profile!("default"),
        &mut in_memory_text_output,
        None,
    )
    .await
    .map_err(into_js_err_value)?;

    status(cmd_context).await.map_err(into_js_err_value)?;
    let output = in_memory_text_output.into_inner();

    Ok(WorkspaceAndOutput {
        workspace_and_flow,
        output,
    })
}

#[wasm_bindgen]
pub async fn wasm_desired(
    workspace_and_output: WorkspaceAndOutput,
) -> Result<WorkspaceAndOutput, JsValue> {
    let WorkspaceAndOutput {
        workspace_and_flow,
        output: _,
    } = workspace_and_output;
    let mut in_memory_text_output = InMemoryTextOutput::new();
    let cmd_context = cmd_context(
        &workspace_and_flow,
        profile!("default"),
        &mut in_memory_text_output,
        None,
    )
    .await
    .map_err(into_js_err_value)?;

    desired(cmd_context).await.map_err(into_js_err_value)?;
    let output = in_memory_text_output.into_inner();

    Ok(WorkspaceAndOutput {
        workspace_and_flow,
        output,
    })
}

#[wasm_bindgen]
pub async fn wasm_diff(
    workspace_and_output: WorkspaceAndOutput,
) -> Result<WorkspaceAndOutput, JsValue> {
    let WorkspaceAndOutput {
        workspace_and_flow,
        output: _,
    } = workspace_and_output;
    let mut in_memory_text_output = InMemoryTextOutput::new();
    let cmd_context = cmd_context(
        &workspace_and_flow,
        profile!("default"),
        &mut in_memory_text_output,
        None,
    )
    .await
    .map_err(into_js_err_value)?;

    diff(cmd_context).await.map_err(into_js_err_value)?;
    let output = in_memory_text_output.into_inner();

    Ok(WorkspaceAndOutput {
        workspace_and_flow,
        output,
    })
}

#[wasm_bindgen]
pub async fn wasm_ensure_dry(
    workspace_and_output: WorkspaceAndOutput,
) -> Result<WorkspaceAndOutput, JsValue> {
    let WorkspaceAndOutput {
        workspace_and_flow,
        output: _,
    } = workspace_and_output;
    let mut in_memory_text_output = InMemoryTextOutput::new();
    let cmd_context = cmd_context(
        &workspace_and_flow,
        profile!("default"),
        &mut in_memory_text_output,
        None,
    )
    .await
    .map_err(into_js_err_value)?;

    ensure_dry(cmd_context).await.map_err(into_js_err_value)?;
    let output = in_memory_text_output.into_inner();

    Ok(WorkspaceAndOutput {
        workspace_and_flow,
        output,
    })
}

#[wasm_bindgen]
pub async fn wasm_ensure(
    workspace_and_output: WorkspaceAndOutput,
) -> Result<WorkspaceAndOutput, JsValue> {
    let WorkspaceAndOutput {
        workspace_and_flow,
        output: _,
    } = workspace_and_output;
    let mut in_memory_text_output = InMemoryTextOutput::new();
    let cmd_context = cmd_context(
        &workspace_and_flow,
        profile!("default"),
        &mut in_memory_text_output,
        None,
    )
    .await
    .map_err(into_js_err_value)?;

    ensure(cmd_context).await.map_err(into_js_err_value)?;
    let output = in_memory_text_output.into_inner();

    Ok(WorkspaceAndOutput {
        workspace_and_flow,
        output,
    })
}

#[wasm_bindgen]
pub async fn wasm_clean_dry(
    workspace_and_output: WorkspaceAndOutput,
) -> Result<WorkspaceAndOutput, JsValue> {
    let WorkspaceAndOutput {
        workspace_and_flow,
        output: _,
    } = workspace_and_output;
    let mut in_memory_text_output = InMemoryTextOutput::new();
    let cmd_context = cmd_context(
        &workspace_and_flow,
        profile!("default"),
        &mut in_memory_text_output,
        None,
    )
    .await
    .map_err(into_js_err_value)?;

    clean_dry(cmd_context).await.map_err(into_js_err_value)?;
    let output = in_memory_text_output.into_inner();

    Ok(WorkspaceAndOutput {
        workspace_and_flow,
        output,
    })
}

#[wasm_bindgen]
pub async fn wasm_clean(
    workspace_and_output: WorkspaceAndOutput,
) -> Result<WorkspaceAndOutput, JsValue> {
    let WorkspaceAndOutput {
        workspace_and_flow,
        output: _,
    } = workspace_and_output;
    let mut in_memory_text_output = InMemoryTextOutput::new();
    let cmd_context = cmd_context(
        &workspace_and_flow,
        profile!("default"),
        &mut in_memory_text_output,
        None,
    )
    .await
    .map_err(into_js_err_value)?;

    clean(cmd_context).await.map_err(into_js_err_value)?;
    let output = in_memory_text_output.into_inner();

    Ok(WorkspaceAndOutput {
        workspace_and_flow,
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
