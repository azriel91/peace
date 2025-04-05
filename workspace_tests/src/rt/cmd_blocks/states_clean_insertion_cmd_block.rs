use peace::{cmd_rt::CmdBlock, rt::cmd_blocks::StatesCleanInsertionCmdBlock};

use crate::peace_cmd_ctx_types::TestCctNoOpOutput;

#[test]
fn input_type_names_is_empty() {
    let cmd_block = StatesCleanInsertionCmdBlock::<TestCctNoOpOutput>::new();

    let input_type_names: Vec<String> = cmd_block.input_type_names();

    assert_eq!(&[] as &[&str], input_type_names.as_slice());
}

#[test]
fn outcome_type_names_includes_states_clean() {
    let cmd_block = StatesCleanInsertionCmdBlock::<TestCctNoOpOutput>::new();

    let outcome_type_names: Vec<String> = cmd_block.outcome_type_names();

    assert_eq!(&["States<Clean>"], outcome_type_names.as_slice());
}
