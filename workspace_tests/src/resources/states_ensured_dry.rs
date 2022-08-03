use peace::{
    cfg::{full_spec_id, FullSpecId},
    resources::{
        resources_type_state::{SetUp, WithStateDiffs, WithStatesCurrentAndDesired},
        type_reg::untagged::TypeMap,
        Resources, StateDiffs, States, StatesDesired, StatesEnsuredDry, StatesMut,
    },
};

#[derive(Debug, Default, PartialEq)]
struct Res;

#[derive(Debug, Default, PartialEq)]
struct Value(u32);

#[test]
fn with_capacity_reserves_enough_capacity() {
    let states_ensured_dry = StatesEnsuredDry::with_capacity(100);
    assert!(states_ensured_dry.capacity() >= 100);
}

#[test]
fn into_inner() {
    let states_ensured_dry = test_states_ensured_dry();

    let type_map = states_ensured_dry.into_inner();

    assert_eq!(1, type_map.len())
}

#[test]
fn deref() {
    let states_ensured_dry = StatesEnsuredDry::new();

    // deref
    assert_eq!(0, states_ensured_dry.len())
}

#[test]
fn from_type_map() {
    let _states_ensured_dry = StatesEnsuredDry::from(TypeMap::new());
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

    let _states_ensured_dry = StatesEnsuredDry::from((States::new(), &resources_with_state_diffs));
}

#[test]
fn debug() {
    let states_ensured_dry = test_states_ensured_dry();

    assert_eq!(
        r#"StatesEnsuredDry({FullSpecId("key"): TypedValue { type: "i32", value: 123 }})"#,
        format!("{states_ensured_dry:?}")
    );
}

fn test_states_ensured_dry() -> StatesEnsuredDry {
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

    StatesEnsuredDry::from((states, &resources_with_state_diffs))
}
