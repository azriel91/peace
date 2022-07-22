use peace::{
    cfg::{full_spec_id, FullSpecId},
    resources::{type_reg::untagged::TypeMap, FullSpecStatesDesired},
};

#[derive(Debug, Default, PartialEq)]
struct Res;

#[derive(Debug, Default, PartialEq)]
struct Value(u32);

#[test]
fn with_capacity_reserves_enough_capacity() {
    let full_spec_states_desired = FullSpecStatesDesired::with_capacity(100);
    assert!(full_spec_states_desired.capacity() >= 100);
}

#[test]
fn into_inner() {
    let full_spec_states_desired = test_full_spec_states_desired();

    let type_map = full_spec_states_desired.into_inner();

    assert_eq!(1, type_map.len())
}

#[test]
fn deref_and_deref_mut() {
    let mut full_spec_states_desired = FullSpecStatesDesired::new();

    // deref_mut
    full_spec_states_desired.insert(full_spec_id!("key"), 123);

    // deref
    assert_eq!(1, full_spec_states_desired.len())
}

#[test]
fn from_type_map() {
    let _full_spec_states_desired = FullSpecStatesDesired::from(TypeMap::new());
}

#[test]
fn debug() {
    let full_spec_states_desired = test_full_spec_states_desired();

    assert_eq!(
        r#"FullSpecStatesDesired({FullSpecId("key"): TypedValue { type: "i32", value: 123 }})"#,
        format!("{full_spec_states_desired:?}")
    );
}

fn test_full_spec_states_desired() -> FullSpecStatesDesired {
    let mut full_spec_states_desired = FullSpecStatesDesired::new();
    full_spec_states_desired.insert(full_spec_id!("key"), 123);

    full_spec_states_desired
}
