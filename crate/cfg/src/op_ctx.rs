use peace_core::ProgressUpdate;
use tokio::sync::mpsc::Sender;

/// References to pass information into the Peace framework.
#[derive(Debug)]
pub struct OpCtx<'op> {
    /// For item spec implementations to send progress to.
    pub progress_tx: &'op Sender<ProgressUpdate>,
}
