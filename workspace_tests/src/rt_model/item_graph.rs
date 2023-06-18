use peace::{
    cfg::{item_id, ItemId},
    rt_model::{fn_graph::FnGraph, ItemBoxed, ItemGraph, ItemGraphBuilder},
};

use crate::{PeaceTestError, VecCopyItem};

#[test]
fn clone() {
    let item_graph = ItemGraphBuilder::<PeaceTestError>::new().build();

    let _item_graph = Clone::clone(&item_graph);
}

#[test]
fn debug() {
    let item_graph = ItemGraphBuilder::<PeaceTestError>::new().build();

    assert!(format!("{item_graph:?}").starts_with("ItemGraph"));
}

#[test]
fn partial_eq() {
    let item_graph_0 = {
        let mut item_graph_builder = ItemGraphBuilder::<PeaceTestError>::new();
        item_graph_builder.add_fn(VecCopyItem::default().into());
        item_graph_builder.build()
    };
    let item_graph_1 = {
        let mut item_graph_builder = ItemGraphBuilder::<PeaceTestError>::new();
        item_graph_builder.add_fn(VecCopyItem::default().into());
        item_graph_builder.build()
    };
    let item_graph_2 = {
        let mut item_graph_builder = ItemGraphBuilder::<PeaceTestError>::new();
        item_graph_builder.add_fn(VecCopyItem::new(item_id!("rara")).into());
        item_graph_builder.build()
    };

    assert_eq!(item_graph_0, item_graph_1);
    assert_ne!(item_graph_0, item_graph_2);
}

#[test]
fn into_inner() {
    let _fn_graph = ItemGraphBuilder::<PeaceTestError>::new()
        .build()
        .into_inner();
}

#[test]
fn deref() {
    let item_graph = ItemGraphBuilder::<PeaceTestError>::new().build();

    let _fn_graph: &FnGraph<ItemBoxed<PeaceTestError>> = &item_graph;
}

#[test]
fn deref_mut() {
    let mut item_graph = ItemGraphBuilder::<PeaceTestError>::new().build();

    let _fn_graph: &mut FnGraph<ItemBoxed<PeaceTestError>> = &mut item_graph;
}

#[test]
fn from() {
    let fn_graph = FnGraph::<ItemBoxed<PeaceTestError>>::new();
    let _item_graph = ItemGraph::<PeaceTestError>::from(fn_graph);
}
