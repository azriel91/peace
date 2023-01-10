use peace::rt_model::{fn_graph::FnGraphBuilder, Error, ItemSpecBoxed, ItemSpecGraphBuilder};

#[test]
fn debug() {
    let builder = ItemSpecGraphBuilder::<Error>::new();

    assert!(format!("{builder:?}").starts_with("ItemSpecGraphBuilder"));
}

#[test]
fn into_inner() {
    let _fn_graph_builder = ItemSpecGraphBuilder::<Error>::new().into_inner();
}

#[test]
fn deref() {
    let builder = ItemSpecGraphBuilder::<Error>::new();

    let _fn_graph_builder: &FnGraphBuilder<ItemSpecBoxed<Error>> = &builder;
}

#[test]
fn deref_mut() {
    let mut builder = ItemSpecGraphBuilder::<Error>::new();

    let _fn_graph_builder: &mut FnGraphBuilder<ItemSpecBoxed<Error>> = &mut builder;
}

#[test]
fn from() {
    let fn_graph_builder = FnGraphBuilder::<ItemSpecBoxed<Error>>::new();
    let _builder = ItemSpecGraphBuilder::<Error>::from(fn_graph_builder);
}
