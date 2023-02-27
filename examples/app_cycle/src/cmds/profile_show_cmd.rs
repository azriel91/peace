use peace::{
    cfg::{app_name, AppName, Profile},
    cmd::ctx::CmdCtx,
    fmt::presentln,
    rt_model::{output::OutputWrite, Workspace, WorkspaceSpec},
};
use peace_item_specs::{file_download::FileDownloadParams, tar_x::TarXParams};

use crate::model::{AppCycleError, EnvType, WebAppFileId};

/// Command to show the current profile.
#[derive(Debug)]
pub struct ProfileShowCmd;

impl ProfileShowCmd {
    /// Shows the currently active profile.
    ///
    /// The active profile is stored in workspace params.
    ///
    /// # Parameters
    ///
    /// * `output`: Output to write the execution outcome.
    pub async fn run<O>(output: &mut O) -> Result<(), AppCycleError>
    where
        O: OutputWrite<AppCycleError>,
    {
        let workspace = Workspace::new(
            app_name!(),
            #[cfg(not(target_arch = "wasm32"))]
            WorkspaceSpec::WorkingDir,
            #[cfg(target_arch = "wasm32")]
            WorkspaceSpec::SessionStorage,
        )?;

        // new CmdCtx
        let cmd_ctx = CmdCtx::builder_single_profile_no_flow::<AppCycleError>(&workspace)
            .with_workspace_params_k::<String>()
            .with_workspace_param::<Profile>(String::from("profile"), None)
            .with_workspace_param::<FileDownloadParams<WebAppFileId>>(
                String::from("web_app_file_download_params"),
                None,
            )
            .with_workspace_param::<TarXParams<WebAppFileId>>(
                String::from("web_app_tar_x_params"),
                None,
            )
            .with_profile_params_k::<String>()
            .with_profile_param::<EnvType>(String::from("env_type"), None)
            .with_profile_from_workspace_param(&String::from("profile"))
            .build()
            .await?;

        let workspace_params = cmd_ctx.workspace_params();
        let profile_params = cmd_ctx.profile_params();

        let profile = workspace_params.get::<Profile, _>("profile");
        let env_type = profile_params.get::<EnvType, _>("env_type");

        if let Some((profile, env_type)) = profile.zip(env_type) {
            presentln!(output, ["Using profile ", profile]);
            presentln!(output, ["Environment type: ", env_type]);
        }

        Ok(())
    }
}
