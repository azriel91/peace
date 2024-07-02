use peace::resource_model::{url::ParseError, ResourceLocation, ResourceLocationType, Url};

#[test]
fn group() {
    let resource_location = ResourceLocation::group("Cloud".to_string());

    assert_eq!(
        ResourceLocation::new(
            "Cloud".to_string(),
            peace::resource_model::ResourceLocationType::Group
        ),
        resource_location
    );
}

#[test]
fn host() {
    let resource_location = ResourceLocation::host("Server".to_string());

    assert_eq!(
        ResourceLocation::new(
            "Server".to_string(),
            peace::resource_model::ResourceLocationType::Host
        ),
        resource_location
    );
}

#[test]
fn host_unknown() {
    let resource_location = ResourceLocation::host_unknown();

    assert_eq!(
        ResourceLocation::new(
            ResourceLocation::HOST_UNKNOWN.to_string(),
            peace::resource_model::ResourceLocationType::Host
        ),
        resource_location
    );
}

#[test]
fn host_from_url_https() -> Result<(), ParseError> {
    let resource_location =
        ResourceLocation::host_from_url(&Url::parse("https://example.com/resource")?);

    assert_eq!(
        ResourceLocation::new(
            "example.com".to_string(),
            peace::resource_model::ResourceLocationType::Host
        ),
        resource_location
    );

    Ok(())
}

#[test]
fn host_from_url_file() -> Result<(), ParseError> {
    let resource_location =
        ResourceLocation::host_from_url(&Url::parse("file:///path/to/resource")?);

    assert_eq!(
        ResourceLocation::new(
            ResourceLocation::LOCALHOST.to_string(),
            peace::resource_model::ResourceLocationType::Host
        ),
        resource_location
    );

    Ok(())
}

#[test]
fn localhost() {
    let resource_location = ResourceLocation::localhost();

    assert_eq!(
        ResourceLocation::new(
            ResourceLocation::LOCALHOST.to_string(),
            peace::resource_model::ResourceLocationType::Host
        ),
        resource_location
    );
}

#[test]
fn path() {
    let resource_location = ResourceLocation::path("/path/to/resource".to_string());

    assert_eq!(
        ResourceLocation::new(
            "/path/to/resource".to_string(),
            peace::resource_model::ResourceLocationType::Path
        ),
        resource_location
    );
}

#[test]
fn name() {
    let resource_location = ResourceLocation::path("/path/to/resource".to_string());

    assert_eq!("/path/to/resource", resource_location.name());
}

#[test]
fn r#type() {
    let resource_location = ResourceLocation::path("/path/to/resource".to_string());

    assert_eq!(ResourceLocationType::Path, resource_location.r#type());
}
