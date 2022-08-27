use std::path::Path;

use peace::{
    cfg::{profile, Profile},
    rt_model::{Error, WorkspaceDirsBuilder, WorkspaceSpec},
};

#[test]
fn returns_workspace_dir_from_working_directory() -> Result<(), Box<dyn std::error::Error>> {
    let workspace_spec = WorkspaceSpec::WorkingDir;
    let profile = profile!("test_profile");

    let workspace_dirs = WorkspaceDirsBuilder::build(&workspace_spec, &profile)?;
    let workspace_dir = workspace_dirs.workspace_dir();

    assert!(
        workspace_dir.ends_with("peace/workspace_tests"),
        "Expected `{}` to end with `peace/workspace_tests`",
        workspace_dir.display()
    );

    Ok(())
}

#[test]
fn returns_workspace_dir_from_first_dir_with_file() -> Result<(), Box<dyn std::error::Error>> {
    let workspace_spec = WorkspaceSpec::FirstDirWithFile("Cargo.lock".into());
    let profile = profile!("test_profile");

    let workspace_dirs = WorkspaceDirsBuilder::build(&workspace_spec, &profile)?;
    let workspace_dir = workspace_dirs.workspace_dir();

    assert!(
        workspace_dir.ends_with("peace"),
        "Expected `{}` to end with `peace`",
        workspace_dir.display()
    );

    Ok(())
}

#[test]
fn returns_workspace_file_not_found_when_workspace_root_file_does_not_exist()
-> Result<(), Box<dyn std::error::Error>> {
    let workspace_spec = WorkspaceSpec::FirstDirWithFile("non_existent_file".into());
    let profile = profile!("test_profile");

    let workspace_dirs_result = WorkspaceDirsBuilder::build(&workspace_spec, &profile);

    assert!(matches!(
        workspace_dirs_result,
        Err(Error::WorkspaceFileNotFound {
            working_dir: _,
            file_name,
        }) if file_name == Path::new("non_existent_file")
    ));

    Ok(())
}

#[test]
fn returns_workspace_dir_from_path() -> Result<(), Box<dyn std::error::Error>> {
    let tempdir = tempfile::tempdir()?;
    let workspace_spec = WorkspaceSpec::Path(Path::new(tempdir.path()).to_path_buf());
    let profile = profile!("test_profile");

    let workspace_dirs = WorkspaceDirsBuilder::build(&workspace_spec, &profile)?;
    let workspace_dir = workspace_dirs.workspace_dir();

    assert!(&**workspace_dir == tempdir.path());

    Ok(())
}

#[test]
fn returns_peace_dir_relative_to_workspace_dir() -> Result<(), Box<dyn std::error::Error>> {
    let workspace_spec = WorkspaceSpec::FirstDirWithFile("Cargo.lock".into());
    let profile = profile!("test_profile");

    let workspace_dirs = WorkspaceDirsBuilder::build(&workspace_spec, &profile)?;
    let peace_dir = workspace_dirs.peace_dir();

    assert!(
        peace_dir.ends_with("peace/.peace"),
        "Expected `{}` to end with `peace/.peace`",
        peace_dir.display()
    );

    Ok(())
}

#[test]
fn returns_profile_history_dir_from_first_dir_with_file() -> Result<(), Box<dyn std::error::Error>>
{
    let workspace_spec = WorkspaceSpec::FirstDirWithFile("Cargo.lock".into());
    let profile = profile!("test_profile");

    let workspace_dirs = WorkspaceDirsBuilder::build(&workspace_spec, &profile)?;
    let profile_history_dir = workspace_dirs.profile_history_dir();

    assert!(
        profile_history_dir.ends_with("peace/.peace/test_profile/.history"),
        "Expected `{}` to end with `peace/.peace/test_profile/.history`",
        profile_history_dir.display()
    );

    Ok(())
}

#[test]
fn returns_profile_dir_from_working_directory() -> Result<(), Box<dyn std::error::Error>> {
    let workspace_spec = WorkspaceSpec::WorkingDir;
    let profile = profile!("test_profile");

    let workspace_dirs = WorkspaceDirsBuilder::build(&workspace_spec, &profile)?;
    let profile_dir = workspace_dirs.profile_dir();

    assert!(
        profile_dir.ends_with(Path::new("peace/workspace_tests/.peace/test_profile")),
        "Expected profile directory `{}` to end with `peace/workspace_tests/.peace/test_profile`",
        profile_dir.display()
    );

    Ok(())
}

#[test]
fn returns_profile_dir_from_first_dir_with_file() -> Result<(), Box<dyn std::error::Error>> {
    let workspace_spec = WorkspaceSpec::FirstDirWithFile("Cargo.lock".into());
    let profile = profile!("test_profile");

    let workspace_dirs = WorkspaceDirsBuilder::build(&workspace_spec, &profile)?;
    let profile_dir = workspace_dirs.profile_dir();

    assert!(
        profile_dir.ends_with("peace/.peace/test_profile"),
        "Expected `{}` to end with `peace/.peace/test_profile`",
        profile_dir.display()
    );

    Ok(())
}

#[test]
fn debug() {
    let workspace_dirs_builder = WorkspaceDirsBuilder;
    assert_eq!(
        "WorkspaceDirsBuilder",
        format!("{workspace_dirs_builder:?}")
    );
}
