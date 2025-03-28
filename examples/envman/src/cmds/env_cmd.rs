use futures::future::LocalBoxFuture;
use peace::{
    cfg::app_name,
    cmd_ctx::{CmdCtxMpsf, CmdCtxSpsf, CmdCtxSpsfFields, ProfileSelection},
    fmt::presentln,
    item_model::item_id,
    params::Params,
    profile_model::Profile,
    rt_model::{output::OutputWrite, Workspace, WorkspaceSpec},
};

use crate::{
    cmds::CmdOpts,
    flows::EnvDeployFlow,
    items::{
        peace_aws_iam_policy::IamPolicyState,
        peace_aws_iam_role::{IamRoleItem, IamRoleParams},
    },
    model::{EnvManError, EnvManFlow, EnvType, ProfileParamsKey, WebApp, WorkspaceParamsKey},
    rt_model::{EnvManCmdCtx, EnvmanCmdCtxTypes},
};

/// Runs a `*Cmd` that accesses the environment.
#[derive(Debug)]
pub struct EnvCmd;

impl EnvCmd {
    /// Returns the `CmdCtx` for the `EnvDeployFlow`.
    pub async fn cmd_ctx<O>(output: &mut O) -> Result<EnvManCmdCtx<'_, O>, EnvManError>
    where
        O: OutputWrite,
        EnvManError: From<<O as OutputWrite>::Error>,
    {
        let workspace = Workspace::new(
            app_name!(),
            #[cfg(not(target_arch = "wasm32"))]
            WorkspaceSpec::WorkingDir,
            #[cfg(target_arch = "wasm32")]
            WorkspaceSpec::SessionStorage,
        )?;
        let flow = EnvDeployFlow::flow().await?;
        let profile_key = WorkspaceParamsKey::Profile;
        let iam_role_path = String::from("/");
        let iam_role_params_spec = IamRoleParams::<WebApp>::field_wise_spec()
            .with_name_from_map(|profile: &Profile| Some(profile.to_string()))
            .with_path(iam_role_path)
            .with_managed_policy_arn_from_map(IamPolicyState::policy_id_arn_version)
            .build();
        let cmd_ctx = {
            let cmd_ctx_builder = CmdCtxSpsf::<EnvmanCmdCtxTypes<O>>::builder()
                .with_output(output.into())
                .with_workspace(workspace.into());
            crate::cmds::interruptibility_augment!(cmd_ctx_builder);

            cmd_ctx_builder
                .with_profile_selection(ProfileSelection::FromWorkspaceParam(profile_key.into()))
                .with_flow(flow.into())
                .with_item_params::<IamRoleItem<WebApp>>(item_id!("iam_role"), iam_role_params_spec)
                .await?
        };
        Ok(cmd_ctx)
    }

    /// Runs a command on the environment with the active profile.
    ///
    /// # Parameters
    ///
    /// * `output`: Output to write the execution outcome.
    /// * `cmd_opts`: Options to configure the `Cmd`'s output.
    /// * `f`: The command to run.
    pub async fn run<O, T, F>(output: &mut O, cmd_opts: CmdOpts, f: F) -> Result<T, EnvManError>
    where
        O: OutputWrite,
        EnvManError: From<<O as OutputWrite>::Error>,
        for<'fn_once> F: FnOnce(
            &'fn_once mut EnvManCmdCtx<'_, O>,
        ) -> LocalBoxFuture<'fn_once, Result<T, EnvManError>>,
    {
        let mut cmd_ctx = Self::cmd_ctx(output).await?;

        let CmdOpts { profile_print } = cmd_opts;

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
        O: OutputWrite,
        EnvManError: From<<O as OutputWrite>::Error>,
        for<'fn_once> F: FnOnce(
            &'fn_once mut CmdCtxMpsf<EnvmanCmdCtxTypes<O>>,
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
            let cmd_ctx_builder = CmdCtxMpsf::<EnvmanCmdCtxTypes<O>>::builder()
                .with_output(output.into())
                .with_workspace((&workspace).into())
                .with_workspace_param::<peace::profile_model::Profile>(
                    WorkspaceParamsKey::Profile,
                    None,
                )
                .with_workspace_param::<EnvManFlow>(WorkspaceParamsKey::Flow, None);
            // .with_profile_param::<EnvType>(ProfileParamsKey::EnvType, None);
            crate::cmds::interruptibility_augment!(cmd_ctx_builder);

            // TODO: We don't yet know the profiles at this point, so we can't insert
            // profile params.
            //
            // ```rust
            // let iam_role_path = String::from("/");
            // let iam_role_params_spec = IamRoleParams::<WebApp>::field_wise_spec()
            //     .with_name_from_map(|profile: &Profile| Some(profile.to_string()))
            //     .with_path(iam_role_path)
            //     .with_managed_policy_arn_from_map(IamPolicyState::policy_id_arn_version)
            //     .build();
            // ```

            cmd_ctx_builder
                .with_flow((&flow).into())
                // ```rust
                // .with_item_params::<IamRoleItem<WebApp>>(item_id!("iam_role"), iam_role_params_spec)
                // ```
                .await?
        };

        let t = f(&mut cmd_ctx).await?;

        Ok(t)
    }

    async fn profile_print<O>(cmd_ctx: &mut EnvManCmdCtx<'_, O>) -> Result<(), EnvManError>
    where
        O: OutputWrite,
        EnvManError: From<<O as OutputWrite>::Error>,
    {
        let CmdCtxSpsf {
            output,
            fields:
                CmdCtxSpsfFields {
                    workspace_params,
                    profile_params,
                    ..
                },
            ..
        } = cmd_ctx;

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
