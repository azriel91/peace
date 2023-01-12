use peace::rt_model::{
    fn_graph::FnGraph, Error, ItemSpecBoxed, ItemSpecGraph, ItemSpecGraphBuilder,
};

#[test]
fn debug() {
    let item_spec_graph = ItemSpecGraphBuilder::<Error>::new().build();

    assert!(format!("{item_spec_graph:?}").starts_with("ItemSpecGraph"));
}

#[test]
fn into_inner() {
    let _fn_graph = ItemSpecGraphBuilder::<Error>::new().build().into_inner();
}

#[test]
fn deref() {
    let item_spec_graph = ItemSpecGraphBuilder::<Error>::new().build();

    let _fn_graph: &FnGraph<ItemSpecBoxed<Error>> = &item_spec_graph;
}

#[test]
fn deref_mut() {
    let mut item_spec_graph = ItemSpecGraphBuilder::<Error>::new().build();

    let _fn_graph: &mut FnGraph<ItemSpecBoxed<Error>> = &mut item_spec_graph;
}

#[test]
fn from() {
    let fn_graph = FnGraph::<ItemSpecBoxed<Error>>::new();
    let _item_spec_graph = ItemSpecGraph::<Error>::from(fn_graph);
}
