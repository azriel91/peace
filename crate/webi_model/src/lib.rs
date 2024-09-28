//! Web interface data types for the peace automation framework.

pub use crate::{
    cmd_exec_request::CmdExecRequest, flow_info_graphs::FlowInfoGraphs,
    flow_outcome_info_graphs::FlowOutcomeInfoGraphs,
    flow_progress_info_graphs::FlowProgressInfoGraphs,
    outcome_info_graph_variant::OutcomeInfoGraphVariant,
    progress_info_graph_variant::ProgressInfoGraphVariant, web_ui_update::WebUiUpdate,
    webi_error::WebiError,
};

mod cmd_exec_request;
mod flow_info_graphs;
mod flow_outcome_info_graphs;
mod flow_progress_info_graphs;
mod outcome_info_graph_variant;
mod progress_info_graph_variant;
mod web_ui_update;
mod webi_error;
