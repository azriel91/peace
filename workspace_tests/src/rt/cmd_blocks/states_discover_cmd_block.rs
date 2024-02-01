use peace::{cmd_rt::CmdBlock, rt::cmd_blocks::StatesDiscoverCmdBlock};

use crate::peace_cmd_ctx_types::PeaceCmdCtxTypes;

#[test]
fn input_type_names_includes_states_compared() {
    macro_rules! assert_input_type_names {
        ($constructor:ident) => {
            let cmd_block = StatesDiscoverCmdBlock::<PeaceCmdCtxTypes, _>::$constructor();

            let input_type_names: Vec<String> = cmd_block.input_type_names();

            assert_eq!(&[] as &[&str], input_type_names.as_slice());
        };
    }

    assert_input_type_names!(current);
    assert_input_type_names!(goal);
    assert_input_type_names!(current_and_goal);
}

#[test]
fn outcome_type_names_includes_states_compared() {
    macro_rules! assert_outcome_type_names {
        ($constructor:ident, $expected:expr) => {
            let cmd_block = StatesDiscoverCmdBlock::<PeaceCmdCtxTypes, _>::$constructor();

            let outcome_type_names = cmd_block.outcome_type_names();

            assert_eq!($expected as &[&str], outcome_type_names.as_slice());
        };
    }

    assert_outcome_type_names!(current, &["States<ItemIdT, Current>"]);
    assert_outcome_type_names!(goal, &["States<ItemIdT, Goal>"]);
    assert_outcome_type_names!(
        current_and_goal,
        &["States<ItemIdT, Current>", "States<ItemIdT, Goal>"]
    );
}
