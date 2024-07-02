use peace::resource_model::{ItemInteractionPull, ItemLocation};

#[test]
fn location_client() {
    let resource_interaction_pull = ItemInteractionPull::new(
        vec![ItemLocation::localhost()],
        vec![ItemLocation::host("server".to_string())],
    );

    assert_eq!(
        vec![ItemLocation::localhost()],
        resource_interaction_pull.location_client()
    );
}

#[test]
fn location_server() {
    let resource_interaction_pull = ItemInteractionPull::new(
        vec![ItemLocation::localhost()],
        vec![ItemLocation::host("server".to_string())],
    );

    assert_eq!(
        vec![ItemLocation::host("server".to_string())],
        resource_interaction_pull.location_server()
    );
}
