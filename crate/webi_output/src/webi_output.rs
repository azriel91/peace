use std::{fmt::Debug, net::SocketAddr};

use peace_fmt::Presentable;
use peace_rt_model::Flow;
use peace_rt_model_core::{async_trait, output::OutputWrite};
use peace_value_traits::AppError;
use peace_webi_model::WebiError;

use crate::WebiServer;

cfg_if::cfg_if! {
    if #[cfg(feature = "output_progress")] {
        use peace_core::progress::{
            // ProgressComplete,
            // ProgressLimit,
            // ProgressStatus,
            ProgressTracker,
            // ProgressUpdate,
            ProgressUpdateAndId,
        };
        use peace_rt_model_core::CmdProgressTracker;
    }
}

/// An `OutputWrite` implementation that writes to web elements.
#[derive(Clone, Debug)]
pub struct WebiOutput<E> {
    /// IP address and port to listen on.
    socket_addr: Option<SocketAddr>,
    /// Flow to work with.
    ///
    /// # Design
    ///
    /// Currently we only take in one flow, but in the future we want to take in
    /// multiple `Flow`s (or functions so we can lazily instantiate them).
    flow: Flow<E>,
}

impl<E> WebiOutput<E>
where
    E: Clone + Debug + 'static,
{
    pub fn new(socket_addr: Option<SocketAddr>, flow: Flow<E>) -> Self {
        Self { socket_addr, flow }
    }

    pub async fn start(&self) -> Result<(), WebiError> {
        let Self { socket_addr, flow } = self.clone();

        WebiServer::new(socket_addr, flow).start().await
    }
}

#[async_trait(?Send)]
impl<AppErrorT, E> OutputWrite<AppErrorT> for WebiOutput<E>
where
    AppErrorT: AppError,
    E: std::fmt::Debug,
{
    #[cfg(feature = "output_progress")]
    async fn progress_begin(&mut self, _cmd_progress_tracker: &CmdProgressTracker) {}

    #[cfg(feature = "output_progress")]
    async fn progress_update(
        &mut self,
        _progress_tracker: &ProgressTracker,
        _progress_update_and_id: &ProgressUpdateAndId,
    ) {
    }

    #[cfg(feature = "output_progress")]
    async fn progress_end(&mut self, _cmd_progress_tracker: &CmdProgressTracker) {}

    async fn present<P>(&mut self, _presentable: P) -> Result<(), AppErrorT>
    where
        AppErrorT: std::error::Error,
        P: Presentable,
    {
        todo!()
    }

    async fn write_err(&mut self, _error: &AppErrorT) -> Result<(), AppErrorT>
    where
        AppErrorT: std::error::Error,
    {
        todo!()
    }
}
