//! Runtime data types for the peace automation framework.
//!
//! This crate re-exports types from `peace_rt_model_native` or
//! `peace_rt_model_web` depending on the compilation target architecture.

// Re-exports
pub use fn_graph::{self, FnRef};
pub use peace_rt_model_core::*;

pub mod output {
    pub use peace_rt_model_core::output::*;

    #[cfg(not(target_arch = "wasm32"))]
    pub use peace_rt_model_native::output::*;
}

#[cfg(not(target_arch = "wasm32"))]
pub use peace_rt_model_native::*;

#[cfg(target_arch = "wasm32")]
pub use peace_rt_model_web::*;

pub use crate::{
    flow::Flow, in_memory_text_output::InMemoryTextOutput, item_spec_boxed::ItemSpecBoxed,
    item_spec_graph::ItemSpecGraph, item_spec_graph_builder::ItemSpecGraphBuilder,
    item_spec_params::ItemSpecParams, item_spec_params_type_reg::ItemSpecParamsTypeReg,
    item_spec_rt::ItemSpecRt, item_spec_wrapper::ItemSpecWrapper,
    states_serializer::StatesSerializer, states_type_reg::StatesTypeReg,
};

pub mod outcomes;

mod flow;
mod in_memory_text_output;
mod item_spec_boxed;
mod item_spec_graph;
mod item_spec_graph_builder;
mod item_spec_params;
mod item_spec_params_type_reg;
mod item_spec_rt;
mod item_spec_wrapper;
mod states_serializer;
mod states_type_reg;
