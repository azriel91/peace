use futures::FutureExt;
use peace::{rt::cmds::DiffCmd, rt_model::output::OutputWrite};

use crate::{cmds::EnvCmd, model::AppCycleError};

/// Shows the diff between current and desired states of the environment.
#[derive(Debug)]
pub struct EnvDiffCmd;

impl EnvDiffCmd {
    /// Shows the diff between current and desired states of the environment.
    ///
    /// # Parameters
    ///
    /// * `output`: Output to write the execution outcome.
    /// * `slug`: Username and repository of the application to download.
    /// * `version`: Version of the application to download.
    /// * `url`: URL to override where to download the application from.
    pub async fn run<O>(output: &mut O) -> Result<(), AppCycleError>
    where
        O: OutputWrite<AppCycleError> + Send,
    {
        EnvCmd::run_and_present(output, |ctx| DiffCmd::exec(ctx).boxed_local()).await
    }
}
