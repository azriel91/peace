use std::path::Path;

use peace::{
    cfg::{profile, Profile},
    rt_model::{Workspace, WorkspaceSpec},
};

#[tokio::test]
async fn init_initializes_dirs_using_profile_and_physically_creates_dirs()
-> Result<(), Box<dyn std::error::Error>> {
    let tempdir = tempfile::tempdir()?;
    let workspace = Workspace::init(
        &WorkspaceSpec::Path(tempdir.path().into()),
        profile!("test_profile"),
    )
    .await?;
    let workspace_dirs = workspace.dirs();

    let workspace_dir = tempdir.path();
    let peace_dir = tempdir
        .path()
        .join(workspace_dirs.peace_dir().file_name().unwrap());
    let profile_dir = peace_dir.join("test_profile");
    let profile_history_dir = profile_dir.join(".history");

    assert_eq!(
        workspace_dir,
        AsRef::<Path>::as_ref(workspace_dirs.workspace_dir())
    );
    assert_eq!(
        profile_dir,
        AsRef::<Path>::as_ref(workspace_dirs.profile_dir())
    );
    [
        workspace_dir,
        &peace_dir,
        &profile_dir,
        &profile_history_dir,
    ]
    .iter()
    .for_each(|dir| assert!(dir.exists()));
    Ok(())
}
