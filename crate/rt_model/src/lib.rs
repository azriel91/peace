//! Runtime data types for the peace automation framework.

// Re-exports
pub use fn_graph::{self, FnRef, FnRefMut};

pub use crate::{
    item_spec_boxed::ItemSpecBoxed, item_spec_graph::ItemSpecGraph,
    item_spec_graph_builder::ItemSpecGraphBuilder, item_spec_rt::ItemSpecRt,
    item_spec_wrapper::ItemSpecWrapper,
};

mod item_spec_boxed;
mod item_spec_graph;
mod item_spec_graph_builder;
mod item_spec_rt;
mod item_spec_wrapper;
