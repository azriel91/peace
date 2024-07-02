use peace::resource_model::{ItemInteractionWithin, ItemLocation};

#[test]
fn location() {
    let resource_interaction_within =
        ItemInteractionWithin::new(vec![ItemLocation::localhost()]);

    assert_eq!(
        vec![ItemLocation::localhost()],
        resource_interaction_within.location()
    );
}
