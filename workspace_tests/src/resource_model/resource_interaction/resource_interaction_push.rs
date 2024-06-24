use peace::resource_model::{ResourceInteractionPush, ResourceLocation};

#[test]
fn location_from() {
    let resource_interaction_push = ResourceInteractionPush::new(
        vec![ResourceLocation::localhost()],
        vec![ResourceLocation::host("server".to_string())],
    );

    assert_eq!(
        vec![ResourceLocation::localhost()],
        resource_interaction_push.location_from()
    );
}

#[test]
fn location_to() {
    let resource_interaction_push = ResourceInteractionPush::new(
        vec![ResourceLocation::localhost()],
        vec![ResourceLocation::host("server".to_string())],
    );

    assert_eq!(
        vec![ResourceLocation::host("server".to_string())],
        resource_interaction_push.location_to()
    );
}
