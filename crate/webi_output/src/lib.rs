//! Web interface output for the peace automation framework.

pub use crate::{
    cmd_exec_spawn_ctx::CmdExecSpawnCtx, cmd_exec_to_leptos_ctx::CmdExecToLeptosCtx,
    flow_webi_fns::FlowWebiFns, outcome_info_graph_calculator::OutcomeInfoGraphCalculator,
    webi_output::WebiOutput, webi_server::WebiServer,
};

pub mod assets;

mod cmd_exec_spawn_ctx;
mod cmd_exec_to_leptos_ctx;
mod flow_webi_fns;
mod outcome_info_graph_calculator;
mod webi_output;
mod webi_server;
