use peace::{
    cfg::{item_spec_id, ItemSpecId},
    resources::{type_reg::untagged::TypeMap, States, StatesMut},
};

#[derive(Debug, Default, PartialEq)]
struct Res;

#[derive(Debug, Default, PartialEq)]
struct Value(u32);

#[test]
fn with_capacity_reserves_enough_capacity() {
    let states = States::with_capacity(100);
    assert!(states.capacity() >= 100);
}

#[test]
fn into_inner() {
    let states = test_states();

    let type_map = states.into_inner();

    assert_eq!(1, type_map.len())
}

#[test]
fn deref() {
    let states = States::new();

    // deref
    assert_eq!(0, states.len())
}

#[test]
fn from_type_map() {
    let _states = States::from(TypeMap::new());
}

#[test]
fn from_states_mut() {
    let _states = States::from(StatesMut::new());
}

#[test]
fn debug() {
    let states = test_states();

    assert_eq!(
        r#"States({ItemSpecId("key"): TypedValue { type: "i32", value: 123 }})"#,
        format!("{states:?}")
    );
}

fn test_states() -> States {
    let mut states = StatesMut::new();
    states.insert(item_spec_id!("key"), 123);

    States::from(states)
}
