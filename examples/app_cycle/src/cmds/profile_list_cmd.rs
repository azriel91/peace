use peace::{
    cfg::{app_name, AppName, Profile},
    rt_model::{
        output::OutputWrite, ItemSpecGraph, ItemSpecGraphBuilder, Workspace, WorkspaceSpec,
    },
};

use crate::{
    cmds::CmdCtxBuilder,
    model::{AppCycleError, EnvType},
};

cfg_if::cfg_if! {
    if #[cfg(not(target_arch = "wasm32"))] {
        use std::{ffi::OsStr, str::FromStr};

        use peace::{
            resources::{
                internal::ProfileParamsFile,
                paths::{PeaceAppDir, ProfileDir},
                type_reg::untagged::{BoxDt, TypeReg}
            },
            rt_model::{cmd_context_params::ProfileParams, Storage},
        };

        use crate::rt_model::CmdContext;
    }
}

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
        let graph = Self::graph()?;

        #[cfg(not(target_arch = "wasm32"))]
        let cmd_context = CmdCtxBuilder::new(&workspace, &graph, output).await?;
        #[cfg(target_arch = "wasm32")]
        let _cmd_context = CmdCtxBuilder::new(&workspace, &graph, output).await?;

        #[cfg(not(target_arch = "wasm32"))]
        let profiles_list = Self::profiles_list(&cmd_context).await?;
        #[cfg(target_arch = "wasm32")]
        let profiles_list = Vec::<(Profile, EnvType)>::new();

        let profiles_presentable = profiles_list
            .iter()
            .map(|(profile, env_type)| (profile, " - type: ".to_string(), env_type))
            .collect::<Vec<_>>();
        output.present("# Profiles\n\n").await?;
        output.present(&profiles_presentable).await?;

        Ok(())
    }

    fn graph() -> Result<ItemSpecGraph<AppCycleError>, AppCycleError> {
        let graph_builder = ItemSpecGraphBuilder::<AppCycleError>::new();

        // No item specs, as we are just reading existing workspace and profile params.

        Ok(graph_builder.build())
    }

    /// Lists profiles in the `PeaceAppDir`, and read their environment type.
    #[cfg(not(target_arch = "wasm32"))]
    async fn profiles_list<'ctx, O, TS>(
        cmd_context: &CmdContext<'ctx, O, TS>,
    ) -> Result<Vec<(Profile, EnvType)>, AppCycleError> {
        let resources = cmd_context.resources();
        let peace_app_dir = &*resources.borrow::<PeaceAppDir>();
        let mut peace_app_read_dir = tokio::fs::read_dir(peace_app_dir).await.map_err(|error| {
            AppCycleError::PeaceAppDirRead {
                peace_app_dir: peace_app_dir.to_path_buf(),
                error,
            }
        })?;

        let mut profiles_list = Vec::new();
        while let Some(entry) = peace_app_read_dir.next_entry().await.map_err(|error| {
            AppCycleError::PeaceAppDirEntryRead {
                peace_app_dir: peace_app_dir.to_path_buf(),
                error,
            }
        })? {
            let file_type = entry.file_type().await.map_err(|error| {
                AppCycleError::PeaceAppDirEntryFileTypeRead {
                    path: entry.path(),
                    error,
                }
            })?;

            if file_type.is_dir() {
                let entry_path = entry.path();
                if let Some(dir_name) = entry_path.file_name().and_then(OsStr::to_str) {
                    // Assume this is a profile directory
                    let profile = Profile::from_str(dir_name).map_err(|error| {
                        AppCycleError::ProfileDirInvalidName {
                            dir_name: dir_name.to_string(),
                            path: entry_path.to_path_buf(),
                            error,
                        }
                    })?;

                    if profile == Profile::workspace_init() {
                        // This profile is Peace's special case which will not have an environment
                        // type.
                        continue;
                    }

                    let profile_dir = ProfileDir::new(entry_path);
                    let env_type = Self::profile_env_type(
                        &profile,
                        cmd_context.profile_params_type_reg(),
                        &profile_dir,
                    )
                    .await?;

                    profiles_list.push((profile, env_type))
                }

                // Assume non-UTF8 file names are not profile directories
            }
        }

        Ok(profiles_list)
    }

    /// Reads profile init params from the given profile directory to determine
    /// its environment type.
    #[cfg(not(target_arch = "wasm32"))]
    async fn profile_env_type(
        profile: &Profile,
        profile_params_type_reg: &TypeReg<String, BoxDt>,
        profile_dir: &ProfileDir,
    ) -> Result<EnvType, AppCycleError> {
        let profile_params_file = ProfileParamsFile::from(profile_dir);

        let profile_params: ProfileParams<String> = Storage
            .serialized_typemap_read_opt(
                "profile_env_type".to_string(),
                profile_params_type_reg,
                &profile_params_file,
                peace::rt_model::Error::ProfileParamsDeserialize,
            )
            .await
            .map_err(AppCycleError::PeaceRtError)?
            .ok_or_else(|| AppCycleError::ProfileParamsNone {
                profile: profile.clone(),
                profile_params_file: profile_params_file.clone(),
            })?;

        profile_params
            .get("env_type")
            .copied()
            .ok_or_else(|| AppCycleError::ProfileEnvTypeNone {
                profile: profile.clone(),
                profile_params_file,
            })
    }
}
