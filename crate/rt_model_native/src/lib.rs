#![cfg_attr(coverage_nightly, feature(coverage_attribute))]

//! Runtime data types for the peace automation framework (native).
//!
//! Consumers should depend on the `peace_rt_model` crate, which re-exports
//! same-named types, depending on whether a native or WASM target is used.

// Re-exports
pub use tokio_util::io::SyncIoBridge;

pub use crate::{
    storage::Storage, workspace::Workspace, workspace_dirs_builder::WorkspaceDirsBuilder,
    workspace_initializer::WorkspaceInitializer, workspace_spec::WorkspaceSpec,
};

pub mod output;
pub mod workspace;

mod storage;
mod workspace_dirs_builder;
mod workspace_initializer;
mod workspace_spec;
