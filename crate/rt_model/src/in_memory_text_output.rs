use peace_fmt::Presentable;
use peace_rt_model_core::{async_trait, output::OutputWrite};

use crate::Error;

cfg_if::cfg_if! {
    if #[cfg(feature = "output_progress")] {
        use peace_item_interaction_model::ItemLocationState;
        use peace_item_model::ItemId;
        use peace_progress_model::{
            CmdBlockItemInteractionType,
            ProgressTracker,
            ProgressUpdateAndId,
        };

        use crate::CmdProgressTracker;
    }
}

/// An `OutputWrite` implementation that writes to the command line.
///
/// Currently this only outputs return values or errors, not progress.
#[derive(Debug, Default)]
pub struct InMemoryTextOutput {
    /// Buffer to write to.
    buffer: String,
    /// The `miette::ReportHandler` to format errors nicely.
    #[cfg(feature = "error_reporting")]
    pub(crate) report_handler: miette::GraphicalReportHandler,
}

impl InMemoryTextOutput {
    /// Returns a new `InMemoryTextOutput`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the inner buffer.
    pub fn into_inner(self) -> String {
        self.buffer
    }
}

/// Simple serialization implementations for now.
///
/// See <https://github.com/azriel91/peace/issues/28> for further improvements.
#[async_trait(?Send)]
impl OutputWrite for InMemoryTextOutput {
    type Error = Error;

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
    async fn cmd_block_start(
        &mut self,
        _cmd_block_item_interaction_type: CmdBlockItemInteractionType,
    ) {
    }

    #[cfg(feature = "output_progress")]
    async fn item_location_state(
        &mut self,
        _item_id: ItemId,
        _item_location_state: ItemLocationState,
    ) {
    }

    #[cfg(feature = "output_progress")]
    async fn progress_end(&mut self, _cmd_progress_tracker: &CmdProgressTracker) {}

    async fn present<P>(&mut self, presentable: P) -> Result<(), Error>
    where
        P: Presentable,
    {
        self.buffer
            .push_str(&serde_yaml::to_string(&presentable).map_err(Error::StatesSerialize)?);

        Ok(())
    }

    #[cfg(not(feature = "error_reporting"))]
    async fn write_err<E>(&mut self, error: &E) -> Result<(), Error>
    where
        E: std::error::Error,
    {
        self.buffer = format!("{error}\n");

        Ok(())
    }

    #[cfg(feature = "error_reporting")]
    async fn write_err<E>(&mut self, error: &E) -> Result<(), Error>
    where
        E: miette::Diagnostic,
    {
        use miette::Diagnostic;

        let mut err_buffer = String::new();

        let mut diagnostic_opt: Option<&dyn Diagnostic> = Some(error);
        while let Some(diagnostic) = diagnostic_opt {
            if diagnostic.help().is_some()
                || diagnostic.labels().is_some()
                || diagnostic.diagnostic_source().is_none()
            {
                // Ignore failures when writing errors
                let (Ok(()) | Err(_)) = self
                    .report_handler
                    .render_report(&mut err_buffer, diagnostic);
                err_buffer.push('\n');

                let err_buffer = err_buffer.lines().fold(
                    String::with_capacity(err_buffer.len()),
                    |mut buffer, line| {
                        if line.trim().is_empty() {
                            buffer.push('\n');
                        } else {
                            buffer.push_str(line);
                            buffer.push('\n');
                        }
                        buffer
                    },
                );

                self.buffer.push_str(&err_buffer);
            }

            diagnostic_opt = diagnostic.diagnostic_source();
            err_buffer.clear();
        }

        Ok(())
    }
}
