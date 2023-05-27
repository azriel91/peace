#![allow(non_snake_case)] // Components in rsx are all PascalCase.

//! Components for rendering a flow.
//!
//! See <https://github.com/DioxusLabs/dioxus/blob/master/packages/html/src/elements.rs> for the
//! elements that can be placed in the `rsx!` macro calls.

pub use self::{
    flow_graph::{FlowGraph, FlowGraphProps},
    home::{Home, HomeProps},
};

mod flow_graph;
mod home;
