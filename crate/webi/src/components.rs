#![allow(non_snake_case)] // Components are all PascalCase.

//! Components for rendering a flow.

pub use self::{flow_graph::FlowGraph, home::Home};

mod flow_graph;
mod home;
