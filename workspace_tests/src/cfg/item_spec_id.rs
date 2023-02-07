use std::{borrow::Cow, str::FromStr};

use peace::{
    cfg::{ItemSpecId, ItemSpecIdInvalidFmt},
    fmt::Presentable,
};

use crate::{FnInvocation, FnTrackerPresenter};

#[test]
fn from_str_returns_ok_owned_for_valid_id() -> Result<(), ItemSpecIdInvalidFmt<'static>> {
    let item_spec_id = ItemSpecId::from_str("good_id")?;

    assert_eq!("good_id", *item_spec_id);
    Ok(())
}

#[test]
fn underscore_is_valid_first_character() -> Result<(), ItemSpecIdInvalidFmt<'static>> {
    let item_spec_id = ItemSpecId::new("_good_id")?;

    assert_eq!("_good_id", *item_spec_id);
    Ok(())
}

#[test]
fn new_unchecked_does_not_validate_id() -> Result<(), ItemSpecIdInvalidFmt<'static>> {
    let item_spec_id = ItemSpecId::new_unchecked("!valid");

    assert_eq!("!valid", *item_spec_id);
    Ok(())
}

#[test]
fn try_from_str_returns_ok_borrowed_for_valid_id() -> Result<(), ItemSpecIdInvalidFmt<'static>> {
    let item_spec_id = ItemSpecId::try_from("good_id")?;

    assert_eq!("good_id", *item_spec_id);
    Ok(())
}

#[test]
fn try_from_string_returns_ok_owned_for_valid_id() -> Result<(), ItemSpecIdInvalidFmt<'static>> {
    let item_spec_id = ItemSpecId::try_from(String::from("good_id"))?;

    assert_eq!("good_id", *item_spec_id);
    Ok(())
}

#[test]
fn from_str_returns_err_owned_for_invalid_id() {
    let error = ItemSpecId::from_str("has space").unwrap_err();

    assert!(matches!(error.value(), Cow::Owned(_)));
    assert_eq!("has space", error.value());
}

#[test]
fn try_from_str_returns_err_borrowed_for_invalid_id() {
    let error = ItemSpecId::try_from("has space").unwrap_err();

    assert!(matches!(error.value(), Cow::Borrowed(_)));
    assert_eq!("has space", error.value());
}

#[test]
fn try_from_string_returns_err_owned_for_invalid_id() {
    let error = ItemSpecId::try_from(String::from("has space")).unwrap_err();

    assert!(matches!(error.value(), Cow::Owned(_)));
    assert_eq!("has space", error.value());
}

#[test]
fn display_returns_inner_str() -> Result<(), ItemSpecIdInvalidFmt<'static>> {
    let item_spec_id = ItemSpecId::try_from("good_id")?;

    assert_eq!("good_id", item_spec_id.to_string());
    Ok(())
}

#[tokio::test]
async fn present_uses_code_inline() -> Result<(), Box<dyn std::error::Error>> {
    let mut presenter = FnTrackerPresenter::new();
    let item_spec_id = ItemSpecId::try_from("item_spec_id")?;

    item_spec_id.present(&mut presenter).await?;

    assert_eq!(
        vec![FnInvocation::new(
            "code_inline",
            vec![Some(r#""item_spec_id""#.to_string())]
        )],
        presenter.fn_invocations()
    );
    Ok(())
}

#[test]
fn clone() -> Result<(), ItemSpecIdInvalidFmt<'static>> {
    let item_spec_id_0 = ItemSpecId::new("id")?;
    let item_spec_id_1 = item_spec_id_0.clone();

    assert_eq!(item_spec_id_0, item_spec_id_1);
    Ok(())
}

#[test]
fn debug() -> Result<(), ItemSpecIdInvalidFmt<'static>> {
    let item_spec_id = ItemSpecId::new("id")?;

    assert_eq!(r#"ItemSpecId("id")"#, format!("{item_spec_id:?}"));
    Ok(())
}

#[test]
fn partial_eq_ne() -> Result<(), ItemSpecIdInvalidFmt<'static>> {
    let item_spec_id_0 = ItemSpecId::new("id0")?;
    let item_spec_id_1 = ItemSpecId::new("id1")?;

    assert!(item_spec_id_0 != item_spec_id_1);
    Ok(())
}
