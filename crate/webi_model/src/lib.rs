//! Web interface data types for the peace automation framework.

pub use crate::{
    cmd_exec_request::CmdExecRequest, flow_info_graphs::FlowInfoGraphs,
    flow_outcome_info_graphs::FlowOutcomeInfoGraphs,
    flow_progress_info_graphs::FlowProgressInfoGraphs, web_ui_update::WebUiUpdate,
    webi_error::WebiError,
};

mod cmd_exec_request;
mod flow_info_graphs;
mod flow_outcome_info_graphs;
mod flow_progress_info_graphs;
mod web_ui_update;
mod webi_error;
