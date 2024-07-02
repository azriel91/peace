use peace::resource_model::{ResourceInteractionWithin, ItemLocation};

#[test]
fn location() {
    let resource_interaction_within =
        ResourceInteractionWithin::new(vec![ItemLocation::localhost()]);

    assert_eq!(
        vec![ItemLocation::localhost()],
        resource_interaction_within.location()
    );
}
