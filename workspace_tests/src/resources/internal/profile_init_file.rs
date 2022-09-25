use std::{
    ffi::OsStr,
    path::{Path, PathBuf},
};

use peace::{
    cfg::{profile, Profile},
    resources::{
        internal::ProfileInitFile,
        paths::{PeaceDir, ProfileDir},
    },
};

#[test]
pub fn debug() {
    let profile_init_file = ProfileInitFile::from(Path::new("init.yaml").to_path_buf());

    assert_eq!(
        r#"ProfileInitFile("init.yaml")"#,
        format!("{profile_init_file:?}")
    );
}

#[test]
pub fn partial_eq() {
    let profile_init_file_0 = ProfileInitFile::from(Path::new("init.yaml").to_path_buf());
    let profile_init_file_1 = profile_init_file_0.clone();

    assert_eq!(profile_init_file_0, profile_init_file_1);
}

#[test]
pub fn from_path_buf() {
    let profile_init_file = ProfileInitFile::from(Path::new("init.yaml").to_path_buf());

    assert_eq!(Path::new("init.yaml"), &*profile_init_file);
}

#[test]
pub fn from_profile_dir_relative() {
    let peace_dir = PeaceDir::from(Path::new(".").to_path_buf());
    let profile = profile!("test_profile");
    let profile_dir = ProfileDir::from((&peace_dir, &profile));
    let profile_init_file = ProfileInitFile::from(&profile_dir);

    let path = PathBuf::from_iter([".", "test_profile", "init.yaml"]);
    assert_eq!(path, &*profile_init_file);
}

#[test]
pub fn into_inner_returns_path_buf() {
    let profile_init_file = ProfileInitFile::new(Path::new("init.yaml").to_path_buf());

    assert_eq!(
        Path::new("init.yaml").to_path_buf(),
        profile_init_file.into_inner()
    );
}

#[test]
pub fn as_ref_os_str() {
    let profile_init_file = ProfileInitFile::new(Path::new("init.yaml").to_path_buf());

    assert_eq!(
        OsStr::new("init.yaml"),
        <ProfileInitFile as AsRef<OsStr>>::as_ref(&profile_init_file)
    );
}

#[test]
pub fn as_ref_path() {
    let profile_init_file = ProfileInitFile::new(Path::new("init.yaml").to_path_buf());

    assert_eq!(
        Path::new("init.yaml"),
        <ProfileInitFile as AsRef<Path>>::as_ref(&profile_init_file)
    );
}
