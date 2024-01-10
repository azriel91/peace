use std::{ffi::OsStr, path::Path};

use peace::{
    cfg::{app_name, profile},
    resources::paths::{PeaceAppDir, PeaceDir, ProfileDir, ProfileHistoryDir},
};

#[test]
pub fn debug() {
    let profile_history_dir = ProfileHistoryDir::from(Path::new(".").to_path_buf());

    assert_eq!(
        r#"ProfileHistoryDir(".")"#,
        format!("{profile_history_dir:?}")
    );
}

#[test]
pub fn partial_eq() {
    let profile_history_dir_0 = ProfileHistoryDir::from(Path::new(".").to_path_buf());
    #[allow(clippy::redundant_clone)] // https://github.com/rust-lang/rust-clippy/issues/9011
    let profile_history_dir_1 = profile_history_dir_0.clone();

    assert_eq!(profile_history_dir_0, profile_history_dir_1);
}

#[test]
pub fn from_path_buf() {
    let profile_history_dir = ProfileHistoryDir::from(Path::new(".").to_path_buf());

    assert_eq!(Path::new("."), &*profile_history_dir);
}

#[test]
pub fn from_profile_dir_relative() {
    let app_name = app_name!();
    let peace_dir = PeaceDir::from(Path::new(".").to_path_buf());
    let profile = profile!("test_profile");
    let peace_app_dir = PeaceAppDir::from((&peace_dir, &app_name));
    let profile_dir = ProfileDir::from((&peace_app_dir, &profile));
    let profile_history_dir = ProfileHistoryDir::from(&profile_dir);

    let mut path = Path::new(".").to_path_buf();
    path.push(&**app_name!());
    path.push("test_profile");
    path.push(".history");
    assert_eq!(path, &*profile_history_dir);
}

#[test]
pub fn into_inner_returns_path_buf() {
    let profile_history_dir = ProfileHistoryDir::new(Path::new(".").to_path_buf());

    assert_eq!(
        Path::new(".").to_path_buf(),
        profile_history_dir.into_inner()
    );
}

#[test]
pub fn as_ref_os_str() {
    let profile_history_dir = ProfileHistoryDir::new(Path::new(".").to_path_buf());

    assert_eq!(
        OsStr::new("."),
        <ProfileHistoryDir as AsRef<OsStr>>::as_ref(&profile_history_dir)
    );
}

#[test]
pub fn as_ref_path() {
    let profile_history_dir = ProfileHistoryDir::new(Path::new(".").to_path_buf());

    assert_eq!(
        Path::new("."),
        <ProfileHistoryDir as AsRef<Path>>::as_ref(&profile_history_dir)
    );
}
