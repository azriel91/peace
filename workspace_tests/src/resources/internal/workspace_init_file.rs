use std::{
    ffi::OsStr,
    path::{Path, PathBuf},
};

use peace::resources::{internal::WorkspaceInitFile, paths::PeaceDir};

#[test]
pub fn debug() {
    let workspace_init_file = WorkspaceInitFile::from(Path::new("init.yaml").to_path_buf());

    assert_eq!(
        r#"WorkspaceInitFile("init.yaml")"#,
        format!("{workspace_init_file:?}")
    );
}

#[test]
pub fn partial_eq() {
    let workspace_init_file_0 = WorkspaceInitFile::from(Path::new("init.yaml").to_path_buf());
    let workspace_init_file_1 = workspace_init_file_0.clone();

    assert_eq!(workspace_init_file_0, workspace_init_file_1);
}

#[test]
pub fn from_path_buf() {
    let workspace_init_file = WorkspaceInitFile::from(Path::new("init.yaml").to_path_buf());

    assert_eq!(Path::new("init.yaml"), &*workspace_init_file);
}

#[test]
pub fn from_peace_dir_relative() {
    let peace_dir = PeaceDir::from(Path::new(".").to_path_buf());
    let workspace_init_file = WorkspaceInitFile::from(&peace_dir);

    let path = PathBuf::from_iter([".", "init.yaml"]);
    assert_eq!(path, &*workspace_init_file);
}

#[test]
pub fn into_inner_returns_path_buf() {
    let workspace_init_file = WorkspaceInitFile::new(Path::new("init.yaml").to_path_buf());

    assert_eq!(
        Path::new("init.yaml").to_path_buf(),
        workspace_init_file.into_inner()
    );
}

#[test]
pub fn as_ref_os_str() {
    let workspace_init_file = WorkspaceInitFile::new(Path::new("init.yaml").to_path_buf());

    assert_eq!(
        OsStr::new("init.yaml"),
        <WorkspaceInitFile as AsRef<OsStr>>::as_ref(&workspace_init_file)
    );
}

#[test]
pub fn as_ref_path() {
    let workspace_init_file = WorkspaceInitFile::new(Path::new("init.yaml").to_path_buf());

    assert_eq!(
        Path::new("init.yaml"),
        <WorkspaceInitFile as AsRef<Path>>::as_ref(&workspace_init_file)
    );
}
