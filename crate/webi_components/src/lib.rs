#![allow(non_snake_case)] // Components are all PascalCase.

//! Web interface components for the peace automation framework.

pub use crate::{arc_mut_cmd_ctx_spsf::ArcMutCmdCtxSpsf, flow_graph::FlowGraph, home::Home};

mod arc_mut_cmd_ctx_spsf;
mod flow_graph;
mod home;
