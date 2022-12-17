use std::{
    ffi::OsStr,
    path::{Path, PathBuf},
};

use peace::{
    cfg::{flow_id, profile, FlowId, Profile},
    resources::paths::{FlowDir, PeaceDir, ProfileDir, StatesPreviousFile},
};

#[test]
pub fn debug() {
    let states_previous_file =
        StatesPreviousFile::from(Path::new("test_states.yaml").to_path_buf());

    assert_eq!(
        r#"StatesPreviousFile("test_states.yaml")"#,
        format!("{states_previous_file:?}")
    );
}

#[test]
pub fn partial_eq() {
    let states_previous_file_0 =
        StatesPreviousFile::from(Path::new("test_states.yaml").to_path_buf());
    let states_previous_file_1 = states_previous_file_0.clone();

    assert_eq!(states_previous_file_0, states_previous_file_1);
}

#[test]
pub fn from_path_buf() {
    let states_previous_file =
        StatesPreviousFile::from(Path::new("test_states.yaml").to_path_buf());

    assert_eq!(Path::new("test_states.yaml"), &*states_previous_file);
}

#[test]
pub fn from_flow_dir_relative() {
    let peace_dir = PeaceDir::from(Path::new(".").to_path_buf());
    let profile = profile!("test_profile");
    let profile_dir = ProfileDir::from((&peace_dir, &profile));
    let flow_dir = FlowDir::from((&profile_dir, &flow_id!("test_flow")));
    let states_previous_file = StatesPreviousFile::from(&flow_dir);

    let path = PathBuf::from_iter([".", "test_profile", "test_flow", "states_previous.yaml"]);
    assert_eq!(path, &*states_previous_file);
}

#[test]
pub fn into_inner_returns_path_buf() {
    let states_previous_file = StatesPreviousFile::new(Path::new("test_states.yaml").to_path_buf());

    assert_eq!(
        Path::new("test_states.yaml").to_path_buf(),
        states_previous_file.into_inner()
    );
}

#[test]
pub fn as_ref_os_str() {
    let states_previous_file = StatesPreviousFile::new(Path::new("test_states.yaml").to_path_buf());

    assert_eq!(
        OsStr::new("test_states.yaml"),
        <StatesPreviousFile as AsRef<OsStr>>::as_ref(&states_previous_file)
    );
}

#[test]
pub fn as_ref_path() {
    let states_previous_file = StatesPreviousFile::new(Path::new("test_states.yaml").to_path_buf());

    assert_eq!(
        Path::new("test_states.yaml"),
        <StatesPreviousFile as AsRef<Path>>::as_ref(&states_previous_file)
    );
}
