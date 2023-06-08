use futures::FutureExt;
use peace::{
    cfg::Profile,
    cmd::scopes::{
        MultiProfileSingleFlowView, SingleProfileSingleFlowView,
        SingleProfileSingleFlowViewAndOutput,
    },
    fmt::presentable::{Heading, HeadingLevel, ListNumbered},
    resources::states::StateDiffs,
    rt::cmds::DiffCmd,
    rt_model::{output::OutputWrite, Flow},
};

use crate::{
    cmds::{
        common::{env_man_flow, workspace},
        AppUploadCmd, EnvCmd,
    },
    model::{EnvDiffSelection, EnvManError, EnvManFlow},
};

/// Shows the diff between current and goal states of the environment.
#[derive(Debug)]
pub struct EnvDiffCmd;

impl EnvDiffCmd {
    /// Shows the diff between current and goal states of the environment.
    ///
    /// # Parameters
    ///
    /// * `output`: Output to write the execution outcome.
    /// * `env_diff_selection`: Profiles to compare.
    pub async fn run<O>(
        output: &mut O,
        env_diff_selection: EnvDiffSelection,
    ) -> Result<(), EnvManError>
    where
        O: OutputWrite<EnvManError> + Send,
    {
        let workspace = workspace()?;
        let env_man_flow = env_man_flow(output, &workspace).await?;
        match env_diff_selection {
            EnvDiffSelection::CurrentAndGoal => {
                Self::active_profile_current_vs_goal(output, env_man_flow).await
            }
            EnvDiffSelection::DiffProfilesCurrent {
                profile_a,
                profile_b,
            } => Self::diff_profiles_current(output, env_man_flow, profile_a, profile_b).await,
        }
    }

    async fn active_profile_current_vs_goal<O>(
        output: &mut O,
        env_man_flow: EnvManFlow,
    ) -> Result<(), EnvManError>
    where
        O: OutputWrite<EnvManError> + Send,
    {
        match env_man_flow {
            EnvManFlow::AppUpload => run!(output, AppUploadCmd, 14usize),
            EnvManFlow::EnvDeploy => run!(output, EnvCmd, 18usize),
        }
    }

    async fn diff_profiles_current<O>(
        output: &mut O,
        env_man_flow: EnvManFlow,
        profile_a: Profile,
        profile_b: Profile,
    ) -> Result<(), EnvManError>
    where
        O: OutputWrite<EnvManError> + Send,
    {
        match env_man_flow {
            EnvManFlow::AppUpload => {
                run_multi!(output, AppUploadCmd, 14usize, profile_a, profile_b)
            }
            EnvManFlow::EnvDeploy => run_multi!(output, EnvCmd, 18usize, profile_a, profile_b),
        };

        Ok(())
    }

    async fn state_diffs_present<O>(
        output: &mut O,
        flow: &Flow<EnvManError>,
        state_diffs: &StateDiffs,
        padding: usize,
    ) -> Result<(), EnvManError>
    where
        O: OutputWrite<EnvManError> + Send,
    {
        let state_diffs_raw_map = &***state_diffs;

        let state_diffs_presentables = {
            let state_diffs_presentables = flow
                .graph()
                .iter_insertion()
                .map(|item| {
                    let item_id = item.id();
                    // Hack: for alignment
                    let padding =
                        " ".repeat(padding.saturating_sub(format!("{item_id}").len() + 2));
                    match state_diffs_raw_map.get(item_id) {
                        Some(state_current) => (item_id, format!("{padding}: {state_current}")),
                        None => (item_id, format!("{padding}: <unknown>")),
                    }
                })
                .collect::<Vec<_>>();

            ListNumbered::new(state_diffs_presentables)
        };

        output
            .present(&(
                Heading::new(HeadingLevel::Level1, "State Diffs"),
                state_diffs_presentables,
                "\n",
            ))
            .await?;

        Ok(())
    }
}

macro_rules! run {
    ($output:ident, $flow_cmd:ident, $padding:expr) => {{
        $flow_cmd::run($output, true, |ctx| {
            async {
                let state_diffs = DiffCmd::current_and_goal(ctx).await?;

                let SingleProfileSingleFlowViewAndOutput {
                    output,
                    cmd_view: SingleProfileSingleFlowView { flow, .. },
                    ..
                } = ctx.view_and_output();
                Self::state_diffs_present(output, flow, &state_diffs, $padding).await?;

                Ok(())
            }
            .boxed_local()
        })
        .await
    }};
}

macro_rules! run_multi {
    ($output:ident, $flow_cmd:ident, $padding:expr, $profile_a:ident, $profile_b:ident) => {{
        $flow_cmd::multi_profile($output, move |ctx| {
            async move {
                let state_diffs =
                    DiffCmd::diff_profiles_current(ctx, &$profile_a, &$profile_b).await?;
                let MultiProfileSingleFlowView { output, flow, .. } = ctx.view();

                Self::state_diffs_present(output, flow, &state_diffs, $padding).await?;

                Ok(())
            }
            .boxed_local()
        })
        .await?;
    }};
}

use run;
use run_multi;
