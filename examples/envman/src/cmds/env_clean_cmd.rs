use futures::FutureExt;
use peace::{
    cmd::scopes::SingleProfileSingleFlowView,
    fmt::presentable::{Heading, HeadingLevel, ListNumbered},
    rt::cmds::{sub::StatesSavedReadCmd, CleanCmd},
    rt_model::{outcomes::CmdOutcome, output::OutputWrite},
};

use crate::{cmds::EnvCmd, model::EnvManError};

/// Cleans up (deletes) the environment.
#[derive(Debug)]
pub struct EnvCleanCmd;

impl EnvCleanCmd {
    /// Cleans up (deletes) the environment.
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
        let states_saved = EnvCmd::run(output, true, |ctx| {
            StatesSavedReadCmd::exec(ctx).boxed_local()
        })
        .await?;
        EnvCmd::run(output, false, |ctx| {
            async move {
                let states_saved_ref = &states_saved;
                let states_cleaned_outcome = CleanCmd::exec(ctx, states_saved_ref).await?;
                let CmdOutcome {
                    value: states_cleaned,
                    errors,
                } = &states_cleaned_outcome;
                let SingleProfileSingleFlowView { output, flow, .. } = ctx.view();

                if states_cleaned_outcome.is_ok() {
                    let states_cleaned_raw_map = &***states_cleaned;

                    let states_cleaned_presentables = {
                        let states_cleaned_presentables = flow
                            .graph()
                            .iter_insertion()
                            .map(|item_spec| {
                                let item_spec_id = item_spec.id();
                                // Hack: for alignment
                                let padding = " ".repeat(
                                    18usize.saturating_sub(format!("{item_spec_id}").len() + 2),
                                );
                                match states_cleaned_raw_map.get(item_spec_id) {
                                    Some(state_cleaned) => {
                                        (item_spec_id, format!("{padding}: {state_cleaned}"))
                                    }
                                    None => (item_spec_id, format!("{padding}: <unknown>")),
                                }
                            })
                            .collect::<Vec<_>>();

                        ListNumbered::new(states_cleaned_presentables)
                    };

                    output
                        .present(&(
                            Heading::new(HeadingLevel::Level1, "States Cleaned"),
                            states_cleaned_presentables,
                            "\n",
                        ))
                        .await?;
                } else {
                    crate::output::item_spec_errors_present(output, errors).await?;
                }

                Ok(())
            }
            .boxed_local()
        })
        .await?;

        Ok(())
    }
}
