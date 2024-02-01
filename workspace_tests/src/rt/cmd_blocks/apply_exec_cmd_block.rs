use peace::{
    cmd_rt::CmdBlock,
    resources::states::ts::{Cleaned, CleanedDry, Ensured, EnsuredDry},
    rt::cmd_blocks::ApplyExecCmdBlock,
};

use crate::peace_cmd_ctx_types::PeaceCmdCtxTypes;

#[test]
fn input_type_names_includes_states_current_and_states_target() {
    macro_rules! assert_input_type_names {
        ($states_ts:ident, $expected:expr) => {
            let cmd_block = ApplyExecCmdBlock::<PeaceCmdCtxTypes, $states_ts>::new();

            let input_type_names = cmd_block.input_type_names();

            assert_eq!($expected, input_type_names.as_slice());
        };
    }

    assert_input_type_names!(
        Ensured,
        &["States<ItemIdT, Current>", "States<ItemIdT, Goal>"]
    );
    assert_input_type_names!(
        EnsuredDry,
        &["States<ItemIdT, Current>", "States<ItemIdT, Goal>"]
    );
    assert_input_type_names!(
        Cleaned,
        &["States<ItemIdT, Current>", "States<ItemIdT, Clean>"]
    );
    assert_input_type_names!(
        CleanedDry,
        &["States<ItemIdT, Current>", "States<ItemIdT, Clean>"]
    );
}

#[test]
fn outcome_type_names_includes_states_previous_states_target() {
    macro_rules! assert_outcome_type_names {
        ($states_ts:ident, $expected:expr) => {
            let cmd_block = ApplyExecCmdBlock::<PeaceCmdCtxTypes, $states_ts>::new();

            let outcome_type_names = cmd_block.outcome_type_names();

            assert_eq!($expected, outcome_type_names.as_slice());
        };
    }

    assert_outcome_type_names!(
        Ensured,
        &[
            "States<ItemIdT, Previous>",
            "States<ItemIdT, Ensured>",
            "States<ItemIdT, Goal>"
        ]
    );
    assert_outcome_type_names!(
        EnsuredDry,
        &[
            "States<ItemIdT, Previous>",
            "States<ItemIdT, EnsuredDry>",
            "States<ItemIdT, Goal>"
        ]
    );
    assert_outcome_type_names!(
        Cleaned,
        &[
            "States<ItemIdT, Previous>",
            "States<ItemIdT, Cleaned>",
            "States<ItemIdT, Clean>"
        ]
    );
    assert_outcome_type_names!(
        CleanedDry,
        &[
            "States<ItemIdT, Previous>",
            "States<ItemIdT, CleanedDry>",
            "States<ItemIdT, Clean>"
        ]
    );
}
