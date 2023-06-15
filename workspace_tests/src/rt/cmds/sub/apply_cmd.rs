use peace::{
    resources::states::ts::{Current, CurrentStored},
    rt::cmds::sub::ApplyCmd,
};

use crate::{NoOpOutput, VecCopyError};

#[test]
fn debug() {
    let debug_str = format!(
        "{:?}",
        ApplyCmd::<VecCopyError, NoOpOutput, (), Current, CurrentStored>::default()
    );
    assert_eq!(
        "ApplyCmd(PhantomData<(\
            workspace_tests::vec_copy_item::VecCopyError, \
            workspace_tests::no_op_output::NoOpOutput, \
            (), \
            peace_resources::states::ts::Current, \
            peace_resources::states::ts::CurrentStored\
        )>)",
        debug_str,
    );
}
