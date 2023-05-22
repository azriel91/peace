use peace::rt_model::{fn_graph::FnGraph, Error, ItemBoxed, ItemGraph, ItemGraphBuilder};

#[test]
fn debug() {
    let item_graph = ItemGraphBuilder::<Error>::new().build();

    assert!(format!("{item_graph:?}").starts_with("ItemGraph"));
}

#[test]
fn into_inner() {
    let _fn_graph = ItemGraphBuilder::<Error>::new().build().into_inner();
}

#[test]
fn deref() {
    let item_graph = ItemGraphBuilder::<Error>::new().build();

    let _fn_graph: &FnGraph<ItemBoxed<Error>> = &item_graph;
}

#[test]
fn deref_mut() {
    let mut item_graph = ItemGraphBuilder::<Error>::new().build();

    let _fn_graph: &mut FnGraph<ItemBoxed<Error>> = &mut item_graph;
}

#[test]
fn from() {
    let fn_graph = FnGraph::<ItemBoxed<Error>>::new();
    let _item_graph = ItemGraph::<Error>::from(fn_graph);
}
