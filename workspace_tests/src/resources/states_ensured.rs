use peace::{
    cfg::{full_spec_id, FullSpecId},
    resources::{
        resources_type_state::{SetUp, WithStateDiffs, WithStatesCurrentAndDesired},
        type_reg::untagged::TypeMap,
        Resources, StateDiffs, States, StatesDesired, StatesEnsured, StatesMut,
    },
};

#[derive(Debug, Default, PartialEq)]
struct Res;

#[derive(Debug, Default, PartialEq)]
struct Value(u32);

#[test]
fn with_capacity_reserves_enough_capacity() {
    let states_ensured = StatesEnsured::with_capacity(100);
    assert!(states_ensured.capacity() >= 100);
}

#[test]
fn into_inner() {
    let states_ensured = test_states_ensured();

    let type_map = states_ensured.into_inner();

    assert_eq!(1, type_map.len())
}

#[test]
fn deref() {
    let states_ensured = StatesEnsured::new();

    // deref
    assert_eq!(0, states_ensured.len())
}

#[test]
fn from_type_map() {
    let _states_ensured = StatesEnsured::from(TypeMap::new());
}

#[test]
fn from_states_and_resources_with_state_diffs() {
    let resources_empty = Resources::new();
    let resources_set_up = Resources::<SetUp>::from(resources_empty);
    let resources_with_states_now_and_desired = Resources::<WithStatesCurrentAndDesired>::from((
        resources_set_up,
        States::new(),
        StatesDesired::new(),
    ));
    let resources_with_state_diffs = Resources::<WithStateDiffs>::from((
        resources_with_states_now_and_desired,
        StateDiffs::new(),
    ));

    let _states_ensured = StatesEnsured::from((States::new(), &resources_with_state_diffs));
}

#[test]
fn debug() {
    let states_ensured = test_states_ensured();

    assert_eq!(
        r#"StatesEnsured({FullSpecId("key"): TypedValue { type: "i32", value: 123 }})"#,
        format!("{states_ensured:?}")
    );
}

fn test_states_ensured() -> StatesEnsured {
    let resources_empty = Resources::new();
    let resources_set_up = Resources::<SetUp>::from(resources_empty);
    let resources_with_states_now_and_desired = Resources::<WithStatesCurrentAndDesired>::from((
        resources_set_up,
        States::new(),
        StatesDesired::new(),
    ));
    let resources_with_state_diffs = Resources::<WithStateDiffs>::from((
        resources_with_states_now_and_desired,
        StateDiffs::new(),
    ));

    let mut states_mut = StatesMut::new();
    states_mut.insert(full_spec_id!("key"), 123);
    let states = States::from(states_mut);

    StatesEnsured::from((states, &resources_with_state_diffs))
}
