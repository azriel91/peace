use std::{ffi::OsStr, path::Path};

use peace::{
    cfg::{app_name, profile, AppName, Profile},
    resources::paths::{PeaceAppDir, PeaceDir, ProfileDir},
};

#[test]
pub fn debug() {
    let profile_dir = ProfileDir::from(Path::new(".").to_path_buf());

    assert_eq!(r#"ProfileDir(".")"#, format!("{profile_dir:?}"));
}

#[test]
pub fn partial_eq() {
    let profile_dir_0 = ProfileDir::from(Path::new(".").to_path_buf());
    let profile_dir_1 = profile_dir_0.clone();

    assert_eq!(profile_dir_0, profile_dir_1);
}

#[test]
pub fn from_path_buf() {
    let profile_dir = ProfileDir::from(Path::new(".").to_path_buf());

    assert_eq!(Path::new("."), &*profile_dir);
}

#[test]
pub fn from_peace_dir_relative() {
    let app_name = app_name!();
    let peace_dir = PeaceDir::from(Path::new(".").to_path_buf());
    let profile = profile!("test_profile");
    let peace_app_dir = PeaceAppDir::from((&peace_dir, &app_name));
    let profile_dir = ProfileDir::from((&peace_app_dir, &profile));

    let mut path = Path::new(".").to_path_buf();
    path.push(&**app_name!());
    path.push("test_profile");
    assert_eq!(path, &*profile_dir);
}

#[test]
pub fn into_inner_returns_path_buf() {
    let profile_dir = ProfileDir::new(Path::new(".").to_path_buf());

    assert_eq!(Path::new(".").to_path_buf(), profile_dir.into_inner());
}

#[test]
pub fn as_ref_os_str() {
    let profile_dir = ProfileDir::new(Path::new(".").to_path_buf());

    assert_eq!(
        OsStr::new("."),
        <ProfileDir as AsRef<OsStr>>::as_ref(&profile_dir)
    );
}

#[test]
pub fn as_ref_path() {
    let profile_dir = ProfileDir::new(Path::new(".").to_path_buf());

    assert_eq!(
        Path::new("."),
        <ProfileDir as AsRef<Path>>::as_ref(&profile_dir)
    );
}
