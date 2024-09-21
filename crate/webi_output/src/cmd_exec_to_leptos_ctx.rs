use interruptible::InterruptSignal;
use peace_cmd_model::CmdExecutionId;
use peace_core::FlowId;
use std::collections::HashMap;
use tokio::sync::mpsc;

use peace_webi_model::FlowInfoGraphs;

/// The shared memory to write to to communicate between the `CmdExecution`s and
/// `leptos`.
#[derive(Clone, Debug, Default)]
pub struct CmdExecToLeptosCtx {
    /// The example outcome `InfoGraph` for all `CmdExecution`s.
    ///
    /// Shared memory for `Map<CmdExecutionId, InfoGraph>`.
    pub flow_outcome_example_info_graphs: FlowInfoGraphs<FlowId>,
    /// The actual outcome `InfoGraph` for all `CmdExecution`s.
    ///
    /// Shared memory for `Map<CmdExecutionId, InfoGraph>`.
    pub flow_outcome_actual_info_graphs: FlowInfoGraphs<CmdExecutionId>,
    /// The interrupt channel sender for each `CmdExecution`.
    pub cmd_exec_interrupt_txs: HashMap<CmdExecutionId, mpsc::Sender<InterruptSignal>>,
}

impl CmdExecToLeptosCtx {
    /// Returns a new `CmdExecToLeptosCtx`.
    pub fn new(
        flow_outcome_example_info_graphs: FlowInfoGraphs<FlowId>,
        flow_outcome_actual_info_graphs: FlowInfoGraphs<CmdExecutionId>,
        cmd_exec_interrupt_txs: HashMap<CmdExecutionId, mpsc::Sender<InterruptSignal>>,
    ) -> Self {
        Self {
            flow_outcome_example_info_graphs,
            flow_outcome_actual_info_graphs,
            cmd_exec_interrupt_txs,
        }
    }
}
