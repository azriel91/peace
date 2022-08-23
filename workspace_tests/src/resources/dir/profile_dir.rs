use std::{ffi::OsStr, path::Path};

use peace::{
    cfg::{profile, Profile},
    resources::dir::{PeaceDir, ProfileDir},
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
    let peace_dir = PeaceDir::from(Path::new(".").to_path_buf());
    let profile = profile!("my_profile");
    let profile_dir = ProfileDir::from((&peace_dir, &profile));

    let mut path = Path::new(".").to_path_buf();
    path.push("my_profile");
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
