#![allow(non_snake_case)] // Components are all PascalCase.

//! Components for rendering a flow.

pub use self::{flow_graph::FlowGraph, home::Home};

#[cfg(feature = "ssr")]
pub use self::flow_graph::FlowGraphSrc;

mod flow_graph;
mod home;
