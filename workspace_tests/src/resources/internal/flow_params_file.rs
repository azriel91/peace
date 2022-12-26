use std::{
    ffi::OsStr,
    path::{Path, PathBuf},
};

use peace::{
    cfg::{flow_id, profile, FlowId, Profile},
    resources::{
        internal::FlowParamsFile,
        paths::{FlowDir, PeaceDir, ProfileDir},
    },
};

#[test]
pub fn debug() {
    let flow_params_file = FlowParamsFile::from(Path::new("init.yaml").to_path_buf());

    assert_eq!(
        r#"FlowParamsFile("init.yaml")"#,
        format!("{flow_params_file:?}")
    );
}

#[test]
pub fn partial_eq() {
    let flow_params_file_0 = FlowParamsFile::from(Path::new("init.yaml").to_path_buf());
    let flow_params_file_1 = flow_params_file_0.clone();

    assert_eq!(flow_params_file_0, flow_params_file_1);
}

#[test]
pub fn from_path_buf() {
    let flow_params_file = FlowParamsFile::from(Path::new("init.yaml").to_path_buf());

    assert_eq!(Path::new("init.yaml"), &*flow_params_file);
}

#[test]
pub fn from_flow_dir_relative() {
    let peace_dir = PeaceDir::from(Path::new(".").to_path_buf());
    let profile = profile!("test_profile");
    let profile_dir = ProfileDir::from((&peace_dir, &profile));
    let flow_dir = FlowDir::from((&profile_dir, &flow_id!("test_flow")));
    let flow_params_file = FlowParamsFile::from(&flow_dir);

    let path = PathBuf::from_iter([".", "test_profile", "test_flow", "init.yaml"]);
    assert_eq!(path, &*flow_params_file);
}

#[test]
pub fn into_inner_returns_path_buf() {
    let flow_params_file = FlowParamsFile::new(Path::new("init.yaml").to_path_buf());

    assert_eq!(
        Path::new("init.yaml").to_path_buf(),
        flow_params_file.into_inner()
    );
}

#[test]
pub fn as_ref_os_str() {
    let flow_params_file = FlowParamsFile::new(Path::new("init.yaml").to_path_buf());

    assert_eq!(
        OsStr::new("init.yaml"),
        <FlowParamsFile as AsRef<OsStr>>::as_ref(&flow_params_file)
    );
}

#[test]
pub fn as_ref_path() {
    let flow_params_file = FlowParamsFile::new(Path::new("init.yaml").to_path_buf());

    assert_eq!(
        Path::new("init.yaml"),
        <FlowParamsFile as AsRef<Path>>::as_ref(&flow_params_file)
    );
}
