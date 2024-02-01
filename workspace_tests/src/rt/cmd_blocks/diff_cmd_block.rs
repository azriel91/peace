use peace::{
    cmd_rt::CmdBlock,
    resources::states::ts::{Current, CurrentStored, Goal, GoalStored},
    rt::cmd_blocks::DiffCmdBlock,
};

use crate::peace_cmd_ctx_types::PeaceCmdCtxTypes;

#[test]
fn input_type_names_includes_states_ts0_and_states_ts1() {
    macro_rules! assert_input_type_names {
        ($states_ts0:ident, $states_ts1:ident, $expected:expr) => {
            let cmd_block = DiffCmdBlock::<PeaceCmdCtxTypes, $states_ts0, $states_ts1>::new();

            let input_type_names: Vec<String> = cmd_block.input_type_names();

            assert_eq!($expected as &[&str], input_type_names.as_slice());
        };
    }

    assert_input_type_names!(Current, Goal, &["States<Current>", "States<Goal>"]);
    assert_input_type_names!(
        CurrentStored,
        GoalStored,
        &["States<CurrentStored>", "States<GoalStored>"]
    );
}

#[test]
fn outcome_type_names_includes_state_diffs_states_ts0_and_states_ts1() {
    macro_rules! assert_outcome_type_names {
        ($states_ts0:ident, $states_ts1:ident, $expected:expr) => {
            let cmd_block = DiffCmdBlock::<PeaceCmdCtxTypes, $states_ts0, $states_ts1>::new();

            let outcome_type_names = cmd_block.outcome_type_names();

            assert_eq!($expected as &[&str], outcome_type_names.as_slice());
        };
    }

    assert_outcome_type_names!(
        Current,
        Goal,
        &["StateDiffs", "States<Current>", "States<Goal>"]
    );
    assert_outcome_type_names!(
        CurrentStored,
        GoalStored,
        &["StateDiffs", "States<CurrentStored>", "States<GoalStored>"]
    );
}
