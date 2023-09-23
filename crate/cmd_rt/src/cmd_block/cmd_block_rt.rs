use std::fmt::Debug;

use async_trait::async_trait;
use peace_cmd::scopes::SingleProfileSingleFlowView;
use peace_cmd_model::CmdBlockDesc;
use peace_resources::resources::ts::SetUp;
use peace_rt_model::params::ParamsKeys;

use crate::CmdBlockError;

cfg_if::cfg_if! {
    if #[cfg(feature = "output_progress")] {
        use peace_cfg::progress::ProgressUpdateAndId;
        use tokio::sync::mpsc::Sender;
    }
}

/// Type erased [`CmdBlock`]
///
/// [`CmdBlock`]: crate::CmdBlock
#[async_trait(?Send)]
pub trait CmdBlockRt: Debug + Unpin {
    /// Automation software error type.
    type Error: std::error::Error + From<peace_rt_model::Error> + Send + 'static;
    /// Types used for params keys.
    type PKeys: ParamsKeys + 'static;
    /// Outcome type of the command execution.
    type ExecutionOutcome: Debug + 'static;

    /// Executes this command block.
    async fn exec(
        &self,
        view: &mut SingleProfileSingleFlowView<'_, Self::Error, Self::PKeys, SetUp>,
        #[cfg(feature = "output_progress")] progress_tx: Sender<ProgressUpdateAndId>,
    ) -> Result<(), CmdBlockError<Self::ExecutionOutcome, Self::Error>>;

    /// Returns the `String` representation of the `CmdBlock` in a
    /// `CmdExecution`.
    ///
    /// This is used to provide a well-formatted error message so that
    /// developers can identify where a bug lies more easily.
    fn cmd_block_desc(&self) -> CmdBlockDesc;
}
