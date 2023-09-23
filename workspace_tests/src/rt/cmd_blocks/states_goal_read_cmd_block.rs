use peace::{
    cmd_rt::CmdBlock, rt::cmd_blocks::StatesGoalReadCmdBlock, rt_model::params::ParamsKeysUnknown,
};

use crate::peace_test_error::PeaceTestError;

#[test]
fn input_type_names_is_empty() {
    let cmd_block = StatesGoalReadCmdBlock::<PeaceTestError, ParamsKeysUnknown>::new();

    let input_type_names: Vec<String> = cmd_block.input_type_names();

    assert_eq!(&[] as &[&str], input_type_names.as_slice());
}

#[test]
fn outcome_type_names_includes_states_goal_stored() {
    let cmd_block = StatesGoalReadCmdBlock::<PeaceTestError, ParamsKeysUnknown>::new();

    let outcome_type_names: Vec<String> = cmd_block.outcome_type_names();

    assert_eq!(&["States<GoalStored>"], outcome_type_names.as_slice());
}
