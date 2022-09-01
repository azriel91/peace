//! Runtime data types for the peace automation framework.

// Re-exports
pub use fn_graph::{self, FnRef, FnRefMut};

pub use crate::{
    cmd_context::CmdContext, item_spec_boxed::ItemSpecBoxed, item_spec_graph::ItemSpecGraph,
    item_spec_graph_builder::ItemSpecGraphBuilder, item_spec_rt::ItemSpecRt,
    item_spec_wrapper::ItemSpecWrapper, workspace::Workspace,
};

#[cfg(not(target_arch = "wasm32"))]
pub use crate::{
    error::Error, workspace_dirs_builder::WorkspaceDirsBuilder, workspace_spec::WorkspaceSpec,
};

#[cfg(target_arch = "wasm32")]
pub use peace_web_support::{Error, WebStorageSpec, WorkspaceDirsBuilder};

#[cfg(not(target_arch = "wasm32"))]
mod error;
#[cfg(not(target_arch = "wasm32"))]
mod workspace_dirs_builder;
#[cfg(not(target_arch = "wasm32"))]
mod workspace_spec;

mod cmd_context;
mod item_spec_boxed;
mod item_spec_graph;
mod item_spec_graph_builder;
mod item_spec_rt;
mod item_spec_wrapper;
mod workspace;
