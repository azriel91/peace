//! Flow data model for the peace automation framework.
//!
//! This includes the serializable representation of a `Flow`. Since an actual
//! `Flow` contains logic, it currently resides in `peace_rt_model`.

// Re-exports;
pub use dot_ix;
pub use fn_graph::GraphInfo;

pub use crate::{
    flow_info::FlowInfo, flow_spec_info::FlowSpecInfo, step_info::StepInfo,
    step_spec_info::StepSpecInfo,
};

mod flow_info;
mod flow_spec_info;
mod step_info;
mod step_spec_info;
