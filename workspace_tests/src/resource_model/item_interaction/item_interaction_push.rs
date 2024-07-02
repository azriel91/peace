use peace::resource_model::{ItemInteractionPush, ItemLocation};

#[test]
fn location_from() {
    let resource_interaction_push = ItemInteractionPush::new(
        vec![ItemLocation::localhost()],
        vec![ItemLocation::host("server".to_string())],
    );

    assert_eq!(
        vec![ItemLocation::localhost()],
        resource_interaction_push.location_from()
    );
}

#[test]
fn location_to() {
    let resource_interaction_push = ItemInteractionPush::new(
        vec![ItemLocation::localhost()],
        vec![ItemLocation::host("server".to_string())],
    );

    assert_eq!(
        vec![ItemLocation::host("server".to_string())],
        resource_interaction_push.location_to()
    );
}
