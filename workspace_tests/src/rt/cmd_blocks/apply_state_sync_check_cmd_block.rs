use peace::{
    cmd_rt::CmdBlock, rt::cmd_blocks::ApplyStateSyncCheckCmdBlock,
    rt_model::params::ParamsKeysUnknown,
};

use crate::peace_test_error::PeaceTestError;

#[test]
fn input_type_names_includes_states_compared() {
    macro_rules! assert_input_type_names {
        ($constructor:ident, $expected:expr) => {
            let cmd_block =
                ApplyStateSyncCheckCmdBlock::<PeaceTestError, ParamsKeysUnknown, _>::$constructor();

            let input_type_names: Vec<String> = cmd_block.input_type_names();

            assert_eq!($expected as &[&str], input_type_names.as_slice());
        };
    }

    assert_input_type_names!(none, &[]);
    assert_input_type_names!(current, &["States<CurrentStored>", "States<Current>"]);
    assert_input_type_names!(goal, &["States<GoalStored>", "States<Goal>"]);
    assert_input_type_names!(
        current_and_goal,
        &[
            "States<CurrentStored>",
            "States<Current>",
            "States<GoalStored>",
            "States<Goal>"
        ]
    );
}

#[test]
fn outcome_type_names_includes_states_compared() {
    macro_rules! assert_outcome_type_names {
        ($constructor:ident, $expected:expr) => {
            let cmd_block =
                ApplyStateSyncCheckCmdBlock::<PeaceTestError, ParamsKeysUnknown, _>::$constructor();

            let outcome_type_names = cmd_block.outcome_type_names();

            assert_eq!($expected as &[&str], outcome_type_names.as_slice());
        };
    }

    assert_outcome_type_names!(none, &[]);
    assert_outcome_type_names!(current, &["States<CurrentStored>", "States<Current>"]);
    assert_outcome_type_names!(goal, &["States<GoalStored>", "States<Goal>"]);
    assert_outcome_type_names!(
        current_and_goal,
        &[
            "States<CurrentStored>",
            "States<Current>",
            "States<GoalStored>",
            "States<Goal>"
        ]
    );
}
