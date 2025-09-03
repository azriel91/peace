use futures::FutureExt;
use peace::{
    cmd_ctx::{CmdCtxSpsf, CmdCtxSpsfFields},
    cmd_model::CmdOutcome,
    fmt::{
        presentable::{Heading, HeadingLevel, ListNumberedAligned},
        PresentableExt,
    },
    rt::cmds::{ApplyStoredStateSync, CleanCmd},
    rt_model::output::OutputWrite,
};

use crate::{
    cmds::{
        common::{env_man_flow, workspace},
        AppUploadCmd, CmdOpts, EnvCmd,
    },
    model::{EnvManError, EnvManFlow},
    rt_model::EnvmanCmdCtxTypes,
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
    /// * `debug`: Whether to print `CmdOutcome` debug info.
    pub async fn run<O>(output: &mut O, debug: bool) -> Result<(), EnvManError>
    where
        O: OutputWrite + Send,
        EnvManError: From<<O as OutputWrite>::Error>,
    {
        let workspace = workspace()?;
        let env_man_flow = env_man_flow(output, &workspace).await?;
        match env_man_flow {
            EnvManFlow::AppUpload => run!(output, AppUploadCmd, debug),
            EnvManFlow::EnvDeploy => run!(output, EnvCmd, debug),
        };

        Ok(())
    }
}

macro_rules! run {
    ($output:ident, $flow_cmd:ident, $debug:expr) => {{
        $flow_cmd::run(
            $output,
            CmdOpts::default().with_profile_print(false),
            |cmd_ctx| run_with_ctx(cmd_ctx, $debug).boxed_local(),
        )
        .await?;
    }};
}

async fn run_with_ctx<O>(
    cmd_ctx: &mut CmdCtxSpsf<'_, EnvmanCmdCtxTypes<O>>,
    debug: bool,
) -> Result<(), EnvManError>
where
    O: OutputWrite,
    EnvManError: From<<O as OutputWrite>::Error>,
{
    let states_cleaned_outcome =
        CleanCmd::exec_with(cmd_ctx, ApplyStoredStateSync::Current).await?;
    let CmdCtxSpsf {
        output,
        fields: CmdCtxSpsfFields {
            flow, resources, ..
        },
        ..
    } = cmd_ctx;

    if let Some(states_cleaned) = states_cleaned_outcome.value() {
        let states_cleaned_raw_map = &***states_cleaned;

        let states_cleaned_presentables: ListNumberedAligned<_, _> = flow
            .graph()
            .iter_insertion()
            .map(|item| {
                let item_id = item.id();

                let state_cleaned_presentable = match states_cleaned_raw_map.get(item_id) {
                    Some(state_cleaned) => format!("{state_cleaned}").left_presentable(),
                    None => "<unknown>".right_presentable(),
                };

                (item_id, state_cleaned_presentable)
            })
            .collect::<Vec<_>>()
            .into();

        output
            .present(&(
                Heading::new(HeadingLevel::Level1, "States Cleaned"),
                states_cleaned_presentables,
                "\n",
            ))
            .await?;
    }
    if let CmdOutcome::ItemError { errors, .. } = &states_cleaned_outcome {
        crate::output::item_errors_present(&mut **output, errors).await?;
    }

    if debug {
        crate::output::cmd_outcome_completion_present(
            &mut **output,
            resources,
            states_cleaned_outcome,
        )
        .await?;
    }

    Ok(())
}

use run;
