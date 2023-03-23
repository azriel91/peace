use futures::FutureExt;
use peace::{
    cmd::scopes::SingleProfileSingleFlowView,
    fmt::presentable::{Heading, HeadingLevel, ListNumbered},
    rt::cmds::DiffCmd,
    rt_model::output::OutputWrite,
};

use crate::{cmds::EnvCmd, model::AppCycleError};

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
    pub async fn run<O>(output: &mut O) -> Result<(), AppCycleError>
    where
        O: OutputWrite<AppCycleError> + Send,
    {
        EnvCmd::run(output, true, |ctx| {
            async {
                let state_diffs = DiffCmd::exec(ctx).await?;
                let state_diffs_raw_map = &**state_diffs;

                let SingleProfileSingleFlowView { output, flow, .. } = ctx.view();
                let state_diffs_presentables = {
                    let state_diffs_presentables = flow
                        .graph()
                        .iter_insertion()
                        .map(|item_spec| {
                            let item_spec_id = item_spec.id();
                            match state_diffs_raw_map.get(item_spec_id) {
                                Some(state_current) => (item_spec_id, format!(": {state_current}")),
                                None => (item_spec_id, String::from(": <unknown>")),
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
