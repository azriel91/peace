use futures::FutureExt;
use peace::{
    cmd::scopes::{SingleProfileSingleFlowView, SingleProfileSingleFlowViewAndOutput},
    fmt::presentable::{Heading, HeadingLevel, ListNumbered},
    rt::cmds::sub::StatesSavedReadCmd,
    rt_model::output::OutputWrite,
};

use crate::{
    cmds::{
        common::{env_man_flow, workspace},
        AppUploadCmd, EnvCmd,
    },
    model::{EnvManError, EnvManFlow},
};

/// Shows the saved state of the environment.
#[derive(Debug)]
pub struct EnvStatusCmd;

impl EnvStatusCmd {
    /// Shows the saved state of the environment.
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
        }

        Ok(())
    }
}

macro_rules! run {
    ($output:ident, $flow_cmd:ident, $padding:expr) => {{
        $flow_cmd::run($output, true, |ctx| {
            async {
                let states_saved = StatesSavedReadCmd::exec(ctx).await?;
                let states_saved_raw_map = &**states_saved;

                let SingleProfileSingleFlowViewAndOutput {
                    output,
                    cmd_view: SingleProfileSingleFlowView { flow, .. },
                    ..
                } = ctx.view_and_output();
                let states_saved_presentables = {
                    let states_saved_presentables = flow
                        .graph()
                        .iter_insertion()
                        .map(|item| {
                            let item_id = item.id();
                            // Hack: for alignment
                            let padding =
                                " ".repeat($padding.saturating_sub(format!("{item_id}").len() + 2));
                            match states_saved_raw_map.get(item_id) {
                                Some(state_saved) => (item_id, format!("{padding}: {state_saved}")),
                                None => (item_id, format!("{padding}: <unknown>")),
                            }
                        })
                        .collect::<Vec<_>>();

                    ListNumbered::new(states_saved_presentables)
                };

                output
                    .present(&(
                        Heading::new(HeadingLevel::Level1, "States Saved"),
                        states_saved_presentables,
                        "\n",
                    ))
                    .await?;

                Ok(())
            }
            .boxed_local()
        })
        .await?;
    }};
}

use run;
