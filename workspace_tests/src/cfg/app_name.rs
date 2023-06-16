use std::{borrow::Cow, collections::HashMap, str::FromStr};

use peace::{
    cfg::{AppName, AppNameInvalidFmt},
    fmt::Presentable,
};

use crate::{FnInvocation, FnTrackerPresenter};

#[test]
fn from_str_returns_ok_owned_for_valid_app_name() -> Result<(), AppNameInvalidFmt<'static>> {
    let app_name = AppName::from_str("good_app_name")?;

    assert_eq!("good_app_name", *app_name);
    Ok(())
}

#[test]
fn underscore_is_valid_first_character() -> Result<(), AppNameInvalidFmt<'static>> {
    let app_name = AppName::new("_good_app_name")?;

    assert_eq!("_good_app_name", *app_name);
    Ok(())
}

#[test]
fn new_unchecked_does_not_validate_app_name() -> Result<(), AppNameInvalidFmt<'static>> {
    let app_name = AppName::new_unchecked("!valid");

    assert_eq!("!valid", *app_name);
    Ok(())
}

#[test]
fn try_from_str_returns_ok_borrowed_for_valid_app_name() -> Result<(), AppNameInvalidFmt<'static>> {
    let app_name = AppName::try_from("good_app_name")?;

    assert_eq!("good_app_name", *app_name);
    Ok(())
}

#[test]
fn try_from_string_returns_ok_owned_for_valid_app_name() -> Result<(), AppNameInvalidFmt<'static>> {
    let app_name = AppName::try_from(String::from("good_app_name"))?;

    assert_eq!("good_app_name", *app_name);
    Ok(())
}

#[test]
fn from_str_returns_err_owned_for_invalid_app_name() {
    let error = AppName::from_str("has space").unwrap_err();

    assert!(matches!(error.value(), Cow::Owned(_)));
    assert_eq!("has space", error.value());
}

#[test]
fn try_from_str_returns_err_borrowed_for_invalid_app_name() {
    let error = AppName::try_from("has space").unwrap_err();

    assert!(matches!(error.value(), Cow::Borrowed(_)));
    assert_eq!("has space", error.value());
}

#[test]
fn try_from_string_returns_err_owned_for_invalid_app_name() {
    let error = AppName::try_from(String::from("has space")).unwrap_err();

    assert!(matches!(error.value(), Cow::Owned(_)));
    assert_eq!("has space", error.value());
}

#[test]
fn display_returns_inner_str() -> Result<(), AppNameInvalidFmt<'static>> {
    let app_name = AppName::try_from("good_app_name")?;

    assert_eq!("good_app_name", app_name.to_string());
    Ok(())
}

#[tokio::test]
async fn present_uses_name() -> Result<(), Box<dyn std::error::Error>> {
    let mut presenter = FnTrackerPresenter::new();
    let app = AppName::try_from("app")?;

    app.present(&mut presenter).await?;

    assert_eq!(
        vec![FnInvocation::new(
            "name",
            vec![Some(r#""app""#.to_string())]
        )],
        presenter.fn_invocations()
    );
    Ok(())
}

#[test]
fn clone() -> Result<(), AppNameInvalidFmt<'static>> {
    let app_name_0 = AppName::new("app_name")?;
    #[allow(clippy::redundant_clone)] // https://github.com/rust-lang/rust-clippy/issues/9011
    let app_name_1 = app_name_0.clone();

    assert_eq!(app_name_0, app_name_1);
    Ok(())
}

#[test]
fn debug() -> Result<(), AppNameInvalidFmt<'static>> {
    let app_name = AppName::new("app_name")?;

    assert_eq!(r#"AppName("app_name")"#, format!("{app_name:?}"));
    Ok(())
}

#[test]
fn hash() -> Result<(), AppNameInvalidFmt<'static>> {
    let app_name = AppName::new("app_name")?;

    let mut hash_map = HashMap::new();
    hash_map.insert(app_name, ());

    Ok(())
}

#[test]
fn partial_eq_ne() -> Result<(), AppNameInvalidFmt<'static>> {
    let app_name_0 = AppName::new("app_name0")?;
    let app_name_1 = AppName::new("app_name1")?;

    assert!(app_name_0 != app_name_1);
    Ok(())
}

#[test]
fn serialize() -> Result<(), Box<dyn std::error::Error>> {
    let app_name = AppName::new("app_name")?;

    assert_eq!("app_name\n", serde_yaml::to_string(&app_name)?);
    Ok(())
}

#[test]
fn deserialize() -> Result<(), Box<dyn std::error::Error>> {
    let app_name = AppName::new("app_name")?;

    assert_eq!(app_name, serde_yaml::from_str("app_name")?);
    Ok(())
}
