// #![cfg(target_arch = "wasm32")]
//! Web support for the peace automation framework.
//!
//! **This crate is intended to be used with `#[cfg(target_arch = "wasm32")]`.**
//!
//! Contains types and logic to make it easier to compile a peace tool to web
//! assembly.

pub use crate::{
    error::Error, web_storage::WebStorage, web_storage_spec::WebStorageSpec,
    workspace_dirs_builder::WorkspaceDirsBuilder,
};

mod error;
mod web_storage;
mod web_storage_spec;
mod workspace_dirs_builder;

/// Converts the `JsValue` to a `String` to allow `Error` to be `Send`.
pub fn stringify_js_value(js_value: wasm_bindgen::JsValue) -> String {
    js_value
        .into_serde::<String>()
        .unwrap_or_else(|_| String::from("<??>"))
}
