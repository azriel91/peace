use futures::FutureExt;
use peace::{
    cfg::Profile,
    cmd::scopes::{MultiProfileSingleFlowView, SingleProfileSingleFlowView},
    fmt::presentable::{Heading, HeadingLevel, ListNumbered},
    resources::states::StateDiffs,
    rt::cmds::DiffCmd,
    rt_model::{output::OutputWrite, Flow},
};

use crate::{
    cmds::EnvCmd,
    model::{EnvDiffSelection, EnvManError},
};

/// Shows the diff between current and desired states of the environment.
#[derive(Debug)]
pub struct EnvDiffCmd;

impl EnvDiffCmd {
    /// Shows the diff between current and desired states of the environment.
    ///
    /// # Parameters
    ///
    /// * `output`: Output to write the execution outcome.
    /// * `profiles`: Profiles to compare.
    pub async fn run<O>(
        output: &mut O,
        env_diff_selection: EnvDiffSelection,
    ) -> Result<(), EnvManError>
    where
        O: OutputWrite<EnvManError> + Send,
    {
        match env_diff_selection {
            EnvDiffSelection::CurrentAndDesired => {
                Self::active_profile_current_vs_desired(output).await
            }
            EnvDiffSelection::DiffProfilesCurrent {
                profile_a,
                profile_b,
            } => Self::diff_profiles_current(output, profile_a, profile_b).await,
        }
    }

    async fn active_profile_current_vs_desired<O>(output: &mut O) -> Result<(), EnvManError>
    where
        O: OutputWrite<EnvManError> + Send,
    {
        EnvCmd::run(output, true, |ctx| {
            async {
                let state_diffs = DiffCmd::current_and_desired(ctx).await?;

                let SingleProfileSingleFlowView { output, flow, .. } = ctx.view();
                Self::state_diffs_present(output, flow, &state_diffs).await?;

                Ok(())
            }
            .boxed_local()
        })
        .await
    }

    async fn diff_profiles_current<O>(
        output: &mut O,
        profile_a: Profile,
        profile_b: Profile,
    ) -> Result<(), EnvManError>
    where
        O: OutputWrite<EnvManError> + Send,
    {
        EnvCmd::multi_profile(output, move |ctx| {
            async move {
                let state_diffs =
                    DiffCmd::diff_profiles_current(ctx, &profile_a, &profile_b).await?;
                let MultiProfileSingleFlowView { output, flow, .. } = ctx.view();

                Self::state_diffs_present(output, flow, &state_diffs).await?;

                Ok(())
            }
            .boxed_local()
        })
        .await?;

        Ok(())
    }

    async fn state_diffs_present<O>(
        output: &mut O,
        flow: &Flow<EnvManError>,
        state_diffs: &StateDiffs,
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
                        " ".repeat(18usize.saturating_sub(format!("{item_id}").len() + 2));
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
