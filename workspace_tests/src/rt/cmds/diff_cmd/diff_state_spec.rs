use peace::rt::cmds::DiffStateSpec;

#[test]
fn clone() {
    let _diff_state_spec = Clone::clone(&DiffStateSpec::Current);
}

#[test]
fn debug() {
    assert_eq!("Current", format!("{:?}", DiffStateSpec::Current));
}

#[test]
fn partial_eq() {
    assert_eq!(DiffStateSpec::Current, DiffStateSpec::Current);
    assert_ne!(DiffStateSpec::Goal, DiffStateSpec::GoalStored);
}
