use peace::rt_model::{ItemSpecBoxed, ItemSpecRt};

use crate::{VecCopyError, VecCopyItemSpec};

#[test]
fn deref_to_dyn_item_spec_rt() {
    let item_spec_boxed: ItemSpecBoxed<VecCopyError> = VecCopyItemSpec.into();
    let item_spec_rt: &dyn ItemSpecRt<_> = &*item_spec_boxed;

    assert_eq!(
        format!("{:?}", VecCopyItemSpec),
        format!("{:?}", item_spec_rt)
    );
}

#[test]
fn deref_mut_to_dyn_item_spec_rt() {
    let item_spec_boxed: ItemSpecBoxed<VecCopyError> = VecCopyItemSpec.into();
    let item_spec_rt: &dyn ItemSpecRt<_> = &*item_spec_boxed;

    assert_eq!(
        format!("{:?}", VecCopyItemSpec),
        format!("{:?}", item_spec_rt)
    );
}
