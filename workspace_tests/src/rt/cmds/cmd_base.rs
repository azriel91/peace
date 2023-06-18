use peace::rt::cmds::CmdBase;

use crate::{NoOpOutput, PeaceTestError};

#[test]
fn debug() {
    let debug_str = format!("{:?}", CmdBase::<PeaceTestError, NoOpOutput, ()>::default());
    assert_eq!(
        r#"CmdBase(PhantomData<(workspace_tests::peace_test_error::PeaceTestError, workspace_tests::no_op_output::NoOpOutput, ())>)"#,
        debug_str,
    );
}
