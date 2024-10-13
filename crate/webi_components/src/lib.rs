#![allow(non_snake_case)] // Components are all PascalCase.

//! Web interface components for the peace automation framework.

pub use crate::{
    children_fn::ChildrenFn, flow_graph::FlowGraph, flow_graph_current::FlowGraphCurrent,
    home::Home,
};

mod children_fn;
mod flow_graph;
mod flow_graph_current;
mod home;
