use std::fmt::Debug;

use peace_fmt::Presentable;
use peace_rt_model_core::{async_trait, output::OutputWrite};

cfg_if::cfg_if! {
    if #[cfg(feature = "output_progress")] {
        use peace_core::progress::{
            ProgressComplete,
            ProgressLimit,
            ProgressStatus,
            ProgressTracker,
            ProgressUpdate,
            ProgressUpdateAndId,
        };
        use peace_rt_model_core::CmdProgressTracker;
    }
}

/// An `OutputWrite` implementation that writes to web elements.
#[derive(Debug)]
pub struct WebiOutput {}

#[async_trait(?Send)]
impl<E> OutputWrite<E> for WebiOutput {
    #[cfg(feature = "output_progress")]
    async fn progress_begin(&mut self, cmd_progress_tracker: &CmdProgressTracker) {}

    #[cfg(feature = "output_progress")]
    async fn progress_update(
        &mut self,
        progress_tracker: &ProgressTracker,
        progress_update_and_id: &ProgressUpdateAndId,
    ) {
    }

    #[cfg(feature = "output_progress")]
    async fn progress_end(&mut self, cmd_progress_tracker: &CmdProgressTracker) {}

    async fn present<P>(&mut self, _presentable: P) -> Result<(), E>
    where
        E: std::error::Error,
        P: Presentable,
    {
        todo!()
    }

    async fn write_err(&mut self, _error: &E) -> Result<(), E>
    where
        E: std::error::Error,
    {
        todo!()
    }
}
