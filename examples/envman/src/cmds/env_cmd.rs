use futures::future::LocalBoxFuture;
use peace::{
    cfg::{app_name, item_spec_id, state::Generated, AppName, ItemSpecId, Profile},
    cmd::{
        ctx::CmdCtx,
        scopes::{MultiProfileSingleFlow, SingleProfileSingleFlowView},
    },
    data::marker::Current,
    fmt::presentln,
    params::ValueSpec,
    resources::resources::ts::SetUp,
    rt_model::{
        output::OutputWrite,
        params::{KeyKnown, KeyUnknown, ParamsKeysImpl},
        Workspace, WorkspaceSpec,
    },
};

use crate::{
    flows::EnvDeployFlow,
    item_specs::{
        peace_aws_iam_policy::IamPolicyState,
        peace_aws_iam_role::{IamRoleItemSpec, IamRoleParamsSpec},
    },
    model::{EnvManError, EnvType, ProfileParamsKey, WebAppFileId, WorkspaceParamsKey},
    rt_model::EnvManCmdCtx,
};

/// Runs a `*Cmd` that accesses the environment.
#[derive(Debug)]
pub struct EnvCmd;

impl EnvCmd {
    /// Runs a command on the environment with the active profile.
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
            &'fn_once mut EnvManCmdCtx<'_, O, SetUp>,
        ) -> LocalBoxFuture<'fn_once, Result<T, EnvManError>>,
    {
        let path = String::from("/");
        let iam_role_params_spec = IamRoleParamsSpec::<WebAppFileId>::new(
            ValueSpec::from_map(|profile: &Profile| Some(profile.to_string())),
            ValueSpec::Value(path),
            ValueSpec::from_map(|iam_policy_state: &Current<IamPolicyState>| {
                let IamPolicyState::Some {
                    policy_id_arn_version: Generated::Value(policy_id_arn_version),
                    ..
                } = iam_policy_state.as_ref()? else {
                    return None;
                };
                Some(policy_id_arn_version.arn().to_string())
            }),
        );

        cmd_ctx_init!(output, cmd_ctx, iam_role_params_spec);

        if profile_print {
            Self::profile_print(&mut cmd_ctx).await?;
        }

        let t = f(&mut cmd_ctx).await?;

        Ok(t)
    }

    /// Runs a multi-profile command using the `EnvDeploy` flow..
    ///
    /// # Parameters
    ///
    /// * `output`: Output to write the execution outcome.
    /// * `f`: The command to run.
    pub async fn multi_profile<O, T, F>(output: &mut O, f: F) -> Result<T, EnvManError>
    where
        O: OutputWrite<EnvManError>,
        for<'fn_once> F: FnOnce(
            &'fn_once mut CmdCtx<
                MultiProfileSingleFlow<
                    '_,
                    EnvManError,
                    O,
                    ParamsKeysImpl<
                        KeyKnown<WorkspaceParamsKey>,
                        KeyKnown<ProfileParamsKey>,
                        KeyUnknown,
                    >,
                    SetUp,
                >,
            >,
        ) -> LocalBoxFuture<'fn_once, Result<T, EnvManError>>,
    {
        let workspace = Workspace::new(
            app_name!(),
            #[cfg(not(target_arch = "wasm32"))]
            WorkspaceSpec::WorkingDir,
            #[cfg(target_arch = "wasm32")]
            WorkspaceSpec::SessionStorage,
        )?;
        let flow = EnvDeployFlow::flow().await?;

        let mut cmd_ctx = {
            let cmd_ctx_builder =
                CmdCtx::builder_multi_profile_single_flow::<EnvManError, _>(output, &workspace);
            crate::cmds::ws_and_profile_params_augment!(cmd_ctx_builder);

            cmd_ctx_builder.with_flow(&flow).await?
        };

        let t = f(&mut cmd_ctx).await?;

        Ok(t)
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

        let profile = workspace_params.get::<Profile, _>(&WorkspaceParamsKey::Profile);
        let env_type = profile_params.get::<EnvType, _>(&ProfileParamsKey::EnvType);

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
    ($output:ident, $cmd_ctx:ident, $iam_role_params_spec:ident) => {
        let workspace = Workspace::new(
            app_name!(),
            #[cfg(not(target_arch = "wasm32"))]
            WorkspaceSpec::WorkingDir,
            #[cfg(target_arch = "wasm32")]
            WorkspaceSpec::SessionStorage,
        )?;
        let flow = EnvDeployFlow::flow().await?;
        let profile_key = WorkspaceParamsKey::Profile;

        let mut $cmd_ctx = {
            let cmd_ctx_builder =
                CmdCtx::builder_single_profile_single_flow::<EnvManError, _>($output, &workspace);
            crate::cmds::ws_and_profile_params_augment!(cmd_ctx_builder);

            cmd_ctx_builder
                .with_profile_from_workspace_param(&profile_key)
                .with_flow(&flow)
                .with_item_spec_params::<IamRoleItemSpec<WebAppFileId>>(
                    item_spec_id!("iam_role"),
                    $iam_role_params_spec,
                )
                .await?
        };
    };
}

use cmd_ctx_init;
