// #![cfg(target_arch = "wasm32")]
//! Web support for the peace automation framework.
//!
//! Consumers should depend on the `peace_rt_model` crate, which re-exports
//! same-named types, depending on whether a native or WASM target is used.
//!
//! **This crate is intended to be used with `#[cfg(target_arch = "wasm32")]`.**

pub use crate::{
    error::Error, web_storage::WebStorage, workspace::Workspace,
    workspace_dirs_builder::WorkspaceDirsBuilder, workspace_spec::WorkspaceSpec,
};

mod error;
mod web_storage;
mod workspace;
mod workspace_dirs_builder;
mod workspace_spec;

/// Converts the `JsValue` to a `String` to allow `Error` to be `Send`.
pub fn stringify_js_value(js_value: wasm_bindgen::JsValue) -> String {
    js_value
        .into_serde::<String>()
        .unwrap_or_else(|_| String::from("<??>"))
}