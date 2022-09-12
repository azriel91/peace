use std::borrow::Cow;

use peace::cfg::ProfileInvalidFmt;

#[test]
fn debug() {
    let item_spec_id_invalid_fmt = ProfileInvalidFmt::new(Cow::Borrowed("invalid profile"));

    assert_eq!(
        r#"ProfileInvalidFmt { value: "invalid profile" }"#,
        format!("{item_spec_id_invalid_fmt:?}")
    );
}

#[test]
fn display() {
    let item_spec_id_invalid_fmt = ProfileInvalidFmt::new(Cow::Borrowed("invalid profile"));

    assert_eq!(
        "`invalid profile` is not a valid `Profile`.\n\
        `Profile`s must begin with a letter or underscore, and contain only letters, numbers, or underscores.",
        format!("{item_spec_id_invalid_fmt}")
    );
}
