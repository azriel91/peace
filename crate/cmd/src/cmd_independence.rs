//! Information about how command logic should be run.
//!
//! This is information used when implementing commands, not necessarily
//! information that developers who *use* commands need to know.
//!
//! If we need to pass multiple kinds of information of how command logic should
//! be run, we should group them together as a `CmdCtxInternal` or similar.

use peace_resources::resources::ts::SetUp;
use peace_rt_model::params::ParamsKeys;

use crate::{
    ctx::CmdCtx,
    scopes::{SingleProfileSingleFlow, SingleProfileSingleFlowView},
};

cfg_if::cfg_if! {
    if #[cfg(feature = "output_progress")] {
        use peace_cfg::progress::ProgressUpdateAndId;
        use tokio::sync::mpsc::Sender;
    }
}

/// Whether a command is an independent top level command, or as part of another
/// command.
///
/// # Design
///
/// The three lifetimes are present because of the complexity of the
/// [`CmdBase::exec`] function. See the documentation on that function for the
/// technical necessity and rationale for choosing separate lifetimes here.
///
/// [`CmdBase::exec`]: crate::cmds::CmdBase::exec
#[derive(Debug)]
pub enum CmdIndependence<'ctx, 'scope, 'view, E, O, PKeys>
where
    E: 'static,
    PKeys: ParamsKeys + 'static,
{
    /// This command is being executed as a top level command.
    Standalone {
        cmd_ctx: &'ctx mut CmdCtx<SingleProfileSingleFlow<'scope, E, O, PKeys, SetUp>>,
    },
    /// This command is being executed as part of another command.
    ///
    /// This takes only the view; sub commands will not have access to the
    /// `output`, which is held by the top level standalone command.
    SubCmd {
        /// Flow and parameters for executing the command.
        cmd_view: &'ctx mut SingleProfileSingleFlowView<'view, E, PKeys, SetUp>,
    },
    /// This command is being executed as part of another command.
    #[cfg(feature = "output_progress")]
    SubCmdWithProgress {
        /// Flow and parameters for executing the command.
        cmd_view: &'ctx mut SingleProfileSingleFlowView<'view, E, PKeys, SetUp>,
        /// Sender to use for progress updates.
        progress_tx: Sender<ProgressUpdateAndId>,
    },
}
