use futures::FutureExt;
use peace::{
    cfg::Profile,
    cmd::scopes::SingleProfileSingleFlowView,
    fmt::presentable::{Heading, HeadingLevel, ListNumbered},
    resources::states::StateDiffs,
    rt::cmds::{
        sub::{StatesDesiredReadCmd, StatesSavedReadCmd},
        DiffCmd,
    },
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
                let states_saved = StatesSavedReadCmd::exec(ctx).await?;
                let states_desired = StatesDesiredReadCmd::exec(ctx).await?;
                let SingleProfileSingleFlowView {
                    output,
                    flow,
                    resources,
                    ..
                } = ctx.view();
                let state_diffs =
                    DiffCmd::diff_any(flow, resources, &states_saved, &states_desired).await?;

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
        let states_saved_a = EnvCmd::run_with_profile(output, profile_a, |ctx| {
            StatesSavedReadCmd::exec(ctx).boxed_local()
        })
        .await?;
        EnvCmd::run_with_profile(output, profile_b, move |ctx| {
            async move {
                let states_saved_b = StatesSavedReadCmd::exec(ctx).await?;

                let SingleProfileSingleFlowView {
                    output,
                    flow,
                    resources,
                    ..
                } = ctx.view();

                let state_diffs =
                    DiffCmd::diff_any(flow, resources, &states_saved_a, &states_saved_b).await?;

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
                .map(|item_spec| {
                    let item_spec_id = item_spec.id();
                    // Hack: for alignment
                    let padding =
                        " ".repeat(18usize.saturating_sub(format!("{item_spec_id}").len() + 2));
                    match state_diffs_raw_map.get(item_spec_id) {
                        Some(state_current) => {
                            (item_spec_id, format!("{padding}: {state_current}"))
                        }
                        None => (item_spec_id, format!("{padding}: <unknown>")),
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
