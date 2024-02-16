//! Flow data model for the peace automation framework.
//!
//! This includes the serializable representation of a `Flow`. Since an actual
//! `Flow` contains logic, it currently resides in `peace_rt_model`.

pub use crate::{
    flow_info::FlowInfo, flow_spec_info::FlowSpecInfo, item_info::ItemInfo,
    item_spec_info::ItemSpecInfo,
};

mod flow_info;
mod flow_spec_info;
mod item_info;
mod item_spec_info;
