use peace::{
    cmd_rt::CmdBlock, rt::cmd_blocks::StatesCurrentReadCmdBlock,
    rt_model::params::ParamsKeysUnknown,
};

use crate::peace_test_error::PeaceTestError;

#[test]
fn input_type_names_is_empty() {
    let cmd_block = StatesCurrentReadCmdBlock::<PeaceTestError, ParamsKeysUnknown>::new();

    let input_type_names: Vec<String> = cmd_block.input_type_names();

    assert_eq!(&[] as &[&str], input_type_names.as_slice());
}

#[test]
fn outcome_type_names_includes_states_current_stored() {
    let cmd_block = StatesCurrentReadCmdBlock::<PeaceTestError, ParamsKeysUnknown>::new();

    let outcome_type_names: Vec<String> = cmd_block.outcome_type_names();

    assert_eq!(&["States<CurrentStored>"], outcome_type_names.as_slice());
}
