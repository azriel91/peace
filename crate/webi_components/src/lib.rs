#![allow(non_snake_case)] // Components are all PascalCase.

//! Web interface components for the peace automation framework.

pub use leptos;

pub use crate::{
    app::App, children_fn::ChildrenFn, flow_graph::FlowGraph, flow_graph_current::FlowGraphCurrent,
    shell::Shell,
};

mod app;
mod children_fn;
mod flow_graph;
mod flow_graph_current;
mod shell;
