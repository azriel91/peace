use std::any::TypeId;

use peace::{
    data::{DataAccessDyn, TypeIds},
    rt_model::{ItemSpecBoxed, ItemSpecRt},
};

use crate::{
    vec_copy_item_spec::{VecA, VecB},
    VecCopyError, VecCopyItemSpec,
};

#[test]
fn deref_to_dyn_item_spec_rt() {
    let vec_copy_item_spec = VecCopyItemSpec::default();
    let item_spec_boxed: ItemSpecBoxed<VecCopyError> = vec_copy_item_spec.clone().into();
    let item_spec_rt: &dyn ItemSpecRt<_> = &*item_spec_boxed;

    assert_eq!(
        format!("{vec_copy_item_spec:?}"),
        format!("{item_spec_rt:?}")
    );
}

#[test]
fn deref_mut_to_dyn_item_spec_rt() {
    let vec_copy_item_spec = VecCopyItemSpec::default();
    let mut item_spec_boxed: ItemSpecBoxed<VecCopyError> = vec_copy_item_spec.clone().into();
    let item_spec_rt: &mut dyn ItemSpecRt<_> = &mut *item_spec_boxed;

    assert_eq!(
        format!("{vec_copy_item_spec:?}"),
        format!("{item_spec_rt:?}")
    );
}

#[test]
fn data_access_dyn_borrows() {
    let mut type_ids = TypeIds::new();
    type_ids.push(TypeId::of::<VecA>());

    let vec_copy_item_spec = VecCopyItemSpec::default();
    let item_spec_boxed: ItemSpecBoxed<VecCopyError> = vec_copy_item_spec.into();

    assert_eq!(
        type_ids,
        <ItemSpecBoxed<VecCopyError> as DataAccessDyn>::borrows(&item_spec_boxed)
    );
}

#[test]
fn data_access_dyn_borrow_muts() {
    let mut type_ids = TypeIds::new();
    type_ids.push(TypeId::of::<VecB>());

    let vec_copy_item_spec = VecCopyItemSpec::default();
    let item_spec_boxed: ItemSpecBoxed<VecCopyError> = vec_copy_item_spec.into();

    assert_eq!(
        type_ids,
        <ItemSpecBoxed<VecCopyError> as DataAccessDyn>::borrow_muts(&item_spec_boxed)
    );
}

#[test]
fn debug() {
    let vec_copy_item_spec = VecCopyItemSpec::default();
    let item_spec_boxed: ItemSpecBoxed<VecCopyError> = vec_copy_item_spec.into();

    assert_eq!(
        "ItemSpecBoxed(VecCopyItemSpec { id: ItemSpecId(\"vec_copy\") })",
        format!("{item_spec_boxed:?}")
    );
}
