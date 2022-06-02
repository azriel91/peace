//! Runtime data types for the peace automation framework.

pub use crate::{
    full_spec_boxed::FullSpecBoxed, full_spec_graph::FullSpecGraph,
    full_spec_graph_builder::FullSpecGraphBuilder,
};

mod full_spec_boxed;
mod full_spec_graph;
mod full_spec_graph_builder;
