use std::{
    ffi::OsStr,
    path::{Path, PathBuf},
};

use peace::{
    cfg::{flow_id, profile, FlowId, Profile},
    resources::{
        internal::FlowInitFile,
        paths::{FlowDir, PeaceDir, ProfileDir},
    },
};

#[test]
pub fn debug() {
    let flow_init_file = FlowInitFile::from(Path::new("init.yaml").to_path_buf());

    assert_eq!(
        r#"FlowInitFile("init.yaml")"#,
        format!("{flow_init_file:?}")
    );
}

#[test]
pub fn partial_eq() {
    let flow_init_file_0 = FlowInitFile::from(Path::new("init.yaml").to_path_buf());
    let flow_init_file_1 = flow_init_file_0.clone();

    assert_eq!(flow_init_file_0, flow_init_file_1);
}

#[test]
pub fn from_path_buf() {
    let flow_init_file = FlowInitFile::from(Path::new("init.yaml").to_path_buf());

    assert_eq!(Path::new("init.yaml"), &*flow_init_file);
}

#[test]
pub fn from_flow_dir_relative() {
    let peace_dir = PeaceDir::from(Path::new(".").to_path_buf());
    let profile = profile!("test_profile");
    let profile_dir = ProfileDir::from((&peace_dir, &profile));
    let flow_dir = FlowDir::from((&profile_dir, &flow_id!("test_flow")));
    let flow_init_file = FlowInitFile::from(&flow_dir);

    let path = PathBuf::from_iter([".", "test_profile", "test_flow", "init.yaml"]);
    assert_eq!(path, &*flow_init_file);
}

#[test]
pub fn into_inner_returns_path_buf() {
    let flow_init_file = FlowInitFile::new(Path::new("init.yaml").to_path_buf());

    assert_eq!(
        Path::new("init.yaml").to_path_buf(),
        flow_init_file.into_inner()
    );
}

#[test]
pub fn as_ref_os_str() {
    let flow_init_file = FlowInitFile::new(Path::new("init.yaml").to_path_buf());

    assert_eq!(
        OsStr::new("init.yaml"),
        <FlowInitFile as AsRef<OsStr>>::as_ref(&flow_init_file)
    );
}

#[test]
pub fn as_ref_path() {
    let flow_init_file = FlowInitFile::new(Path::new("init.yaml").to_path_buf());

    assert_eq!(
        Path::new("init.yaml"),
        <FlowInitFile as AsRef<Path>>::as_ref(&flow_init_file)
    );
}
