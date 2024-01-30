use futures::FutureExt;
use peace::{
    cmd::scopes::{SingleProfileSingleFlowView, SingleProfileSingleFlowViewAndOutput},
    cmd_model::CmdOutcome,
    fmt::{
        presentable::{Heading, HeadingLevel, ListNumberedAligned},
        PresentableExt,
    },
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
            EnvManFlow::AppUpload => run!(output, AppUploadCmd, debug),
            EnvManFlow::EnvDeploy => run!(output, EnvCmd, debug),
        }

        Ok(())
    }
}

macro_rules! run {
    ($output:ident, $flow_cmd:ident, $debug:expr) => {{
        $flow_cmd::run($output, CmdOpts::default(), |cmd_ctx| {
            run_with_ctx(cmd_ctx, $debug).boxed_local()
        })
        .await?;
    }};
}

async fn run_with_ctx<O>(
    cmd_ctx: &mut EnvManCmdCtx<'_, O, SetUp>,

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

        let states_current_presentables: ListNumberedAligned<_, _> = flow
            .graph()
            .iter_insertion()
            .map(|item| {
                let item_id = item.id();

                let state_current_presentable = match states_current_raw_map.get(item_id) {
                    Some(state_current) => format!("{state_current}").left_presentable(),
                    None => "<unknown>".right_presentable(),
                };

                (item_id, state_current_presentable)
            })
            .collect::<Vec<_>>()
            .into();
        let states_goal_presentables: ListNumberedAligned<_, _> = flow
            .graph()
            .iter_insertion()
            .map(|item| {
                let item_id = item.id();

                let state_goal_presentable = match states_goal_raw_map.get(item_id) {
                    Some(state_goal) => format!("{state_goal}").left_presentable(),
                    None => "<unknown>".right_presentable(),
                };

                (item_id, state_goal_presentable)
            })
            .collect::<Vec<_>>()
            .into();

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
    if let CmdOutcome::ItemError { errors, .. } = &states_discover_outcome {
        crate::output::item_errors_present(output, errors).await?;
    }

    if debug {
        crate::output::cmd_outcome_completion_present(output, resources, states_discover_outcome)
            .await?;
    }

    Ok(())
}

use run;
