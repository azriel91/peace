//! Runtime data types for the peace automation framework.

pub use crate::{
    full_spec_boxed::FullSpecBoxed, full_spec_graph::FullSpecGraph,
    full_spec_graph_builder::FullSpecGraphBuilder, full_spec_rt::FullSpecRt,
    full_spec_wrapper::FullSpecWrapper,
};

mod full_spec_boxed;
mod full_spec_graph;
mod full_spec_graph_builder;
mod full_spec_rt;
mod full_spec_wrapper;
