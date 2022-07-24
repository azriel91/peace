use peace::{
    cfg::{full_spec_id, FullSpecId},
    resources::{type_reg::untagged::TypeMap, StatesMut},
};

#[derive(Debug, Default, PartialEq)]
struct Res;

#[derive(Debug, Default, PartialEq)]
struct Value(u32);

#[test]
fn with_capacity_reserves_enough_capacity() {
    let states = StatesMut::with_capacity(100);
    assert!(states.capacity() >= 100);
}

#[test]
fn into_inner() {
    let states = test_states();

    let type_map = states.into_inner();

    assert_eq!(1, type_map.len())
}

#[test]
fn deref_and_deref_mut() {
    let mut states = StatesMut::new();

    // deref_mut
    states.insert(full_spec_id!("key"), 123);

    // deref
    assert_eq!(1, states.len())
}

#[test]
fn from_type_map() {
    let _states = StatesMut::from(TypeMap::new());
}

#[test]
fn debug() {
    let states = test_states();

    assert_eq!(
        r#"StatesMut({FullSpecId("key"): TypedValue { type: "i32", value: 123 }})"#,
        format!("{states:?}")
    );
}

fn test_states() -> StatesMut {
    let mut states = StatesMut::new();
    states.insert(full_spec_id!("key"), 123);

    states
}
