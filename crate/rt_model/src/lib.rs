//! Runtime data types for the peace automation framework.
//!
//! This crate re-exports types from `peace_rt_model_native` or
//! `peace_rt_model_web` depending on the compilation target architecture.

// Re-exports
pub use fn_graph::{self, FnRef, FnRefMut};

pub use crate::{
    cmd_context::CmdContext, item_spec_boxed::ItemSpecBoxed, item_spec_graph::ItemSpecGraph,
    item_spec_graph_builder::ItemSpecGraphBuilder, item_spec_rt::ItemSpecRt,
    item_spec_wrapper::ItemSpecWrapper, states_type_regs::StatesTypeRegs,
};

#[cfg(not(target_arch = "wasm32"))]
pub use peace_rt_model_native::{
    Error, NativeStorage as Storage, SyncIoBridge, Workspace, WorkspaceDirsBuilder, WorkspaceSpec,
};

#[cfg(target_arch = "wasm32")]
pub use peace_rt_model_web::{
    Error, WebStorage as Storage, Workspace, WorkspaceDirsBuilder, WorkspaceSpec,
};

mod cmd_context;
mod item_spec_boxed;
mod item_spec_graph;
mod item_spec_graph_builder;
mod item_spec_rt;
mod item_spec_wrapper;
mod states_type_regs;
