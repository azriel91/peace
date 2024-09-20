#![allow(non_snake_case)] // Components are all PascalCase.

//! Web interface components for the peace automation framework.

pub use crate::{children_fn::ChildrenFn, flow_graph::FlowGraph, home::Home};

mod children_fn;
mod flow_graph;
mod home;
