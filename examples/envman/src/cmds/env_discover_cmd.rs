use futures::FutureExt;
use peace::{
    cmd::scopes::{SingleProfileSingleFlowView, SingleProfileSingleFlowViewAndOutput},
    fmt::presentable::{Heading, HeadingLevel, ListNumbered},
    resources::resources::ts::SetUp,
    rt::cmds::StatesDiscoverCmd,
    rt_model::output::OutputWrite,
};

use crate::{
    cmds::{
        common::{env_man_flow, workspace},
        AppUploadCmd, CmdOpts, EnvCmd,
    },
    model::{EnvManError, EnvManFlow},
    rt_model::EnvManCmdCtx,
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
    /// * `debug`: Whether to print `CmdOutcome` debug info.
    pub async fn run<O>(output: &mut O, debug: bool) -> Result<(), EnvManError>
    where
        O: OutputWrite<EnvManError> + Send,
    {
        let workspace = workspace()?;
        let env_man_flow = env_man_flow(output, &workspace).await?;
        match env_man_flow {
            EnvManFlow::AppUpload => run!(output, AppUploadCmd, 14usize, debug),
            EnvManFlow::EnvDeploy => run!(output, EnvCmd, 18usize, debug),
        }

        Ok(())
    }
}

macro_rules! run {
    ($output:ident, $flow_cmd:ident, $padding:expr, $debug:expr) => {{
        $flow_cmd::run($output, CmdOpts::default(), |cmd_ctx| {
            run_with_ctx(cmd_ctx, $padding, $debug).boxed_local()
        })
        .await?;
    }};
}

async fn run_with_ctx<O>(
    cmd_ctx: &mut EnvManCmdCtx<'_, O, SetUp>,
    padding_count: usize,
    debug: bool,
) -> Result<(), EnvManError>
where
    O: OutputWrite<EnvManError>,
{
    let states_discover_outcome = StatesDiscoverCmd::current_and_goal(cmd_ctx).await?;

    let SingleProfileSingleFlowViewAndOutput {
        output,
        cmd_view: SingleProfileSingleFlowView {
            flow, resources, ..
        },
        ..
    } = cmd_ctx.view_and_output();

    if let Some((states_current, states_goal)) = states_discover_outcome.value() {
        let states_current_raw_map = &***states_current;
        let states_goal_raw_map = &***states_goal;

        let states_current_presentables = {
            let states_current_presentables = flow
                .graph()
                .iter_insertion()
                .map(|item| {
                    let item_id = item.id();
                    // Hack: for alignment
                    let padding =
                        " ".repeat(padding_count.saturating_sub(format!("{item_id}").len() + 2));
                    match states_current_raw_map.get(item_id) {
                        Some(state_current) => (item_id, format!("{padding}: {state_current}")),
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
                Heading::new(HeadingLevel::Level1, "Current States"),
                states_current_presentables,
                "\n",
                Heading::new(HeadingLevel::Level1, "Goal States"),
                states_goal_presentables,
                "\n",
            ))
            .await?;
    }

    if debug {
        crate::output::cmd_outcome_completion_present(output, resources, states_discover_outcome)
            .await?;
    }

    Ok(())
}

use run;
