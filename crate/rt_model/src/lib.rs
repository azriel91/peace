//! Runtime data types for the peace automation framework.

// Re-exports
pub use fn_graph::{self, FnRef, FnRefMut};

pub use crate::{
    cmd_context::CmdContext, error::Error, item_spec_boxed::ItemSpecBoxed,
    item_spec_graph::ItemSpecGraph, item_spec_graph_builder::ItemSpecGraphBuilder,
    item_spec_rt::ItemSpecRt, item_spec_wrapper::ItemSpecWrapper, workspace::Workspace,
    workspace_dirs::WorkspaceDirs, workspace_dirs_builder::WorkspaceDirsBuilder,
    workspace_spec::WorkspaceSpec,
};

mod cmd_context;
mod error;
mod item_spec_boxed;
mod item_spec_graph;
mod item_spec_graph_builder;
mod item_spec_rt;
mod item_spec_wrapper;
mod workspace;
mod workspace_dirs;
mod workspace_dirs_builder;
mod workspace_spec;
