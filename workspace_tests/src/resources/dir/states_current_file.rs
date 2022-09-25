use std::{
    ffi::OsStr,
    path::{Path, PathBuf},
};

use peace::{
    cfg::{flow_id, profile, FlowId, Profile},
    resources::paths::{FlowDir, PeaceDir, ProfileDir, StatesCurrentFile},
};

#[test]
pub fn debug() {
    let states_current_file = StatesCurrentFile::from(Path::new("test_states.yaml").to_path_buf());

    assert_eq!(
        r#"StatesCurrentFile("test_states.yaml")"#,
        format!("{states_current_file:?}")
    );
}

#[test]
pub fn partial_eq() {
    let states_current_file_0 =
        StatesCurrentFile::from(Path::new("test_states.yaml").to_path_buf());
    let states_current_file_1 = states_current_file_0.clone();

    assert_eq!(states_current_file_0, states_current_file_1);
}

#[test]
pub fn from_path_buf() {
    let states_current_file = StatesCurrentFile::from(Path::new("test_states.yaml").to_path_buf());

    assert_eq!(Path::new("test_states.yaml"), &*states_current_file);
}

#[test]
pub fn from_flow_dir_relative() {
    let peace_dir = PeaceDir::from(Path::new(".").to_path_buf());
    let profile = profile!("test_profile");
    let profile_dir = ProfileDir::from((&peace_dir, &profile));
    let flow_dir = FlowDir::from((&profile_dir, &flow_id!("test_flow")));
    let states_current_file = StatesCurrentFile::from(&flow_dir);

    let path = PathBuf::from_iter([".", "test_profile", "test_flow", "states_current.yaml"]);
    assert_eq!(path, &*states_current_file);
}

#[test]
pub fn into_inner_returns_path_buf() {
    let states_current_file = StatesCurrentFile::new(Path::new("test_states.yaml").to_path_buf());

    assert_eq!(
        Path::new("test_states.yaml").to_path_buf(),
        states_current_file.into_inner()
    );
}

#[test]
pub fn as_ref_os_str() {
    let states_current_file = StatesCurrentFile::new(Path::new("test_states.yaml").to_path_buf());

    assert_eq!(
        OsStr::new("test_states.yaml"),
        <StatesCurrentFile as AsRef<OsStr>>::as_ref(&states_current_file)
    );
}

#[test]
pub fn as_ref_path() {
    let states_current_file = StatesCurrentFile::new(Path::new("test_states.yaml").to_path_buf());

    assert_eq!(
        Path::new("test_states.yaml"),
        <StatesCurrentFile as AsRef<Path>>::as_ref(&states_current_file)
    );
}
