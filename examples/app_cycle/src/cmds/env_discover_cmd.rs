use futures::FutureExt;
use peace::{
    fmt::presentable::{Heading, HeadingLevel},
    rt::cmds::StatesDiscoverCmd,
    rt_model::output::OutputWrite,
};

use crate::{cmds::EnvCmd, model::AppCycleError};

/// Shows the desired state of the environment.
#[derive(Debug)]
pub struct EnvDiscoverCmd;

impl EnvDiscoverCmd {
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
        let (states_current, states_desired) =
            EnvCmd::run(output, |ctx| StatesDiscoverCmd::exec(ctx).boxed_local()).await?;

        output
            .present(&(
                Heading::new(HeadingLevel::Level1, "States Current"),
                &states_current,
                "\n",
                Heading::new(HeadingLevel::Level1, "States Desired"),
                &states_desired,
                "\n",
            ))
            .await?;
        Ok(())
    }
}
