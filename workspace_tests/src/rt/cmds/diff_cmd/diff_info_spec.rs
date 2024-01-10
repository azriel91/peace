use peace::{
    cfg::{profile},
    rt::cmds::{DiffInfoSpec, DiffStateSpec},
};

#[test]
fn clone() {
    let _diff_info_spec = Clone::clone(&DiffInfoSpec::new(
        &profile!("profile"),
        DiffStateSpec::Current,
    ));
}

#[test]
fn debug() {
    let profile = profile!("profile");
    let diff_info_spec = DiffInfoSpec::new(&profile, DiffStateSpec::Current);

    assert_eq!(
        "DiffInfoSpec { \
            profile: Profile(\"profile\"), \
            diff_state_spec: Current \
        }",
        format!("{diff_info_spec:?}")
    );
}

#[test]
fn partial_eq() {
    let profile = profile!("profile");
    let profile_2 = profile!("profile_2");
    let diff_info_spec_0 = DiffInfoSpec::new(&profile, DiffStateSpec::Current);
    let diff_info_spec_1 = DiffInfoSpec::new(&profile, DiffStateSpec::Current);
    let diff_info_spec_2 = DiffInfoSpec::new(&profile_2, DiffStateSpec::Current);

    assert_eq!(diff_info_spec_0, diff_info_spec_1);
    assert_ne!(diff_info_spec_0, diff_info_spec_2);
}
