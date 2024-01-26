use peace::{cmd_rt::CmdBlock, rt::cmd_blocks::StatesGoalReadCmdBlock};

use crate::peace_cmd_ctx_type_params::PeaceCmdCtxTypeParams;

#[test]
fn input_type_names_is_empty() {
    let cmd_block = StatesGoalReadCmdBlock::<PeaceCmdCtxTypeParams>::new();

    let input_type_names: Vec<String> = cmd_block.input_type_names();

    assert_eq!(&[] as &[&str], input_type_names.as_slice());
}

#[test]
fn outcome_type_names_includes_states_goal_stored() {
    let cmd_block = StatesGoalReadCmdBlock::<PeaceCmdCtxTypeParams>::new();

    let outcome_type_names: Vec<String> = cmd_block.outcome_type_names();

    assert_eq!(&["States<GoalStored>"], outcome_type_names.as_slice());
}
