use std::{
    ffi::OsStr,
    path::{Path, PathBuf},
};

use peace::{
    cfg::{app_name, profile, AppName, Profile},
    resources::{
        internal::ProfileParamsFile,
        paths::{PeaceAppDir, PeaceDir, ProfileDir},
    },
};

#[test]
pub fn debug() {
    let profile_params_file = ProfileParamsFile::from(Path::new("init.yaml").to_path_buf());

    assert_eq!(
        r#"ProfileParamsFile("init.yaml")"#,
        format!("{profile_params_file:?}")
    );
}

#[test]
pub fn partial_eq() {
    let profile_params_file_0 = ProfileParamsFile::from(Path::new("init.yaml").to_path_buf());
    let profile_params_file_1 = profile_params_file_0.clone();

    assert_eq!(profile_params_file_0, profile_params_file_1);
}

#[test]
pub fn from_path_buf() {
    let profile_params_file = ProfileParamsFile::from(Path::new("init.yaml").to_path_buf());

    assert_eq!(Path::new("init.yaml"), &*profile_params_file);
}

#[test]
pub fn from_profile_dir_relative() {
    let app_name = app_name!();
    let peace_dir = PeaceDir::from(Path::new(".").to_path_buf());
    let profile = profile!("test_profile");
    let peace_app_dir = PeaceAppDir::from((&peace_dir, &app_name));
    let profile_dir = ProfileDir::from((&peace_app_dir, &profile));
    let profile_params_file = ProfileParamsFile::from(&profile_dir);

    let path = PathBuf::from_iter([".", &**app_name!(), "test_profile", "init.yaml"]);
    assert_eq!(path, &*profile_params_file);
}

#[test]
pub fn into_inner_returns_path_buf() {
    let profile_params_file = ProfileParamsFile::new(Path::new("init.yaml").to_path_buf());

    assert_eq!(
        Path::new("init.yaml").to_path_buf(),
        profile_params_file.into_inner()
    );
}

#[test]
pub fn as_ref_os_str() {
    let profile_params_file = ProfileParamsFile::new(Path::new("init.yaml").to_path_buf());

    assert_eq!(
        OsStr::new("init.yaml"),
        <ProfileParamsFile as AsRef<OsStr>>::as_ref(&profile_params_file)
    );
}

#[test]
pub fn as_ref_path() {
    let profile_params_file = ProfileParamsFile::new(Path::new("init.yaml").to_path_buf());

    assert_eq!(
        Path::new("init.yaml"),
        <ProfileParamsFile as AsRef<Path>>::as_ref(&profile_params_file)
    );
}
