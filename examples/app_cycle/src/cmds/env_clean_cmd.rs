use futures::FutureExt;
use peace::{rt::cmds::CleanCmd, rt_model::output::OutputWrite};

use crate::{cmds::EnvCmd, model::AppCycleError};

/// Cleans up (deletes) the environment.
#[derive(Debug)]
pub struct EnvCleanCmd;

impl EnvCleanCmd {
    /// Cleans up (deletes) the environment.
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
        EnvCmd::run_and_present(output, |ctx| CleanCmd::exec(ctx).boxed_local()).await
    }
}
