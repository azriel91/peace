use peace::{
    cfg::{app_name, AppName, Profile},
    cmd::ctx::CmdCtx,
    rt_model::{output::OutputWrite, Workspace, WorkspaceSpec},
};
use peace_item_specs::{file_download::FileDownloadParams, tar_x::TarXParams};

use crate::model::{AppCycleError, EnvType, WebAppFileId};

/// Command to list initialized profiles.
#[derive(Debug)]
pub struct ProfileListCmd;

impl ProfileListCmd {
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
        let profile_workspace_init = Profile::workspace_init();
        let cmd_ctx =
            CmdCtx::builder_multi_profile_no_flow::<Box<dyn std::error::Error>>(&workspace)
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
                .with_profile_param::<EnvType>(String::from("env_type"))
                .with_profile_filter(|profile| profile != &profile_workspace_init)
                .build()
                .await?;

        output.present("# Profiles\n\n").await?;

        let profiles_presentable = cmd_ctx
            .profile_to_profile_params()
            .iter()
            .filter_map(|(profile, profile_params)| {
                let env_type = profile_params.get::<EnvType, _>("env_type");
                env_type.map(|env_type| (profile, " - type: ".to_string(), env_type))
            })
            .collect::<Vec<_>>();
        output.present(&profiles_presentable).await?;

        Ok(())
    }
}
