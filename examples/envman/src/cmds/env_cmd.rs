use futures::future::LocalBoxFuture;
use peace::{
    cfg::{app_name, AppName, Profile},
    cmd::{
        ctx::CmdCtx,
        scopes::{SingleProfileSingleFlow, SingleProfileSingleFlowView},
    },
    fmt::{presentln, Presentable},
    resources::resources::ts::SetUp,
    rt_model::{
        output::OutputWrite,
        params::{KeyKnown, ParamsKeysImpl},
        Workspace, WorkspaceSpec,
    },
};

use crate::{
    flows::EnvDeployFlow,
    model::{EnvManError, EnvType},
    rt_model::EnvManCmdCtx,
};

/// Runs a `*Cmd` that accesses the environment.
#[derive(Debug)]
pub struct EnvCmd;

impl EnvCmd {
    /// Runs a command on the environment.
    ///
    /// # Parameters
    ///
    /// * `output`: Output to write the execution outcome.
    /// * `profile_print`: Whether to print the profile used.
    /// * `f`: The command to run.
    pub async fn run<O, T, F>(output: &mut O, profile_print: bool, f: F) -> Result<T, EnvManError>
    where
        O: OutputWrite<EnvManError>,
        for<'fn_once> F: FnOnce(
            &'fn_once mut CmdCtx<
                SingleProfileSingleFlow<
                    '_,
                    EnvManError,
                    O,
                    ParamsKeysImpl<KeyKnown<String>, KeyKnown<String>, KeyKnown<String>>,
                    SetUp,
                >,
            >,
        ) -> LocalBoxFuture<'fn_once, Result<T, EnvManError>>,
    {
        cmd_ctx_init!(output, cmd_ctx);

        if profile_print {
            Self::profile_print(&mut cmd_ctx).await?;
        }

        let t = f(&mut cmd_ctx).await?;

        Ok(t)
    }

    /// Runs a command on the environment and presents the returned information.
    ///
    /// # Parameters
    ///
    /// * `output`: Output to write the execution outcome.
    /// * `profile_print`: Whether to print the profile used.
    /// * `f`: The command to run.
    pub async fn run_and_present<O, T, F>(
        output: &mut O,
        profile_print: bool,
        f: F,
    ) -> Result<(), EnvManError>
    where
        O: OutputWrite<EnvManError>,
        for<'fn_once> F: FnOnce(
            &'fn_once mut EnvManCmdCtx<'_, O, SetUp>,
        ) -> LocalBoxFuture<'fn_once, Result<T, EnvManError>>,
        T: Presentable,
    {
        cmd_ctx_init!(output, cmd_ctx);

        if profile_print {
            Self::profile_print(&mut cmd_ctx).await?;
        }

        let t = f(&mut cmd_ctx).await?;

        let output = cmd_ctx.output_mut();
        presentln!(output, [&t]);

        Ok(())
    }

    async fn profile_print<O>(cmd_ctx: &mut EnvManCmdCtx<'_, O, SetUp>) -> Result<(), EnvManError>
    where
        O: OutputWrite<EnvManError>,
    {
        let SingleProfileSingleFlowView {
            output,
            workspace_params,
            profile_params,
            ..
        } = cmd_ctx.view();

        let profile = workspace_params.get::<Profile, _>("profile");
        let env_type = profile_params.get::<EnvType, _>("env_type");

        if let Some((profile, env_type)) = profile.zip(env_type) {
            presentln!(
                output,
                ["Using profile ", profile, " -- type ", env_type, "\n"]
            );
        }

        Ok(())
    }
}

macro_rules! cmd_ctx_init {
    ($output:ident, $cmd_ctx:ident) => {
        let workspace = Workspace::new(
            app_name!(),
            #[cfg(not(target_arch = "wasm32"))]
            WorkspaceSpec::WorkingDir,
            #[cfg(target_arch = "wasm32")]
            WorkspaceSpec::SessionStorage,
        )?;
        let flow = EnvDeployFlow::flow().await?;
        let profile_key = String::from("profile");

        let mut $cmd_ctx = {
            let cmd_ctx_builder =
                CmdCtx::builder_single_profile_single_flow::<EnvManError, _>($output, &workspace);
            crate::cmds::ws_profile_and_flow_params_augment!(cmd_ctx_builder);

            cmd_ctx_builder
                .with_profile_from_workspace_param(&profile_key)
                .with_flow(&flow)
                .await?
        };
    };
}

use cmd_ctx_init;
