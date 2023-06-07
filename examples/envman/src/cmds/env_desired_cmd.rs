use futures::FutureExt;
use peace::{
    cmd::scopes::{SingleProfileSingleFlowView, SingleProfileSingleFlowViewAndOutput},
    fmt::presentable::{Heading, HeadingLevel, ListNumbered},
    rt::cmds::StatesDesiredReadCmd,
    rt_model::output::OutputWrite,
};

use crate::{
    cmds::{
        common::{env_man_flow, workspace},
        AppUploadCmd, EnvCmd,
    },
    model::{EnvManError, EnvManFlow},
};

/// Shows the desired state of the environment.
#[derive(Debug)]
pub struct EnvDesiredCmd;

impl EnvDesiredCmd {
    /// Shows the desired state of the environment.
    ///
    /// # Parameters
    ///
    /// * `output`: Output to write the execution outcome.
    pub async fn run<O>(output: &mut O) -> Result<(), EnvManError>
    where
        O: OutputWrite<EnvManError> + Send,
    {
        let workspace = workspace()?;
        let env_man_flow = env_man_flow(output, &workspace).await?;
        match env_man_flow {
            EnvManFlow::AppUpload => run!(output, AppUploadCmd, 14usize),
            EnvManFlow::EnvDeploy => run!(output, EnvCmd, 18usize),
        }

        Ok(())
    }
}

macro_rules! run {
    ($output:ident, $flow_cmd:ident, $padding:expr) => {{
        $flow_cmd::run($output, true, |ctx| {
            async {
                let states_desired = StatesDesiredReadCmd::exec(ctx).await?;
                let states_desired_raw_map = &**states_desired;

                let SingleProfileSingleFlowViewAndOutput {
                    output,
                    cmd_view: SingleProfileSingleFlowView { flow, .. },
                    ..
                } = ctx.view_and_output();
                let states_desired_presentables = {
                    let states_desired_presentables = flow
                        .graph()
                        .iter_insertion()
                        .map(|item| {
                            let item_id = item.id();
                            // Hack: for alignment
                            let padding =
                                " ".repeat($padding.saturating_sub(format!("{item_id}").len() + 2));
                            match states_desired_raw_map.get(item_id) {
                                Some(state_desired) => {
                                    (item_id, format!("{padding}: {state_desired}"))
                                }
                                None => (item_id, format!("{padding}: <unknown>")),
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
    }};
}

use run;
