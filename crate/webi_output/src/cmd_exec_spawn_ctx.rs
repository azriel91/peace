use std::fmt;

use futures::future::LocalBoxFuture;
use interruptible::InterruptSignal;
use tokio::sync::mpsc;

/// The `CmdExecution` task, as well as the channels to interact with it.
///
/// This is returned by the `CmdExecution` spawning function for each `Flow`,
/// which is registered in `WebiOutput`.
pub struct CmdExecSpawnCtx {
    /// Channel sender to send an `InterruptSignal`.
    pub interrupt_tx: Option<mpsc::Sender<InterruptSignal>>,
    /// The `*Cmd::run(..)` task.
    ///
    /// This will be submitted to the tokio task pool.
    pub cmd_exec_task: LocalBoxFuture<'static, ()>,
}

impl fmt::Debug for CmdExecSpawnCtx {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CmdExecSpawnCtx")
            .field("interrupt_tx", &self.interrupt_tx)
            .field("cmd_exec_task", &stringify!(LocalBoxFuture<'static, ()>))
            .finish()
    }
}
