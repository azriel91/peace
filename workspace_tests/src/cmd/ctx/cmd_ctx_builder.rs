use peace::{
    cfg::AppName,
    rt_model::{Workspace, WorkspaceSpec},
};

mod no_profile_no_flow_builder;
mod single_profile_single_flow_builder;

fn workspace(
    tempdir: tempfile::TempDir,
    app_name: AppName,
) -> Result<Workspace, Box<dyn std::error::Error>> {
    let workspace = {
        let workspace_spec = WorkspaceSpec::Path(tempdir.path().to_path_buf());
        Workspace::new(app_name, workspace_spec)?
    };
    Ok(workspace)
}
