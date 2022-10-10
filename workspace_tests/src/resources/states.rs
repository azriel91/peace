use peace::{
    cfg::{item_spec_id, ItemSpecId},
    resources::{internal::StatesMut, states::StatesCurrent, type_reg::untagged::TypeMap},
};

#[test]
fn with_capacity_reserves_enough_capacity() {
    let states = StatesCurrent::with_capacity(100);
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
    let states = StatesCurrent::new();

    // deref
    assert_eq!(0, states.len())
}

#[test]
fn from_type_map() {
    let _states = StatesCurrent::from(TypeMap::new_typed());
}

#[test]
fn from_states_mut() {
    let _states = StatesCurrent::from(StatesMut::new());
}

#[test]
fn debug() {
    let states = test_states();

    assert_eq!(
        r#"States({ItemSpecId("key"): TypedValue { type: "i32", value: 123 }}, PhantomData<peace_resources::states::ts::Current>)"#,
        format!("{states:?}")
    );
}

fn test_states() -> StatesCurrent {
    let mut states = StatesMut::new();
    states.insert(item_spec_id!("key"), 123);

    StatesCurrent::from(states)
}
