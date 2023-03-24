use futures::FutureExt;
use peace::{
    cmd::scopes::SingleProfileSingleFlowView,
    fmt::presentable::{Heading, HeadingLevel, ListNumbered},
    rt::cmds::sub::StatesSavedReadCmd,
    rt_model::output::OutputWrite,
};

use crate::{cmds::EnvCmd, model::AppCycleError};

/// Shows the saved state of the environment.
#[derive(Debug)]
pub struct EnvStatusCmd;

impl EnvStatusCmd {
    /// Shows the saved state of the environment.
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
        EnvCmd::run(output, true, |ctx| {
            async {
                let states_saved = StatesSavedReadCmd::exec(ctx).await?;
                let states_saved_raw_map = &**states_saved;

                let SingleProfileSingleFlowView { output, flow, .. } = ctx.view();
                let states_saved_presentables = {
                    let states_saved_presentables = flow
                        .graph()
                        .iter_insertion()
                        .map(|item_spec| {
                            let item_spec_id = item_spec.id();
                            match states_saved_raw_map.get(item_spec_id) {
                                Some(state_saved) => (item_spec_id, format!(": {state_saved}")),
                                None => (item_spec_id, String::from(": <unknown>")),
                            }
                        })
                        .collect::<Vec<_>>();

                    ListNumbered::new(states_saved_presentables)
                };

                output
                    .present(&(
                        Heading::new(HeadingLevel::Level1, "States Saved"),
                        states_saved_presentables,
                        "\n",
                    ))
                    .await?;

                Ok(())
            }
            .boxed_local()
        })
        .await?;

        Ok(())
    }
}
