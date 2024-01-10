use std::path::Path;

use peace::{
    cfg::app_name,
    rt_model::{Error, NativeError, WorkspaceDirsBuilder, WorkspaceSpec},
};

#[test]
fn returns_workspace_dir_from_working_directory() -> Result<(), Box<dyn std::error::Error>> {
    let workspace_dirs = WorkspaceDirsBuilder::build(&app_name!(), WorkspaceSpec::WorkingDir)?;

    let workspace_dir = workspace_dirs.workspace_dir();

    ({
        #[cfg_attr(coverage_nightly, coverage(off))]
        || {
            assert!(
                workspace_dir.ends_with("peace/workspace_tests"),
                "Expected `{}` to end with `peace/workspace_tests`",
                workspace_dir.display()
            );
        }
    })();
    Ok(())
}

#[test]
fn returns_workspace_dir_from_first_dir_with_file() -> Result<(), Box<dyn std::error::Error>> {
    let workspace_dirs = WorkspaceDirsBuilder::build(
        &app_name!(),
        WorkspaceSpec::FirstDirWithFile("Cargo.lock".into()),
    )?;

    let workspace_dir = workspace_dirs.workspace_dir();

    ({
        #[cfg_attr(coverage_nightly, coverage(off))]
        || {
            assert!(
                workspace_dir.ends_with("peace"),
                "Expected `{}` to end with `peace`",
                workspace_dir.display()
            );
        }
    })();
    Ok(())
}

#[test]
fn returns_workspace_file_not_found_when_workspace_root_file_does_not_exist()
-> Result<(), Box<dyn std::error::Error>> {
    let workspace_dirs_result = WorkspaceDirsBuilder::build(
        &app_name!(),
        WorkspaceSpec::FirstDirWithFile("non_existent_file".into()),
    );

    assert!(matches!(
        workspace_dirs_result,
        Err(Error::Native(NativeError::WorkspaceFileNotFound {
            working_dir: _,
            file_name,
        })) if file_name == Path::new("non_existent_file")
    ));
    Ok(())
}

#[test]
fn returns_workspace_dir_from_path() -> Result<(), Box<dyn std::error::Error>> {
    let tempdir = tempfile::tempdir()?;
    let workspace_dirs = WorkspaceDirsBuilder::build(
        &app_name!(),
        WorkspaceSpec::Path(Path::new(tempdir.path()).to_path_buf()),
    )?;

    let workspace_dir = workspace_dirs.workspace_dir();

    assert!(&**workspace_dir == tempdir.path());
    Ok(())
}

#[test]
fn returns_peace_dir_relative_to_workspace_dir() -> Result<(), Box<dyn std::error::Error>> {
    let workspace_dirs = WorkspaceDirsBuilder::build(
        &app_name!(),
        WorkspaceSpec::FirstDirWithFile("Cargo.lock".into()),
    )?;

    let peace_dir = workspace_dirs.peace_dir();

    ({
        #[cfg_attr(coverage_nightly, coverage(off))]
        || {
            assert!(
                peace_dir.ends_with("peace/.peace"),
                "Expected `{}` to end with `peace/.peace`",
                peace_dir.display()
            );
        }
    })();
    Ok(())
}

#[test]
fn returns_peace_app_dir_relative_to_peace_dir() -> Result<(), Box<dyn std::error::Error>> {
    let workspace_dirs = WorkspaceDirsBuilder::build(
        &app_name!(),
        WorkspaceSpec::FirstDirWithFile("Cargo.lock".into()),
    )?;

    let peace_app_dir = workspace_dirs.peace_app_dir();

    ({
        #[cfg_attr(coverage_nightly, coverage(off))]
        || {
            assert!(
                peace_app_dir.ends_with("peace/.peace/workspace_tests"),
                "Expected `{}` to end with `peace/.peace/workspace_tests`",
                peace_app_dir.display()
            );
        }
    })();
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
