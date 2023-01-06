use std::any::TypeId;

use peace::{
    data::{DataAccessDyn, TypeIds},
    rt_model::{ItemSpecBoxed, ItemSpecRt},
};

use crate::{vec_copy_item_spec::VecB, VecCopyError, VecCopyItemSpec};

#[test]
fn deref_to_dyn_item_spec_rt() {
    let item_spec_boxed: ItemSpecBoxed<VecCopyError> = VecCopyItemSpec.into();
    let item_spec_rt: &dyn ItemSpecRt<_> = &*item_spec_boxed;

    assert_eq!(format!("{VecCopyItemSpec:?}"), format!("{item_spec_rt:?}"));
}

#[test]
fn deref_mut_to_dyn_item_spec_rt() {
    let mut item_spec_boxed: ItemSpecBoxed<VecCopyError> = VecCopyItemSpec.into();
    let item_spec_rt: &mut dyn ItemSpecRt<_> = &mut *item_spec_boxed;

    assert_eq!(format!("{VecCopyItemSpec:?}"), format!("{item_spec_rt:?}"));
}

#[test]
fn data_access_dyn_borrows() {
    let type_ids = TypeIds::new();

    let item_spec_boxed: ItemSpecBoxed<VecCopyError> = VecCopyItemSpec.into();

    assert_eq!(
        type_ids,
        <ItemSpecBoxed<VecCopyError> as DataAccessDyn>::borrows(&item_spec_boxed)
    );
}

#[test]
fn data_access_dyn_borrow_muts() {
    let mut type_ids = TypeIds::new();
    type_ids.push(TypeId::of::<VecB>());

    let item_spec_boxed: ItemSpecBoxed<VecCopyError> = VecCopyItemSpec.into();

    assert_eq!(
        type_ids,
        <ItemSpecBoxed<VecCopyError> as DataAccessDyn>::borrow_muts(&item_spec_boxed)
    );
}

#[test]
fn debug() {
    let item_spec_boxed: ItemSpecBoxed<VecCopyError> = VecCopyItemSpec.into();

    assert_eq!(
        "ItemSpecBoxed(VecCopyItemSpec)",
        format!("{item_spec_boxed:?}")
    );
}
