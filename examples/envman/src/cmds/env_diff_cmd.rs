use futures::FutureExt;
use peace::{
    cmd::scopes::SingleProfileSingleFlowView,
    fmt::presentable::{Heading, HeadingLevel, ListNumbered},
    rt::cmds::{
        sub::{StatesDesiredReadCmd, StatesSavedReadCmd},
        DiffCmd,
    },
    rt_model::output::OutputWrite,
};

use crate::{cmds::EnvCmd, model::EnvManError};

/// Shows the diff between current and desired states of the environment.
#[derive(Debug)]
pub struct EnvDiffCmd;

impl EnvDiffCmd {
    /// Shows the diff between current and desired states of the environment.
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
                let states_saved = StatesSavedReadCmd::exec(ctx).await?;
                let states_desired = StatesDesiredReadCmd::exec(ctx).await?;
                let SingleProfileSingleFlowView {
                    output,
                    flow,
                    resources,
                    ..
                } = ctx.view();
                let state_diffs =
                    DiffCmd::exec(flow, resources, &states_saved, &states_desired).await?;
                let state_diffs_raw_map = &**state_diffs;

                let state_diffs_presentables = {
                    let state_diffs_presentables = flow
                        .graph()
                        .iter_insertion()
                        .map(|item_spec| {
                            let item_spec_id = item_spec.id();
                            // Hack: for alignment
                            let padding = " ".repeat(
                                18usize.saturating_sub(format!("{item_spec_id}").len() + 2),
                            );
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
            .boxed_local()
        })
        .await
    }
}
