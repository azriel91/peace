use peace::{
    cmd_rt::CmdBlock,
    resources::states::ts::{Cleaned, CleanedDry, Ensured, EnsuredDry},
    rt::cmd_blocks::ApplyExecCmdBlock,
    rt_model::params::ParamsKeysUnknown,
};

use crate::peace_test_error::PeaceTestError;

#[test]
fn input_type_names_includes_states_current_and_states_target() {
    macro_rules! assert_input_type_names {
        ($states_ts:ident, $expected:expr) => {
            let cmd_block =
                ApplyExecCmdBlock::<PeaceTestError, ParamsKeysUnknown, $states_ts>::new();

            let input_type_names = cmd_block.input_type_names();

            assert_eq!($expected, input_type_names.as_slice());
        };
    }

    assert_input_type_names!(Ensured, &["States<Current>", "States<Goal>"]);
    assert_input_type_names!(EnsuredDry, &["States<Current>", "States<Goal>"]);
    assert_input_type_names!(Cleaned, &["States<Current>", "States<Clean>"]);
    assert_input_type_names!(CleanedDry, &["States<Current>", "States<Clean>"]);
}

#[test]
fn outcome_type_names_includes_states_previous_states_target() {
    macro_rules! assert_outcome_type_names {
        ($states_ts:ident, $expected:expr) => {
            let cmd_block =
                ApplyExecCmdBlock::<PeaceTestError, ParamsKeysUnknown, $states_ts>::new();

            let outcome_type_names = cmd_block.outcome_type_names();

            assert_eq!($expected, outcome_type_names.as_slice());
        };
    }

    assert_outcome_type_names!(
        Ensured,
        &["States<Previous>", "States<Ensured>", "States<Goal>"]
    );
    assert_outcome_type_names!(
        EnsuredDry,
        &["States<Previous>", "States<EnsuredDry>", "States<Goal>"]
    );
    assert_outcome_type_names!(
        Cleaned,
        &["States<Previous>", "States<Cleaned>", "States<Clean>"]
    );
    assert_outcome_type_names!(
        CleanedDry,
        &["States<Previous>", "States<CleanedDry>", "States<Clean>"]
    );
}
