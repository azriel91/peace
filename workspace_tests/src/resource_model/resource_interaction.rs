use peace::resource_model::{
    ResourceInteraction, ResourceInteractionPull, ResourceInteractionPush,
    ResourceInteractionWithin, ItemLocation,
};

mod resource_interaction_pull;
mod resource_interaction_push;
mod resource_interaction_within;

#[test]
fn from_resource_interaction_push() {
    let resource_interaction_push = ResourceInteractionPush::new(
        vec![ItemLocation::localhost()],
        vec![ItemLocation::host("server".to_string())],
    );
    let resource_interaction = ResourceInteraction::from(resource_interaction_push.clone());

    assert_eq!(
        ResourceInteraction::Push(resource_interaction_push),
        resource_interaction
    );
}

#[test]
fn from_resource_interaction_pull() {
    let resource_interaction_pull = ResourceInteractionPull::new(
        vec![ItemLocation::localhost()],
        vec![ItemLocation::host("server".to_string())],
    );
    let resource_interaction = ResourceInteraction::from(resource_interaction_pull.clone());

    assert_eq!(
        ResourceInteraction::Pull(resource_interaction_pull),
        resource_interaction
    );
}

#[test]
fn from_resource_interaction_within() {
    let resource_interaction_within =
        ResourceInteractionWithin::new(vec![ItemLocation::localhost()]);
    let resource_interaction = ResourceInteraction::from(resource_interaction_within.clone());

    assert_eq!(
        ResourceInteraction::Within(resource_interaction_within),
        resource_interaction
    );
}
