use std::fmt::Debug;

use async_trait::async_trait;
use peace_cmd::scopes::SingleProfileSingleFlowView;
use peace_resources::{resources::ts::SetUp, Resource};
use peace_rt_model::{outcomes::CmdOutcome, params::ParamsKeys};

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

    /// Executes this command block.
    async fn exec(
        &self,
        view: &mut SingleProfileSingleFlowView<'_, Self::Error, Self::PKeys, SetUp>,
        #[cfg(feature = "output_progress")] progress_tx: Sender<ProgressUpdateAndId>,
        input: Box<dyn Resource>,
    ) -> Result<CmdOutcome<Box<dyn Resource>, Self::Error>, Self::Error>;

    /// Runs the error handler and maps the `CmdBlock`'s `Outcome` to
    /// `CmdExecution::Outcome`.
    ///
    /// This allows a `Cmd` to run logic to map an intermediate `CmdBlock`s
    /// outcome which contains item failures, to the `CmdExecution` outcome
    /// type.
    fn execution_outcome_from(&self, outcome_acc: Box<dyn Resource>) -> Box<dyn Resource>;
}
