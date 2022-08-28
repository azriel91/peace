use std::path::PathBuf;

use peace::{
    cfg::{profile, Profile},
    rt_model::{Workspace, WorkspaceSpec},
};
use url::Url;

pub use crate::{
    desired, diff, ensure, ensure_dry, setup_workspace, status, DownloadArgs, DownloadCleanOpSpec,
    DownloadCommand, DownloadEnsureOpSpec, DownloadError, DownloadItemSpec, DownloadParams,
    DownloadStateCurrentFnSpec, DownloadStateDesiredFnSpec, DownloadStateDiffFnSpec, FileState,
    FileStateDiff,
};

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

// Need to create a concrete type for each `Workspace<TS, _>` as wasm_bindgen
// does not support type parameterized types.
macro_rules! workspace_and_content {
    ($name:ident, $type_state:ident) => {
        #[wasm_bindgen(getter_with_clone)]
        pub struct $name {
            // Only the SetUp type state's workspace is read
            #[allow(dead_code)]
            workspace:
                Workspace<peace::resources::resources_type_state::$type_state, DownloadError>,
            content: std::collections::HashMap<PathBuf, String>,
            pub output: String,
        }

        #[wasm_bindgen]
        impl $name {
            /// Returns the content of the hashmap.
            #[wasm_bindgen]
            pub fn contents(&self) -> Result<JsValue, JsValue> {
                JsValue::from_serde(&self.content).map_err(into_js_err_value)
            }
        }
    };
}

workspace_and_content!(WorkspaceAndContentSetUp, SetUp);
workspace_and_content!(WorkspaceAndContentWithStates, WithStates);
workspace_and_content!(WorkspaceAndContentWithStatesDesired, WithStatesDesired);
workspace_and_content!(WorkspaceAndContentWithStateDiffs, WithStateDiffs);
workspace_and_content!(WorkspaceAndContentEnsuredDry, EnsuredDry);
workspace_and_content!(WorkspaceAndContentEnsured, Ensured);

#[wasm_bindgen]
pub async fn wasm_setup(url: String, name: String) -> Result<WorkspaceAndContentSetUp, JsValue> {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    let workspace_spec = &WorkspaceSpec::WorkingDir;
    let profile = profile!("default");
    setup_workspace(
        workspace_spec,
        profile,
        Url::parse(&url).expect("Failed to parse URL."),
        std::path::PathBuf::from(name),
    )
    .await
    .map(|mut workspace| async move {
        let resources = workspace.resources_mut();
        let content = resources
            .remove::<std::collections::HashMap<PathBuf, String>>()
            .ok_or(JsValue::from_str(
                "Resources did not contain content HashMap.",
            ))?;

        let output = String::new();
        Ok(WorkspaceAndContentSetUp {
            workspace,
            content,
            output,
        })
    })
    .map_err(into_js_err_value)?
    .await
}

#[wasm_bindgen]
pub async fn wasm_status(
    workspace_and_content: WorkspaceAndContentSetUp,
) -> Result<WorkspaceAndContentWithStates, JsValue> {
    let WorkspaceAndContentSetUp {
        mut workspace,
        content,
        output: _,
    } = workspace_and_content;
    let resources = workspace.resources_mut();
    resources.insert(content);

    let mut buffer = Vec::<u8>::with_capacity(256);
    let mut workspace = status(&mut buffer, workspace)
        .await
        .map_err(into_js_err_value)?;
    let output = String::from_utf8(buffer).map_err(into_js_err_value)?;

    let resources = workspace.resources_mut();
    let content = resources
        .remove::<std::collections::HashMap<PathBuf, String>>()
        .ok_or(JsValue::from_str(
            "Resources did not contain content HashMap.",
        ))?;
    Ok(WorkspaceAndContentWithStates {
        workspace,
        content,
        output,
    })
}

#[wasm_bindgen]
pub async fn wasm_desired(
    workspace_and_content: WorkspaceAndContentSetUp,
) -> Result<WorkspaceAndContentWithStatesDesired, JsValue> {
    let WorkspaceAndContentSetUp {
        mut workspace,
        content,
        output: _,
    } = workspace_and_content;
    let resources = workspace.resources_mut();
    resources.insert(content);

    let mut buffer = Vec::<u8>::with_capacity(256);
    let mut workspace = desired(&mut buffer, workspace)
        .await
        .map_err(into_js_err_value)?;
    let output = String::from_utf8(buffer).map_err(into_js_err_value)?;

    let resources = workspace.resources_mut();
    let content = resources
        .remove::<std::collections::HashMap<PathBuf, String>>()
        .ok_or(JsValue::from_str(
            "Resources did not contain content HashMap.",
        ))?;
    Ok(WorkspaceAndContentWithStatesDesired {
        workspace,
        content,
        output,
    })
}

#[wasm_bindgen]
pub async fn wasm_diff(
    workspace_and_content: WorkspaceAndContentSetUp,
) -> Result<WorkspaceAndContentWithStateDiffs, JsValue> {
    let WorkspaceAndContentSetUp {
        mut workspace,
        content,
        output: _,
    } = workspace_and_content;
    let resources = workspace.resources_mut();
    resources.insert(content);

    let mut buffer = Vec::<u8>::with_capacity(256);
    let mut workspace = diff(&mut buffer, workspace)
        .await
        .map_err(into_js_err_value)?;
    let output = String::from_utf8(buffer).map_err(into_js_err_value)?;

    let resources = workspace.resources_mut();
    let content = resources
        .remove::<std::collections::HashMap<PathBuf, String>>()
        .ok_or(JsValue::from_str(
            "Resources did not contain content HashMap.",
        ))?;
    Ok(WorkspaceAndContentWithStateDiffs {
        workspace,
        content,
        output,
    })
}

#[wasm_bindgen]
pub async fn wasm_ensure_dry(
    workspace_and_content: WorkspaceAndContentSetUp,
) -> Result<WorkspaceAndContentEnsuredDry, JsValue> {
    let WorkspaceAndContentSetUp {
        mut workspace,
        content,
        output: _,
    } = workspace_and_content;
    let resources = workspace.resources_mut();
    resources.insert(content);

    let mut buffer = Vec::<u8>::with_capacity(256);
    let mut workspace = ensure_dry(&mut buffer, workspace)
        .await
        .map_err(into_js_err_value)?;
    let output = String::from_utf8(buffer).map_err(into_js_err_value)?;

    let resources = workspace.resources_mut();
    let content = resources
        .remove::<std::collections::HashMap<PathBuf, String>>()
        .ok_or(JsValue::from_str(
            "Resources did not contain content HashMap.",
        ))?;
    Ok(WorkspaceAndContentEnsuredDry {
        workspace,
        content,
        output,
    })
}

#[wasm_bindgen]
pub async fn wasm_ensure(
    workspace_and_content: WorkspaceAndContentSetUp,
) -> Result<WorkspaceAndContentEnsured, JsValue> {
    let WorkspaceAndContentSetUp {
        mut workspace,
        content,
        output: _,
    } = workspace_and_content;
    let resources = workspace.resources_mut();
    resources.insert(content);

    let mut buffer = Vec::<u8>::with_capacity(256);
    let mut workspace = ensure(&mut buffer, workspace)
        .await
        .map_err(into_js_err_value)?;
    let output = String::from_utf8(buffer).map_err(into_js_err_value)?;

    let resources = workspace.resources_mut();
    let content = resources
        .remove::<std::collections::HashMap<PathBuf, String>>()
        .ok_or(JsValue::from_str(
            "Resources did not contain content HashMap.",
        ))?;
    Ok(WorkspaceAndContentEnsured {
        workspace,
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
