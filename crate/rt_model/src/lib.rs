//! Runtime data types for the peace automation framework.

// Re-exports
pub use rt_vec;

pub use crate::{
    error::Error,
    full_spec_boxed::{FullSpecBoxed, FullSpecRt},
    full_spec_graph::FullSpecGraph,
    full_spec_graph_builder::FullSpecGraphBuilder,
    full_spec_resourceses::FullSpecResourceses,
    full_spec_rt_id::FullSpecRtId,
    full_spec_wrapper::FullSpecWrapper,
};

mod error;
mod full_spec_boxed;
mod full_spec_graph;
mod full_spec_graph_builder;
mod full_spec_resourceses;
mod full_spec_rt_id;
mod full_spec_wrapper;
