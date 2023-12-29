use futures::FutureExt;
use peace::{
    cmd::scopes::{SingleProfileSingleFlowView, SingleProfileSingleFlowViewAndOutput},
    fmt::presentable::{Heading, HeadingLevel, ListNumbered},
    resources::resources::ts::SetUp,
    rt::cmds::{ApplyStoredStateSync, EnsureCmd},
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

/// Deploys or updates the environment.
#[derive(Debug)]
pub struct EnvDeployCmd;

impl EnvDeployCmd {
    /// Deploys or updates the environment.
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
        };

        Ok(())
    }
}

macro_rules! run {
    ($output:ident, $flow_cmd:ident, $padding:expr, $debug:expr) => {{
        $flow_cmd::run(
            $output,
            CmdOpts::default().with_profile_print(false),
            |cmd_ctx| run_with_ctx(cmd_ctx, $padding, $debug).boxed_local(),
        )
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
    let states_ensured_outcome =
        EnsureCmd::exec_with(cmd_ctx, ApplyStoredStateSync::Current).await?;
    let SingleProfileSingleFlowViewAndOutput {
        output,
        cmd_view: SingleProfileSingleFlowView {
            flow, resources, ..
        },
        ..
    } = cmd_ctx.view_and_output();

    if let Some(states_ensured) = states_ensured_outcome.value() {
        let states_ensured_raw_map = &***states_ensured;

        let states_ensured_presentables = {
            let states_ensured_presentables = flow
                .graph()
                .iter_insertion()
                .map(|item| {
                    let item_id = item.id();
                    // Hack: for alignment
                    let padding =
                        " ".repeat(padding_count.saturating_sub(format!("{item_id}").len() + 2));
                    match states_ensured_raw_map.get(item_id) {
                        Some(state_ensured) => (item_id, format!("{padding}: {state_ensured}")),
                        None => (item_id, format!("{padding}: <unknown>")),
                    }
                })
                .collect::<Vec<_>>();

            ListNumbered::new(states_ensured_presentables)
        };

        output
            .present(&(
                Heading::new(HeadingLevel::Level1, "States Ensured"),
                states_ensured_presentables,
                "\n",
            ))
            .await?;
    }

    if debug {
        crate::output::cmd_outcome_completion_present(output, resources, states_ensured_outcome)
            .await?;
    }

    Ok(())
}

use run;
