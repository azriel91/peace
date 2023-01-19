use std::{
    ffi::OsStr,
    path::{Path, PathBuf},
};

use peace::{
    cfg::{app_name, flow_id, profile, AppName, FlowId, Profile},
    resources::paths::{FlowDir, PeaceAppDir, PeaceDir, ProfileDir, StatesSavedFile},
};

#[test]
pub fn debug() {
    let states_saved_file = StatesSavedFile::from(Path::new("test_states.yaml").to_path_buf());

    assert_eq!(
        r#"StatesSavedFile("test_states.yaml")"#,
        format!("{states_saved_file:?}")
    );
}

#[test]
pub fn partial_eq() {
    let states_saved_file_0 = StatesSavedFile::from(Path::new("test_states.yaml").to_path_buf());
    let states_saved_file_1 = states_saved_file_0.clone();

    assert_eq!(states_saved_file_0, states_saved_file_1);
}

#[test]
pub fn from_path_buf() {
    let states_saved_file = StatesSavedFile::from(Path::new("test_states.yaml").to_path_buf());

    assert_eq!(Path::new("test_states.yaml"), &*states_saved_file);
}

#[test]
pub fn from_flow_dir_relative() {
    let app_name = app_name!();
    let peace_dir = PeaceDir::from(Path::new(".").to_path_buf());
    let profile = profile!("test_profile");
    let peace_app_dir = PeaceAppDir::from((&peace_dir, &app_name));
    let profile_dir = ProfileDir::from((&peace_app_dir, &profile));
    let flow_dir = FlowDir::from((&profile_dir, &flow_id!("test_flow")));
    let states_saved_file = StatesSavedFile::from(&flow_dir);

    let path = PathBuf::from_iter([
        ".",
        &**app_name!(),
        "test_profile",
        "test_flow",
        "states_saved.yaml",
    ]);
    assert_eq!(path, &*states_saved_file);
}

#[test]
pub fn into_inner_returns_path_buf() {
    let states_saved_file = StatesSavedFile::new(Path::new("test_states.yaml").to_path_buf());

    assert_eq!(
        Path::new("test_states.yaml").to_path_buf(),
        states_saved_file.into_inner()
    );
}

#[test]
pub fn as_ref_os_str() {
    let states_saved_file = StatesSavedFile::new(Path::new("test_states.yaml").to_path_buf());

    assert_eq!(
        OsStr::new("test_states.yaml"),
        <StatesSavedFile as AsRef<OsStr>>::as_ref(&states_saved_file)
    );
}

#[test]
pub fn as_ref_path() {
    let states_saved_file = StatesSavedFile::new(Path::new("test_states.yaml").to_path_buf());

    assert_eq!(
        Path::new("test_states.yaml"),
        <StatesSavedFile as AsRef<Path>>::as_ref(&states_saved_file)
    );
}
