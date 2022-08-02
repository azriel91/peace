use peace::{
    cfg::{full_spec_id, FullSpecId},
    resources::{type_reg::untagged::TypeMap, StatesDesiredMut},
};

#[derive(Debug, Default, PartialEq)]
struct Res;

#[derive(Debug, Default, PartialEq)]
struct Value(u32);

#[test]
fn with_capacity_reserves_enough_capacity() {
    let states_desired_mut = StatesDesiredMut::with_capacity(100);
    assert!(states_desired_mut.capacity() >= 100);
}

#[test]
fn into_inner() {
    let states_desired_mut = test_states_desired_mut();

    let type_map = states_desired_mut.into_inner();

    assert_eq!(1, type_map.len())
}

#[test]
fn deref_and_deref_mut() {
    let mut states_desired_mut = StatesDesiredMut::new();

    // deref_mut
    states_desired_mut.insert(full_spec_id!("key"), 123);

    // deref
    assert_eq!(1, states_desired_mut.len())
}

#[test]
fn from_type_map() {
    let _states_desired_mut = StatesDesiredMut::from(TypeMap::new());
}

#[test]
fn debug() {
    let states_desired_mut = test_states_desired_mut();

    assert_eq!(
        r#"StatesDesiredMut({FullSpecId("key"): TypedValue { type: "i32", value: 123 }})"#,
        format!("{states_desired_mut:?}")
    );
}

fn test_states_desired_mut() -> StatesDesiredMut {
    let mut states_desired_mut = StatesDesiredMut::new();
    states_desired_mut.insert(full_spec_id!("key"), 123);

    states_desired_mut
}
