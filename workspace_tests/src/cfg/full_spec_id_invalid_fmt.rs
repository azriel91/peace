use std::borrow::Cow;

use peace::cfg::FullSpecIdInvalidFmt;

#[test]
fn debug() {
    let full_spec_id_invalid_fmt = FullSpecIdInvalidFmt::new(Cow::Borrowed("invalid id"));

    assert_eq!(
        r#"FullSpecIdInvalidFmt { value: "invalid id" }"#,
        format!("{full_spec_id_invalid_fmt:?}")
    );
}

#[test]
fn display() {
    let full_spec_id_invalid_fmt = FullSpecIdInvalidFmt::new(Cow::Borrowed("invalid id"));

    assert_eq!(
        "`invalid id` is not a valid `FullSpecId`.\n\
        `FullSpecId`s must begin with a letter or underscore, and contain only letters, numbers, or underscores.",
        format!("{full_spec_id_invalid_fmt}")
    );
}
