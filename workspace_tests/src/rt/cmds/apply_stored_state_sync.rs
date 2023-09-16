use peace::rt::cmds::ApplyStoredStateSync;

#[test]
fn clone() {
    let _diff_state_spec = Clone::clone(&ApplyStoredStateSync::Both);
}

#[test]
fn debug() {
    assert_eq!("Both", format!("{:?}", ApplyStoredStateSync::Both));
}

#[test]
fn partial_eq() {
    assert_eq!(ApplyStoredStateSync::Both, ApplyStoredStateSync::Both);
    assert_ne!(ApplyStoredStateSync::Goal, ApplyStoredStateSync::Current);
}
