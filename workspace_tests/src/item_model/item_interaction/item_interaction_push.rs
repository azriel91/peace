use peace::resource_model::{ItemInteractionPush, ItemLocation};

#[test]
fn location_from() {
    let item_interaction_push = ItemInteractionPush::new(
        vec![ItemLocation::localhost()],
        vec![ItemLocation::host("server".to_string())],
    );

    assert_eq!(
        vec![ItemLocation::localhost()],
        item_interaction_push.location_from()
    );
}

#[test]
fn location_to() {
    let item_interaction_push = ItemInteractionPush::new(
        vec![ItemLocation::localhost()],
        vec![ItemLocation::host("server".to_string())],
    );

    assert_eq!(
        vec![ItemLocation::host("server".to_string())],
        item_interaction_push.location_to()
    );
}
