use futures::FutureExt;
use peace::{
    rt::cmds::{sub::StatesSavedReadCmd, CleanCmd},
    rt_model::output::OutputWrite,
};

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
        let states_saved =
            EnvCmd::run(output, |ctx| StatesSavedReadCmd::exec(ctx).boxed_local()).await?;

        // https://github.com/rust-lang/rust-clippy/issues/10482
        #[allow(clippy::redundant_async_block)]
        EnvCmd::run_and_present(output, |ctx| {
            async move { CleanCmd::exec(ctx, &states_saved).await }.boxed_local()
        })
        .await
    }
}
