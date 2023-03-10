use futures::FutureExt;
use peace::{rt::cmds::sub::StatesDesiredDiscoverCmd, rt_model::output::OutputWrite};

use crate::{cmds::EnvCmd, model::AppCycleError};

/// Shows the desired state of the environment.
#[derive(Debug)]
pub struct EnvDesiredCmd;

impl EnvDesiredCmd {
    /// Shows the desired state of the environment.
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
        EnvCmd::run(output, |ctx| {
            StatesDesiredDiscoverCmd::exec(ctx).boxed_local()
        })
        .await
    }
}
