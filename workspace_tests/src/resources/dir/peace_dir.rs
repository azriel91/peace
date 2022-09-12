use std::{ffi::OsStr, path::Path};

use peace::resources::paths::{PeaceDir, WorkspaceDir};

#[test]
pub fn debug() {
    let peace_dir = PeaceDir::from(Path::new(".").to_path_buf());

    assert_eq!(r#"PeaceDir(".")"#, format!("{peace_dir:?}"));
}

#[test]
pub fn partial_eq() {
    let peace_dir_0 = PeaceDir::from(Path::new(".").to_path_buf());
    let peace_dir_1 = peace_dir_0.clone();

    assert_eq!(peace_dir_0, peace_dir_1);
}

#[test]
pub fn from_path_buf() {
    let peace_dir = PeaceDir::from(Path::new(".").to_path_buf());

    assert_eq!(Path::new("."), &*peace_dir);
}

#[test]
pub fn from_workspace_dir_relative() {
    let workspace_dir = WorkspaceDir::from(Path::new(".").to_path_buf());
    let peace_dir = PeaceDir::from(&workspace_dir);

    let mut path = Path::new(".").to_path_buf();
    path.push(PeaceDir::NAME);
    assert_eq!(path, &*peace_dir);
}

#[test]
pub fn from_workspace_dir_blank() {
    let workspace_dir = WorkspaceDir::from(Path::new("").to_path_buf());
    let peace_dir = PeaceDir::from(&workspace_dir);

    assert_eq!(Path::new(PeaceDir::NAME), &*peace_dir);
}

#[test]
pub fn into_inner_returns_path_buf() {
    let peace_dir = PeaceDir::new(Path::new(".").to_path_buf());

    assert_eq!(Path::new(".").to_path_buf(), peace_dir.into_inner());
}

#[test]
pub fn as_ref_os_str() {
    let peace_dir = PeaceDir::new(Path::new(".").to_path_buf());

    assert_eq!(
        OsStr::new("."),
        <PeaceDir as AsRef<OsStr>>::as_ref(&peace_dir)
    );
}

#[test]
pub fn as_ref_path() {
    let peace_dir = PeaceDir::new(Path::new(".").to_path_buf());

    assert_eq!(
        Path::new("."),
        <PeaceDir as AsRef<Path>>::as_ref(&peace_dir)
    );
}
