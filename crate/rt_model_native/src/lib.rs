//! Runtime data types for the peace automation framework (native).
//!
//! Consumers should depend on the `peace_rt_model` crate, which re-exports
//! same-named types, depending on whether a native or WASM target is used.

// Re-exports
pub use tokio_util::io::SyncIoBridge;

pub use crate::{
    cli_output::CliOutput, error::Error, native_storage::NativeStorage, workspace::Workspace,
    workspace_dirs_builder::WorkspaceDirsBuilder, workspace_spec::WorkspaceSpec,
};

mod cli_output;
mod error;
mod native_storage;
mod workspace;
mod workspace_dirs_builder;
mod workspace_spec;
