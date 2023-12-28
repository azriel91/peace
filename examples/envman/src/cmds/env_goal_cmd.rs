use futures::FutureExt;
use peace::{
    cmd::scopes::{SingleProfileSingleFlowView, SingleProfileSingleFlowViewAndOutput},
    fmt::presentable::{Heading, HeadingLevel, ListNumbered},
    resources::resources::ts::SetUp,
    rt::cmds::StatesGoalReadCmd,
    rt_model::output::OutputWrite,
};

use crate::{
    cmds::{
        common::{env_man_flow, workspace},
        AppUploadCmd, EnvCmd,
    },
    model::{EnvManError, EnvManFlow},
    rt_model::EnvManCmdCtx,
};

/// Shows the goal state of the environment.
#[derive(Debug)]
pub struct EnvGoalCmd;

impl EnvGoalCmd {
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
        $flow_cmd::run($output, true, |cmd_ctx| {
            run_with_ctx(cmd_ctx, $padding).boxed_local()
        })
        .await?;
    }};
}

async fn run_with_ctx<O>(
    cmd_ctx: &mut EnvManCmdCtx<'_, O, SetUp>,
    padding_count: usize,
) -> Result<(), EnvManError>
where
    O: OutputWrite<EnvManError>,
{
    let states_goal_outcome = StatesGoalReadCmd::exec(cmd_ctx).await?;
    if let Some(states_goal) = states_goal_outcome.value() {
        let states_goal_raw_map = &***states_goal;

        let SingleProfileSingleFlowViewAndOutput {
            output,
            cmd_view: SingleProfileSingleFlowView { flow, .. },
            ..
        } = cmd_ctx.view_and_output();
        let states_goal_presentables = {
            let states_goal_presentables = flow
                .graph()
                .iter_insertion()
                .map(|item| {
                    let item_id = item.id();
                    // Hack: for alignment
                    let padding =
                        " ".repeat(padding_count.saturating_sub(format!("{item_id}").len() + 2));
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
                Heading::new(HeadingLevel::Level1, "Goal States"),
                states_goal_presentables,
                "\n",
            ))
            .await?;
    }

    Ok(())
}

use run;
