use std::{borrow::Cow, str::FromStr};

use peace::cfg::{FullSpecId, FullSpecIdInvalidFmt};

#[test]
fn from_str_returns_ok_owned_for_valid_id() -> Result<(), FullSpecIdInvalidFmt<'static>> {
    let full_spec_id = FullSpecId::from_str("good_id")?;

    assert_eq!("good_id", *full_spec_id);
    Ok(())
}

#[test]
fn underscore_is_valid_first_character() -> Result<(), FullSpecIdInvalidFmt<'static>> {
    let full_spec_id = FullSpecId::new("_good_id")?;

    assert_eq!("_good_id", *full_spec_id);
    Ok(())
}

#[test]
fn new_unchecked_does_not_validate_id() -> Result<(), FullSpecIdInvalidFmt<'static>> {
    let full_spec_id = FullSpecId::new_unchecked("!valid");

    assert_eq!("!valid", *full_spec_id);
    Ok(())
}

#[test]
fn try_from_str_returns_ok_borrowed_for_valid_id() -> Result<(), FullSpecIdInvalidFmt<'static>> {
    let full_spec_id = FullSpecId::try_from("good_id")?;

    assert_eq!("good_id", *full_spec_id);
    Ok(())
}

#[test]
fn try_from_string_returns_ok_owned_for_valid_id() -> Result<(), FullSpecIdInvalidFmt<'static>> {
    let full_spec_id = FullSpecId::try_from(String::from("good_id"))?;

    assert_eq!("good_id", *full_spec_id);
    Ok(())
}

#[test]
fn from_str_returns_err_owned_for_invalid_id() {
    let error = FullSpecId::from_str("has space").unwrap_err();

    assert!(matches!(error.value(), Cow::Owned(_)));
    assert_eq!("has space", error.value());
}

#[test]
fn try_from_str_returns_err_borrowed_for_invalid_id() {
    let error = FullSpecId::try_from("has space").unwrap_err();

    assert!(matches!(error.value(), Cow::Borrowed(_)));
    assert_eq!("has space", error.value());
}

#[test]
fn try_from_string_returns_err_owned_for_invalid_id() {
    let error = FullSpecId::try_from(String::from("has space")).unwrap_err();

    assert!(matches!(error.value(), Cow::Owned(_)));
    assert_eq!("has space", error.value());
}

#[test]
fn display_returns_inner_str() -> Result<(), FullSpecIdInvalidFmt<'static>> {
    let full_spec_id = FullSpecId::try_from("good_id")?;

    assert_eq!("good_id", full_spec_id.to_string());
    Ok(())
}

#[test]
fn clone() -> Result<(), FullSpecIdInvalidFmt<'static>> {
    let full_spec_id_0 = FullSpecId::new("id")?;
    let full_spec_id_1 = full_spec_id_0.clone();

    assert_eq!(full_spec_id_0, full_spec_id_1);
    Ok(())
}

#[test]
fn debug() -> Result<(), FullSpecIdInvalidFmt<'static>> {
    let full_spec_id = FullSpecId::new("id")?;

    assert_eq!(r#"FullSpecId("id")"#, format!("{full_spec_id:?}"));
    Ok(())
}

#[test]
fn partial_eq_ne() -> Result<(), FullSpecIdInvalidFmt<'static>> {
    let full_spec_id_0 = FullSpecId::new("id0")?;
    let full_spec_id_1 = FullSpecId::new("id1")?;

    assert!(full_spec_id_0 != full_spec_id_1);
    Ok(())
}
