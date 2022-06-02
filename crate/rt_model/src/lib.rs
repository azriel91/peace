//! Runtime data types for the peace automation framework.

pub use crate::{
    error::Error, full_spec_boxed::FullSpecBoxed, full_spec_graph::FullSpecGraph,
    full_spec_graph_builder::FullSpecGraphBuilder,
};

mod error;
mod full_spec_boxed;
mod full_spec_graph;
mod full_spec_graph_builder;
