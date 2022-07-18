use peace::rt_model::{FullSpecBoxed, FullSpecRt};

use crate::{VecCopyError, VecCopyFullSpec};

#[test]
fn deref_to_dyn_full_spec_rt() {
    let full_spec_boxed: FullSpecBoxed<VecCopyError> = VecCopyFullSpec.into();
    let full_spec_rt: &dyn FullSpecRt<_> = &*full_spec_boxed;

    assert_eq!(
        format!("{:?}", VecCopyFullSpec),
        format!("{:?}", full_spec_rt)
    );
}

#[test]
fn deref_mut_to_dyn_full_spec_rt() {
    let full_spec_boxed: FullSpecBoxed<VecCopyError> = VecCopyFullSpec.into();
    let full_spec_rt: &dyn FullSpecRt<_> = &*full_spec_boxed;

    assert_eq!(
        format!("{:?}", VecCopyFullSpec),
        format!("{:?}", full_spec_rt)
    );
}
