#![allow(non_snake_case)] // Components are all PascalCase.

//! Web interface components for the peace automation framework.

pub use crate::{flow_graph::FlowGraph, home::Home};

mod flow_graph;
mod home;
