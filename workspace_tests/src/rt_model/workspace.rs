use std::path::Path;

use peace::{
    cfg::{profile, Profile},
    rt_model::{Workspace, WorkspaceSpec},
};

#[tokio::test]
async fn init_initializes_dirs_using_profile() -> Result<(), Box<dyn std::error::Error>> {
    let tempdir = tempfile::tempdir()?;
    let workspace = Workspace::try_new(
        &WorkspaceSpec::Path(tempdir.path().into()),
        profile!("test_profile"),
    )
    .await?;
    let workspace_dirs = workspace.dirs();

    assert_eq!(
        tempdir.path(),
        AsRef::<Path>::as_ref(workspace_dirs.workspace_dir())
    );
    assert_eq!(
        tempdir
            .path()
            .join(workspace_dirs.peace_dir().file_name().unwrap())
            .join("test_profile"),
        AsRef::<Path>::as_ref(workspace_dirs.profile_dir())
    );
    Ok(())
}
