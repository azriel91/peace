use peace::resource_model::{
    ItemInteraction, ItemInteractionPull, ItemInteractionPush,
    ItemInteractionWithin, ItemLocation,
};

mod resource_interaction_pull;
mod resource_interaction_push;
mod resource_interaction_within;

#[test]
fn from_resource_interaction_push() {
    let resource_interaction_push = ItemInteractionPush::new(
        vec![ItemLocation::localhost()],
        vec![ItemLocation::host("server".to_string())],
    );
    let resource_interaction = ItemInteraction::from(resource_interaction_push.clone());

    assert_eq!(
        ItemInteraction::Push(resource_interaction_push),
        resource_interaction
    );
}

#[test]
fn from_resource_interaction_pull() {
    let resource_interaction_pull = ItemInteractionPull::new(
        vec![ItemLocation::localhost()],
        vec![ItemLocation::host("server".to_string())],
    );
    let resource_interaction = ItemInteraction::from(resource_interaction_pull.clone());

    assert_eq!(
        ItemInteraction::Pull(resource_interaction_pull),
        resource_interaction
    );
}

#[test]
fn from_resource_interaction_within() {
    let resource_interaction_within =
        ItemInteractionWithin::new(vec![ItemLocation::localhost()]);
    let resource_interaction = ItemInteraction::from(resource_interaction_within.clone());

    assert_eq!(
        ItemInteraction::Within(resource_interaction_within),
        resource_interaction
    );
}
