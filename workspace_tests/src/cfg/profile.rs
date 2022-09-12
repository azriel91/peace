use std::{borrow::Cow, str::FromStr};

use peace::cfg::{Profile, ProfileInvalidFmt};

#[test]
fn from_str_returns_ok_owned_for_valid_profile_name() -> Result<(), ProfileInvalidFmt<'static>> {
    let profile_name = Profile::from_str("good_profile_name")?;

    assert_eq!("good_profile_name", *profile_name);
    Ok(())
}

#[test]
fn underscore_is_valid_first_character() -> Result<(), ProfileInvalidFmt<'static>> {
    let profile_name = Profile::new("_good_profile_name")?;

    assert_eq!("_good_profile_name", *profile_name);
    Ok(())
}

#[test]
fn new_unchecked_does_not_validate_profile_name() -> Result<(), ProfileInvalidFmt<'static>> {
    let profile_name = Profile::new_unchecked("!valid");

    assert_eq!("!valid", *profile_name);
    Ok(())
}

#[test]
fn try_from_str_returns_ok_borrowed_for_valid_profile_name()
-> Result<(), ProfileInvalidFmt<'static>> {
    let profile_name = Profile::try_from("good_profile_name")?;

    assert_eq!("good_profile_name", *profile_name);
    Ok(())
}

#[test]
fn try_from_string_returns_ok_owned_for_valid_profile_name()
-> Result<(), ProfileInvalidFmt<'static>> {
    let profile_name = Profile::try_from(String::from("good_profile_name"))?;

    assert_eq!("good_profile_name", *profile_name);
    Ok(())
}

#[test]
fn from_str_returns_err_owned_for_invalid_profile_name() {
    let error = Profile::from_str("has space").unwrap_err();

    assert!(matches!(error.value(), Cow::Owned(_)));
    assert_eq!("has space", error.value());
}

#[test]
fn try_from_str_returns_err_borrowed_for_invalid_profile_name() {
    let error = Profile::try_from("has space").unwrap_err();

    assert!(matches!(error.value(), Cow::Borrowed(_)));
    assert_eq!("has space", error.value());
}

#[test]
fn try_from_string_returns_err_owned_for_invalid_profile_name() {
    let error = Profile::try_from(String::from("has space")).unwrap_err();

    assert!(matches!(error.value(), Cow::Owned(_)));
    assert_eq!("has space", error.value());
}

#[test]
fn display_returns_inner_str() -> Result<(), ProfileInvalidFmt<'static>> {
    let profile_name = Profile::try_from("good_profile_name")?;

    assert_eq!("good_profile_name", profile_name.to_string());
    Ok(())
}

#[test]
fn clone() -> Result<(), ProfileInvalidFmt<'static>> {
    let profile_name_0 = Profile::new("profile_name")?;
    let profile_name_1 = profile_name_0.clone();

    assert_eq!(profile_name_0, profile_name_1);
    Ok(())
}

#[test]
fn debug() -> Result<(), ProfileInvalidFmt<'static>> {
    let profile_name = Profile::new("profile_name")?;

    assert_eq!(r#"Profile("profile_name")"#, format!("{profile_name:?}"));
    Ok(())
}

#[test]
fn partial_eq_ne() -> Result<(), ProfileInvalidFmt<'static>> {
    let profile_name_0 = Profile::new("profile_name0")?;
    let profile_name_1 = Profile::new("profile_name1")?;

    assert!(profile_name_0 != profile_name_1);
    Ok(())
}
