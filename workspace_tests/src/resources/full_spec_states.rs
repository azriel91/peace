use peace::{
    cfg::{full_spec_id, FullSpecId},
    resources::{type_reg::untagged::TypeMap, FullSpecStates},
};

#[derive(Debug, Default, PartialEq)]
struct Res;

#[derive(Debug, Default, PartialEq)]
struct Value(u32);

#[test]
fn with_capacity_reserves_enough_capacity() {
    let full_spec_states = FullSpecStates::with_capacity(100);
    assert!(full_spec_states.capacity() >= 100);
}

#[test]
fn into_inner() {
    let full_spec_states = test_full_spec_states();

    let type_map = full_spec_states.into_inner();

    assert_eq!(1, type_map.len())
}

#[test]
fn deref_and_deref_mut() {
    let mut full_spec_states = FullSpecStates::new();

    // deref_mut
    full_spec_states.insert(full_spec_id!("key"), 123);

    // deref
    assert_eq!(1, full_spec_states.len())
}

#[test]
fn from_type_map() {
    let _full_spec_states = FullSpecStates::from(TypeMap::new());
}

#[test]
fn debug() {
    let full_spec_states = test_full_spec_states();

    assert_eq!(
        r#"FullSpecStates({FullSpecId("key"): TypedValue { type: "i32", value: 123 }})"#,
        format!("{full_spec_states:?}")
    );
}

fn test_full_spec_states() -> FullSpecStates {
    let mut full_spec_states = FullSpecStates::new();
    full_spec_states.insert(full_spec_id!("key"), 123);

    full_spec_states
}
