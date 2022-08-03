use peace::{
    cfg::{full_spec_id, FullSpecId},
    resources::{type_reg::untagged::TypeMap, StatesDesired, StatesDesiredMut},
};

#[derive(Debug, Default, PartialEq)]
struct Res;

#[derive(Debug, Default, PartialEq)]
struct Value(u32);

#[test]
fn with_capacity_reserves_enough_capacity() {
    let states_desired = StatesDesired::with_capacity(100);
    assert!(states_desired.capacity() >= 100);
}

#[test]
fn into_inner() {
    let states_desired = test_states_desired();

    let type_map = states_desired.into_inner();

    assert_eq!(1, type_map.len())
}

#[test]
fn deref() {
    let states_desired = StatesDesired::new();

    // deref
    assert_eq!(0, states_desired.len())
}

#[test]
fn from_type_map() {
    let _states_desired = StatesDesired::from(TypeMap::new());
}

#[test]
fn from_states_desired_mut() {
    let _states_desired = StatesDesired::from(StatesDesiredMut::new());
}

#[test]
fn debug() {
    let states_desired = test_states_desired();

    assert_eq!(
        r#"StatesDesired({FullSpecId("key"): TypedValue { type: "i32", value: 123 }})"#,
        format!("{states_desired:?}")
    );
}

fn test_states_desired() -> StatesDesired {
    let mut states_desired = StatesDesiredMut::new();
    states_desired.insert(full_spec_id!("key"), 123);

    StatesDesired::from(states_desired)
}
