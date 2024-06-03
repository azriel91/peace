use std::{ffi::OsStr, path::Path};

use peace::resource_rt::paths::WorkspaceDir;

#[test]
pub fn debug() {
    let workspace_dir = WorkspaceDir::from(Path::new(".").to_path_buf());

    assert_eq!(r#"WorkspaceDir(".")"#, format!("{workspace_dir:?}"));
}

#[test]
pub fn partial_eq() {
    let workspace_dir_0 = WorkspaceDir::from(Path::new(".").to_path_buf());
    #[allow(clippy::redundant_clone)] // https://github.com/rust-lang/rust-clippy/issues/9011
    let workspace_dir_1 = workspace_dir_0.clone();

    assert_eq!(workspace_dir_0, workspace_dir_1);
}

#[test]
pub fn from_path_buf() {
    let workspace_dir = WorkspaceDir::from(Path::new(".").to_path_buf());

    assert_eq!(Path::new("."), &*workspace_dir);
}

#[test]
pub fn into_inner_returns_path_buf() {
    let workspace_dir = WorkspaceDir::new(Path::new(".").to_path_buf());

    assert_eq!(Path::new(".").to_path_buf(), workspace_dir.into_inner());
}

#[test]
pub fn as_ref_os_str() {
    let workspace_dir = WorkspaceDir::new(Path::new(".").to_path_buf());

    assert_eq!(
        OsStr::new("."),
        <WorkspaceDir as AsRef<OsStr>>::as_ref(&workspace_dir)
    );
}

#[test]
pub fn as_ref_path() {
    let workspace_dir = WorkspaceDir::new(Path::new(".").to_path_buf());

    assert_eq!(
        Path::new("."),
        <WorkspaceDir as AsRef<Path>>::as_ref(&workspace_dir)
    );
}
