use futures::FutureExt;
use peace::{
    cmd::scopes::{SingleProfileSingleFlowView, SingleProfileSingleFlowViewAndOutput},
    fmt::presentable::{Heading, HeadingLevel, ListNumbered},
    rt::cmds::{sub::StatesSavedReadCmd, CleanCmd},
    rt_model::{outcomes::CmdOutcome, output::OutputWrite},
};

use crate::{
    cmds::{
        common::{env_man_flow, workspace},
        AppUploadCmd, EnvCmd,
    },
    model::{EnvManError, EnvManFlow},
};

/// Cleans up (deletes) the environment.
#[derive(Debug)]
pub struct EnvCleanCmd;

impl EnvCleanCmd {
    /// Cleans up (deletes) the environment.
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
        };

        Ok(())
    }
}

macro_rules! run {
    ($output:ident, $flow_cmd:ident, $padding:expr) => {{
        let states_saved = $flow_cmd::run($output, true, |ctx| {
            StatesSavedReadCmd::exec(ctx).boxed_local()
        })
        .await?;
        $flow_cmd::run($output, false, |ctx| {
            async move {
                let states_saved_ref = &states_saved;
                let states_cleaned_outcome = CleanCmd::exec(ctx, states_saved_ref).await?;
                let CmdOutcome {
                    value: states_cleaned,
                    errors,
                } = &states_cleaned_outcome;
                let SingleProfileSingleFlowViewAndOutput {
                    output,
                    cmd_view: SingleProfileSingleFlowView { flow, .. },
                    ..
                } = ctx.view_and_output();

                if states_cleaned_outcome.is_ok() {
                    let states_cleaned_raw_map = &***states_cleaned;

                    let states_cleaned_presentables = {
                        let states_cleaned_presentables = flow
                            .graph()
                            .iter_insertion()
                            .map(|item| {
                                let item_id = item.id();
                                // Hack: for alignment
                                let padding = " ".repeat(
                                    $padding.saturating_sub(format!("{item_id}").len() + 2),
                                );
                                match states_cleaned_raw_map.get(item_id) {
                                    Some(state_cleaned) => {
                                        (item_id, format!("{padding}: {state_cleaned}"))
                                    }
                                    None => (item_id, format!("{padding}: <unknown>")),
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
                    crate::output::item_errors_present(output, errors).await?;
                }

                Ok(())
            }
            .boxed_local()
        })
        .await?;
    }};
}

use run;
