use peace::resource_model::{ResourceInteractionPull, ResourceLocation};

#[test]
fn location_client() {
    let resource_interaction_pull = ResourceInteractionPull::new(
        vec![ResourceLocation::localhost()],
        vec![ResourceLocation::host("server".to_string())],
    );

    assert_eq!(
        vec![ResourceLocation::localhost()],
        resource_interaction_pull.location_client()
    );
}

#[test]
fn location_server() {
    let resource_interaction_pull = ResourceInteractionPull::new(
        vec![ResourceLocation::localhost()],
        vec![ResourceLocation::host("server".to_string())],
    );

    assert_eq!(
        vec![ResourceLocation::host("server".to_string())],
        resource_interaction_pull.location_server()
    );
}
