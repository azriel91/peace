use std::{fmt::Debug, net::SocketAddr};

use peace_flow_model::FlowSpecInfo;
use peace_fmt::Presentable;
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
pub struct WebiOutput {
    /// IP address and port to listen on.
    socket_addr: Option<SocketAddr>,
    /// Flow to display to the user.
    flow_spec_info: FlowSpecInfo,
}

impl WebiOutput {
    pub fn new(socket_addr: Option<SocketAddr>, flow_spec_info: FlowSpecInfo) -> Self {
        Self {
            socket_addr,
            flow_spec_info,
        }
    }
}

impl WebiOutput {
    pub async fn start(&self) -> Result<(), WebiError> {
        let Self {
            socket_addr,
            flow_spec_info,
        } = self.clone();

        WebiServer::new(socket_addr, flow_spec_info).start().await
    }
}

#[async_trait(?Send)]
impl<AppErrorT> OutputWrite<AppErrorT> for WebiOutput
where
    AppErrorT: AppError,
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
