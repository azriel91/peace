use std::{any::TypeId, marker::PhantomData};

use peace::data::{
    accessors::{R, W},
    Data, DataAccess, TypeIds,
};

#[test]
fn data_named_fields_borrows() {
    let mut type_ids_expected = TypeIds::new();
    type_ids_expected.push(TypeId::of::<A>());
    type_ids_expected.push(TypeId::of::<B>());

    let type_ids_actual = <DataNamedFields<'_> as DataAccess>::borrows();

    assert_eq!(type_ids_expected, type_ids_actual);
}

#[test]
fn data_unnamed_fields_borrows() {
    let mut type_ids_expected = TypeIds::new();
    type_ids_expected.push(TypeId::of::<A>());
    type_ids_expected.push(TypeId::of::<B>());

    let type_ids_actual = <DataUnnamedFields<'_> as DataAccess>::borrows();

    assert_eq!(type_ids_expected, type_ids_actual);
}

#[test]
fn data_mix_fields_borrows() {
    let mut type_ids_expected = TypeIds::new();
    type_ids_expected.push(TypeId::of::<A>());

    let type_ids_actual = <DataMixFields<'_> as DataAccess>::borrows();

    assert_eq!(type_ids_expected, type_ids_actual);
}

#[test]
fn r_borrows() {
    let mut type_ids_expected = TypeIds::new();
    type_ids_expected.push(TypeId::of::<A>());

    let type_ids_actual = <R<'_, A> as DataAccess>::borrows();

    assert_eq!(type_ids_expected, type_ids_actual);
}

#[test]
fn w_borrows() {
    let type_ids_expected = TypeIds::new();

    let type_ids_actual = <W<'_, A> as DataAccess>::borrows();

    assert_eq!(type_ids_expected, type_ids_actual);
}

#[test]
fn data_named_fields_borrow_muts() {
    let type_ids_expected = TypeIds::new();

    let type_ids_actual = <DataNamedFields<'_> as DataAccess>::borrow_muts();

    assert_eq!(type_ids_expected, type_ids_actual);
}

#[test]
fn data_unnamed_fields_borrow_muts() {
    let type_ids_expected = TypeIds::new();

    let type_ids_actual = <DataUnnamedFields<'_> as DataAccess>::borrow_muts();

    assert_eq!(type_ids_expected, type_ids_actual);
}

#[test]
fn data_mut_fields_borrow_muts() {
    let mut type_ids_expected = TypeIds::new();
    type_ids_expected.push(TypeId::of::<A>());
    type_ids_expected.push(TypeId::of::<B>());

    let type_ids_actual = <DataMutFields<'_> as DataAccess>::borrow_muts();

    assert_eq!(type_ids_expected, type_ids_actual);
}

#[test]
fn data_mix_fields_borrow_muts() {
    let mut type_ids_expected = TypeIds::new();
    type_ids_expected.push(TypeId::of::<B>());

    let type_ids_actual = <DataMixFields<'_> as DataAccess>::borrow_muts();

    assert_eq!(type_ids_expected, type_ids_actual);
}

#[test]
fn r_borrow_muts() {
    let type_ids_expected = TypeIds::new();

    let type_ids_actual = <R<'_, A> as DataAccess>::borrow_muts();

    assert_eq!(type_ids_expected, type_ids_actual);
}

#[test]
fn w_borrow_muts() {
    let mut type_ids_expected = TypeIds::new();
    type_ids_expected.push(TypeId::of::<A>());

    let type_ids_actual = <W<'_, A> as DataAccess>::borrow_muts();

    assert_eq!(type_ids_expected, type_ids_actual);
}

#[derive(Debug, Data)]
struct DataNamedFields<'exec> {
    a_imm: R<'exec, A>,
    b_imm: R<'exec, B>,
}

#[derive(Debug, Data)]
struct DataUnnamedFields<'exec>(R<'exec, A>, R<'exec, B>);

#[derive(Debug, Data)]
struct DataMutFields<'exec> {
    a_mut: W<'exec, A>,
    b_mut: W<'exec, B>,
}

#[derive(Debug, Data)]
struct DataMixFields<'exec> {
    a_imm: R<'exec, A>,
    phantom_0: PhantomData<()>,
    b_mut: W<'exec, B>,
    phantom_1: PhantomData<()>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct A(u32);

#[derive(Clone, Copy, Debug, PartialEq)]
struct B(u32);
