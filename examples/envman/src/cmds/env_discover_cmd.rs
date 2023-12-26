use futures::FutureExt;
use peace::{
    cmd::scopes::{SingleProfileSingleFlowView, SingleProfileSingleFlowViewAndOutput},
    fmt::presentable::{Heading, HeadingLevel, ListNumbered},
    rt::cmds::StatesDiscoverCmd,
    rt_model::output::OutputWrite,
};
use peace_cmd_model::CmdOutcome;

use crate::{
    cmds::{
        common::{env_man_flow, workspace},
        AppUploadCmd, EnvCmd,
    },
    model::{EnvManError, EnvManFlow},
};

/// Shows the goal state of the environment.
#[derive(Debug)]
pub struct EnvDiscoverCmd;

impl EnvDiscoverCmd {
    /// Shows the goal state of the environment.
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
                let CmdOutcome {
                    value: (states_current, states_goal),
                    errors: _,
                } = StatesDiscoverCmd::current_and_goal(ctx).await?;
                let states_current_raw_map = &**states_current;
                let states_goal_raw_map = &**states_goal;

                let SingleProfileSingleFlowViewAndOutput {
                    output,
                    cmd_view: SingleProfileSingleFlowView { flow, .. },
                    ..
                } = ctx.view_and_output();
                let states_current_presentables = {
                    let states_current_presentables = flow
                        .graph()
                        .iter_insertion()
                        .map(|item| {
                            let item_id = item.id();
                            // Hack: for alignment
                            let padding =
                                " ".repeat($padding.saturating_sub(format!("{item_id}").len() + 2));
                            match states_current_raw_map.get(item_id) {
                                Some(state_current) => {
                                    (item_id, format!("{padding}: {state_current}"))
                                }
                                None => (item_id, format!("{padding}: <unknown>")),
                            }
                        })
                        .collect::<Vec<_>>();

                    ListNumbered::new(states_current_presentables)
                };
                let states_goal_presentables = {
                    let states_goal_presentables = flow
                        .graph()
                        .iter_insertion()
                        .map(|item| {
                            let item_id = item.id();
                            // Hack: for alignment
                            let padding =
                                " ".repeat($padding.saturating_sub(format!("{item_id}").len() + 2));
                            match states_goal_raw_map.get(item_id) {
                                Some(state_goal) => (item_id, format!("{padding}: {state_goal}")),
                                None => (item_id, format!("{padding}: <unknown>")),
                            }
                        })
                        .collect::<Vec<_>>();

                    ListNumbered::new(states_goal_presentables)
                };

                output
                    .present(&(
                        Heading::new(HeadingLevel::Level1, "Current States"),
                        states_current_presentables,
                        "\n",
                        Heading::new(HeadingLevel::Level1, "Goal States"),
                        states_goal_presentables,
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
