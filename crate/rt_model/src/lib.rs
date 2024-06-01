//! Runtime data types for the peace automation framework.
//!
//! This crate re-exports types from `peace_rt_model_native` or
//! `peace_rt_model_web` depending on the compilation target architecture.

// Re-exports
pub use peace_data::fn_graph::{self, FnRef};
pub use peace_rt_model_core::*;

#[cfg(not(target_arch = "wasm32"))]
pub use peace_rt_model_native::*;

#[cfg(target_arch = "wasm32")]
pub use peace_rt_model_web::*;

pub use crate::{
    flow::Flow, in_memory_text_output::InMemoryTextOutput, step_boxed::StepBoxed,
    step_graph::StepGraph, step_graph_builder::StepGraphBuilder, step_rt::StepRt,
    step_wrapper::StepWrapper, params_specs_serializer::ParamsSpecsSerializer,
    params_specs_type_reg::ParamsSpecsTypeReg, states_serializer::StatesSerializer,
    states_type_reg::StatesTypeReg,
};

pub mod outcomes;

mod flow;
mod in_memory_text_output;
mod step_boxed;
mod step_graph;
mod step_graph_builder;
mod step_rt;
mod step_wrapper;
mod params_specs_serializer;
mod params_specs_type_reg;
mod states_serializer;
mod states_type_reg;

#[cfg(feature = "error_reporting")]
mod yaml_error_context_hack;
