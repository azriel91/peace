use peace::resource_model::{url::ParseError, ItemLocation, ItemLocationType, Url};

#[test]
fn group() {
    let resource_location = ItemLocation::group("Cloud".to_string());

    assert_eq!(
        ItemLocation::new(
            "Cloud".to_string(),
            peace::resource_model::ItemLocationType::Group
        ),
        resource_location
    );
}

#[test]
fn host() {
    let resource_location = ItemLocation::host("Server".to_string());

    assert_eq!(
        ItemLocation::new(
            "Server".to_string(),
            peace::resource_model::ItemLocationType::Host
        ),
        resource_location
    );
}

#[test]
fn host_unknown() {
    let resource_location = ItemLocation::host_unknown();

    assert_eq!(
        ItemLocation::new(
            ItemLocation::HOST_UNKNOWN.to_string(),
            peace::resource_model::ItemLocationType::Host
        ),
        resource_location
    );
}

#[test]
fn host_from_url_https() -> Result<(), ParseError> {
    let resource_location =
        ItemLocation::host_from_url(&Url::parse("https://example.com/resource")?);

    assert_eq!(
        ItemLocation::new(
            "example.com".to_string(),
            peace::resource_model::ItemLocationType::Host
        ),
        resource_location
    );

    Ok(())
}

#[test]
fn host_from_url_file() -> Result<(), ParseError> {
    let resource_location =
        ItemLocation::host_from_url(&Url::parse("file:///path/to/resource")?);

    assert_eq!(
        ItemLocation::new(
            ItemLocation::LOCALHOST.to_string(),
            peace::resource_model::ItemLocationType::Host
        ),
        resource_location
    );

    Ok(())
}

#[test]
fn localhost() {
    let resource_location = ItemLocation::localhost();

    assert_eq!(
        ItemLocation::new(
            ItemLocation::LOCALHOST.to_string(),
            peace::resource_model::ItemLocationType::Host
        ),
        resource_location
    );
}

#[test]
fn path() {
    let resource_location = ItemLocation::path("/path/to/resource".to_string());

    assert_eq!(
        ItemLocation::new(
            "/path/to/resource".to_string(),
            peace::resource_model::ItemLocationType::Path
        ),
        resource_location
    );
}

#[test]
fn name() {
    let resource_location = ItemLocation::path("/path/to/resource".to_string());

    assert_eq!("/path/to/resource", resource_location.name());
}

#[test]
fn r#type() {
    let resource_location = ItemLocation::path("/path/to/resource".to_string());

    assert_eq!(ItemLocationType::Path, resource_location.r#type());
}
