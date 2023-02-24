use futures::stream::{self, StreamExt, TryStreamExt};
use peace::{
    cfg::{AppName, FlowId, Profile},
    resources::{
        internal::{FlowParamsFile, ProfileParamsFile, WorkspaceParamsFile},
        paths::{FlowDir, ProfileDir},
    },
    rt_model::{
        cmd_context_params::{FlowParams, ProfileParams, WorkspaceParams},
        Error, Storage, Workspace, WorkspaceSpec,
    },
};

mod multi_profile_single_flow_builder;
mod no_profile_no_flow_builder;
mod single_profile_no_flow_builder;
mod single_profile_single_flow_builder;

fn workspace(
    tempdir: &tempfile::TempDir,
    app_name: AppName,
) -> Result<Workspace, Box<dyn std::error::Error>> {
    let workspace = {
        let workspace_spec = WorkspaceSpec::Path(tempdir.path().to_path_buf());
        Workspace::new(app_name, workspace_spec)?
    };
    Ok(workspace)
}

async fn workspace_with(
    tempdir: &tempfile::TempDir,
    app_name: AppName,
    profiles_existing: &[Profile],
    flow_id: Option<&FlowId>,
) -> Result<Workspace, Box<dyn std::error::Error>> {
    let workspace = {
        let workspace_spec = WorkspaceSpec::Path(tempdir.path().to_path_buf());
        Workspace::new(app_name, workspace_spec)?
    };

    let peace_app_dir = workspace.dirs().peace_app_dir();
    tokio::fs::create_dir_all(peace_app_dir).await?;

    let workspace_params_file = WorkspaceParamsFile::from(peace_app_dir);
    let mut workspace_params = WorkspaceParams::new();
    workspace_params.insert(String::from("profile"), profiles_existing[0].clone());
    workspace_params.insert(String::from("something_else"), String::from("a string"));

    Storage
        .serialized_write(
            crate::fn_name_short!().to_string(),
            &workspace_params_file,
            &workspace_params,
            Error::WorkspaceParamsSerialize,
        )
        .await?;

    let profile_dirs = profiles_existing
        .iter()
        .map(|profile| ProfileDir::from((peace_app_dir, profile)));
    stream::iter(profile_dirs)
        .map(Result::<_, Box<dyn std::error::Error>>::Ok)
        .try_for_each(|profile_dir| async move {
            tokio::fs::create_dir_all(&profile_dir).await?;

            let profile_params_file = ProfileParamsFile::from(&profile_dir);
            let mut profile_params = ProfileParams::new();
            profile_params.insert(String::from("profile_param"), 1u32);
            profile_params.insert(String::from("profile_param_other"), 2u64);

            Storage
                .serialized_write(
                    crate::fn_name_short!().to_string(),
                    &profile_params_file,
                    &profile_params,
                    Error::ProfileParamsSerialize,
                )
                .await?;

            if let Some(flow_id) = flow_id {
                let flow_dir = FlowDir::from((&profile_dir, flow_id));
                tokio::fs::create_dir_all(&flow_dir).await?;

                let flow_params_file = FlowParamsFile::from(&flow_dir);
                let mut flow_params = FlowParams::new();
                flow_params.insert(String::from("flow_param"), String::from("flow param value"));
                flow_params.insert(String::from("flow_param_other"), 456u32);

                Storage
                    .serialized_write(
                        crate::fn_name_short!().to_string(),
                        &flow_params_file,
                        &flow_params,
                        Error::FlowParamsSerialize,
                    )
                    .await?;
            }

            Ok(())
        })
        .await?;

    Ok(workspace)
}
