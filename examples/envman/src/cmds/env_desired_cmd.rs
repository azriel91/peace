use futures::FutureExt;
use peace::{
    cmd::scopes::SingleProfileSingleFlowView,
    fmt::presentable::{Heading, HeadingLevel, ListNumbered},
    rt::cmds::sub::StatesDesiredReadCmd,
    rt_model::output::OutputWrite,
};

use crate::{cmds::EnvCmd, model::EnvManError};

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
    pub async fn run<O>(output: &mut O) -> Result<(), EnvManError>
    where
        O: OutputWrite<EnvManError> + Send,
    {
        EnvCmd::run(output, true, |ctx| {
            async {
                let states_desired = StatesDesiredReadCmd::exec(ctx).await?;
                let states_desired_raw_map = &**states_desired;

                let SingleProfileSingleFlowView { output, flow, .. } = ctx.view();
                let states_desired_presentables = {
                    let states_desired_presentables = flow
                        .graph()
                        .iter_insertion()
                        .map(|item_spec| {
                            let item_spec_id = item_spec.id();
                            match states_desired_raw_map.get(item_spec_id) {
                                Some(state_desired) => (item_spec_id, format!(": {state_desired}")),
                                None => (item_spec_id, String::from(": <unknown>")),
                            }
                        })
                        .collect::<Vec<_>>();

                    ListNumbered::new(states_desired_presentables)
                };

                output
                    .present(&(
                        Heading::new(HeadingLevel::Level1, "States Desired"),
                        states_desired_presentables,
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
