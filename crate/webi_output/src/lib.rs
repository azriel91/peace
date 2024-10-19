//! Web interface output for the peace automation framework.

pub use crate::{
    cmd_exec_spawn_ctx::CmdExecSpawnCtx, cmd_exec_to_leptos_ctx::CmdExecToLeptosCtx,
    flow_webi_fns::FlowWebiFns, webi_output::WebiOutput, webi_server::WebiServer,
};

#[cfg(feature = "item_interactions")]
pub use crate::outcome_info_graph_calculator::OutcomeInfoGraphCalculator;

#[cfg(feature = "output_progress")]
pub use crate::progress_info_graph_calculator::ProgressInfoGraphCalculator;

pub mod assets;

mod cmd_exec_spawn_ctx;
mod cmd_exec_to_leptos_ctx;
mod flow_webi_fns;
mod webi_output;
mod webi_server;

#[cfg(feature = "item_interactions")]
mod outcome_info_graph_calculator;

#[cfg(feature = "output_progress")]
mod progress_info_graph_calculator;
