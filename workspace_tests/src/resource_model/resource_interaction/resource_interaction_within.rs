use peace::resource_model::{ResourceInteractionWithin, ResourceLocation};

#[test]
fn location() {
    let resource_interaction_within =
        ResourceInteractionWithin::new(vec![ResourceLocation::localhost()]);

    assert_eq!(
        vec![ResourceLocation::localhost()],
        resource_interaction_within.location()
    );
}
