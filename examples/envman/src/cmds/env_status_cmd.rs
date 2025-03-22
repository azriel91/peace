use futures::FutureExt;
use peace::{
    cmd_ctx::{CmdCtxSpsf, CmdCtxSpsfFields},
    cmd_model::CmdOutcome,
    fmt::{
        presentable::{Heading, HeadingLevel, ListNumberedAligned},
        PresentableExt,
    },
    rt::cmds::StatesCurrentReadCmd,
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

/// Shows the current stored state of the environment.
#[derive(Debug)]
pub struct EnvStatusCmd;

impl EnvStatusCmd {
    /// Shows the current stored state of the environment.
    ///
    /// # Parameters
    ///
    /// * `output`: Output to write the execution outcome.
    pub async fn run<O>(output: &mut O) -> Result<(), EnvManError>
    where
        O: OutputWrite + Send,
        EnvManError: From<<O as OutputWrite>::Error>,
    {
        let workspace = workspace()?;
        let env_man_flow = env_man_flow(output, &workspace).await?;
        match env_man_flow {
            EnvManFlow::AppUpload => run!(output, AppUploadCmd),
            EnvManFlow::EnvDeploy => run!(output, EnvCmd),
        }

        Ok(())
    }
}

macro_rules! run {
    ($output:ident, $flow_cmd:ident) => {{
        $flow_cmd::run($output, CmdOpts::default(), |cmd_ctx| {
            run_with_ctx(cmd_ctx).boxed_local()
        })
        .await?;
    }};
}

async fn run_with_ctx<O>(cmd_ctx: &mut EnvManCmdCtx<'_, O>) -> Result<(), EnvManError>
where
    O: OutputWrite,
    EnvManError: From<<O as OutputWrite>::Error>,
{
    let states_current_stored_outcome = StatesCurrentReadCmd::exec(cmd_ctx).await?;
    let CmdCtxSpsf {
        output,
        fields: CmdCtxSpsfFields { flow, .. },
        ..
    } = cmd_ctx;

    if let Some(states_current_stored) = states_current_stored_outcome.value() {
        let states_current_stored_raw_map = &***states_current_stored;

        let states_current_stored_presentables: ListNumberedAligned<_, _> = flow
            .graph()
            .iter_insertion()
            .map(|item| {
                let item_id = item.id();
                let state_presentable = match states_current_stored_raw_map.get(item_id) {
                    Some(state_current_stored) => {
                        format!("{state_current_stored}").left_presentable()
                    }
                    None => "<unknown>".right_presentable(),
                };

                (item_id, state_presentable)
            })
            .collect::<Vec<_>>()
            .into();

        output
            .present(&(
                Heading::new(HeadingLevel::Level1, "Current States (Stored)"),
                states_current_stored_presentables,
                "\n",
            ))
            .await?;
    }
    if let CmdOutcome::ItemError { errors, .. } = &states_current_stored_outcome {
        crate::output::item_errors_present(&mut **output, errors).await?;
    }

    Ok(())
}

use run;
