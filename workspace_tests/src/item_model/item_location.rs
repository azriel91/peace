use std::{ffi::OsStr, path::Path};

use peace::item_model::{url::ParseError, ItemLocation, ItemLocationType, Url};

#[test]
fn group() {
    let item_location = ItemLocation::group("Cloud".to_string());

    assert_eq!(
        ItemLocation::new(
            "Cloud".to_string(),
            peace::item_model::ItemLocationType::Group
        ),
        item_location
    );
}

#[test]
fn host() {
    let item_location = ItemLocation::host("Server".to_string());

    assert_eq!(
        ItemLocation::new(
            "Server".to_string(),
            peace::item_model::ItemLocationType::Host
        ),
        item_location
    );
}

#[test]
fn host_unknown() {
    let item_location = ItemLocation::host_unknown();

    assert_eq!(
        ItemLocation::new(
            ItemLocation::HOST_UNKNOWN.to_string(),
            peace::item_model::ItemLocationType::Host
        ),
        item_location
    );
}

#[test]
fn host_from_url_https() -> Result<(), ParseError> {
    let item_location = ItemLocation::host_from_url(&Url::parse("https://example.com/resource")?);

    assert_eq!(
        ItemLocation::new(
            "example.com".to_string(),
            peace::item_model::ItemLocationType::Host
        ),
        item_location
    );

    Ok(())
}

#[test]
fn host_from_url_file() -> Result<(), ParseError> {
    let item_location = ItemLocation::host_from_url(&Url::parse("file:///path/to/resource")?);

    assert_eq!(
        ItemLocation::new(
            ItemLocation::LOCALHOST.to_string(),
            peace::item_model::ItemLocationType::Host
        ),
        item_location
    );

    Ok(())
}

#[test]
fn localhost() {
    let item_location = ItemLocation::localhost();

    assert_eq!(
        ItemLocation::new(
            ItemLocation::LOCALHOST.to_string(),
            peace::item_model::ItemLocationType::Host
        ),
        item_location
    );
}

#[test]
fn path() {
    let item_location = ItemLocation::path("/path/to/resource".to_string());

    assert_eq!(
        ItemLocation::new(
            "/path/to/resource".to_string(),
            peace::item_model::ItemLocationType::Path
        ),
        item_location
    );
}

#[test]
fn path_lossy() {
    let path = unsafe {
        Path::new(OsStr::from_encoded_bytes_unchecked(
            b"/path/to/lossy_fo\xF0\x90\x80.txt",
        ))
    };
    let item_location = ItemLocation::path_lossy(path);

    assert_eq!(
        ItemLocation::new(
            "/path/to/lossy_fo�.txt".to_string(),
            peace::item_model::ItemLocationType::Path
        ),
        item_location
    );
}

#[test]
fn name() {
    let item_location = ItemLocation::path("/path/to/resource".to_string());

    assert_eq!("/path/to/resource", item_location.name());
}

#[test]
fn r#type() {
    let item_location = ItemLocation::path("/path/to/resource".to_string());

    assert_eq!(ItemLocationType::Path, item_location.r#type());
}