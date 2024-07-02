use peace::item_model::{ItemInteractionPull, ItemLocation};

#[test]
fn location_client() {
    let item_interaction_pull = ItemInteractionPull::new(
        vec![ItemLocation::localhost()],
        vec![ItemLocation::host("server".to_string())],
    );

    assert_eq!(
        vec![ItemLocation::localhost()],
        item_interaction_pull.location_client()
    );
}

#[test]
fn location_server() {
    let item_interaction_pull = ItemInteractionPull::new(
        vec![ItemLocation::localhost()],
        vec![ItemLocation::host("server".to_string())],
    );

    assert_eq!(
        vec![ItemLocation::host("server".to_string())],
        item_interaction_pull.location_server()
    );
}
