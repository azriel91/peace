use std::{borrow::Cow, str::FromStr};

use peace::cfg::{FlowId, FlowIdInvalidFmt};

#[test]
fn from_str_returns_ok_owned_for_valid_id() -> Result<(), FlowIdInvalidFmt<'static>> {
    let flow_id = FlowId::from_str("good_id")?;

    assert_eq!("good_id", *flow_id);
    Ok(())
}

#[test]
fn underscore_is_valid_first_character() -> Result<(), FlowIdInvalidFmt<'static>> {
    let flow_id = FlowId::new("_good_id")?;

    assert_eq!("_good_id", *flow_id);
    Ok(())
}

#[test]
fn new_unchecked_does_not_validate_id() -> Result<(), FlowIdInvalidFmt<'static>> {
    let flow_id = FlowId::new_unchecked("!valid");

    assert_eq!("!valid", *flow_id);
    Ok(())
}

#[test]
fn try_from_str_returns_ok_borrowed_for_valid_id() -> Result<(), FlowIdInvalidFmt<'static>> {
    let flow_id = FlowId::try_from("good_id")?;

    assert_eq!("good_id", *flow_id);
    Ok(())
}

#[test]
fn try_from_string_returns_ok_owned_for_valid_id() -> Result<(), FlowIdInvalidFmt<'static>> {
    let flow_id = FlowId::try_from(String::from("good_id"))?;

    assert_eq!("good_id", *flow_id);
    Ok(())
}

#[test]
fn from_str_returns_err_owned_for_invalid_id() {
    let error = FlowId::from_str("has space").unwrap_err();

    assert!(matches!(error.value(), Cow::Owned(_)));
    assert_eq!("has space", error.value());
}

#[test]
fn try_from_str_returns_err_borrowed_for_invalid_id() {
    let error = FlowId::try_from("has space").unwrap_err();

    assert!(matches!(error.value(), Cow::Borrowed(_)));
    assert_eq!("has space", error.value());
}

#[test]
fn try_from_string_returns_err_owned_for_invalid_id() {
    let error = FlowId::try_from(String::from("has space")).unwrap_err();

    assert!(matches!(error.value(), Cow::Owned(_)));
    assert_eq!("has space", error.value());
}

#[test]
fn display_returns_inner_str() -> Result<(), FlowIdInvalidFmt<'static>> {
    let flow_id = FlowId::try_from("good_id")?;

    assert_eq!("good_id", flow_id.to_string());
    Ok(())
}

#[test]
fn clone() -> Result<(), FlowIdInvalidFmt<'static>> {
    let flow_id_0 = FlowId::new("id")?;
    let flow_id_1 = flow_id_0.clone();

    assert_eq!(flow_id_0, flow_id_1);
    Ok(())
}

#[test]
fn debug() -> Result<(), FlowIdInvalidFmt<'static>> {
    let flow_id = FlowId::new("id")?;

    assert_eq!(r#"FlowId("id")"#, format!("{flow_id:?}"));
    Ok(())
}

#[test]
fn partial_eq_ne() -> Result<(), FlowIdInvalidFmt<'static>> {
    let flow_id_0 = FlowId::new("id0")?;
    let flow_id_1 = FlowId::new("id1")?;

    assert!(flow_id_0 != flow_id_1);
    Ok(())
}
