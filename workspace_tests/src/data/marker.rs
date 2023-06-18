use peace::data::marker::{ApplyDry, Clean, Current, Goal};

#[test]
fn debug() {
    assert_eq!("ApplyDry(Some(1))", format!("{:?}", ApplyDry(Some(1u8))));
    assert_eq!("Clean(Some(1))", format!("{:?}", Clean(Some(1u8))));
    assert_eq!("Current(Some(1))", format!("{:?}", Current(Some(1u8))));
    assert_eq!("Goal(Some(1))", format!("{:?}", Goal(Some(1u8))));
}
