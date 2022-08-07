use std::borrow::Cow;

use peace::cfg::ItemSpecIdInvalidFmt;

#[test]
fn debug() {
    let item_spec_id_invalid_fmt = ItemSpecIdInvalidFmt::new(Cow::Borrowed("invalid id"));

    assert_eq!(
        r#"ItemSpecIdInvalidFmt { value: "invalid id" }"#,
        format!("{item_spec_id_invalid_fmt:?}")
    );
}

#[test]
fn display() {
    let item_spec_id_invalid_fmt = ItemSpecIdInvalidFmt::new(Cow::Borrowed("invalid id"));

    assert_eq!(
        "`invalid id` is not a valid `ItemSpecId`.\n\
        `ItemSpecId`s must begin with a letter or underscore, and contain only letters, numbers, or underscores.",
        format!("{item_spec_id_invalid_fmt}")
    );
}
