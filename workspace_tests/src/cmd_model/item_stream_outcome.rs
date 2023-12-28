use peace::{
    cfg::{item_id, ItemId},
    cmd_model::ItemStreamOutcome,
};

#[test]
fn replace() {
    let item_stream_outcome = ItemStreamOutcome::finished_with(1u16, vec![item_id!("mock")]);

    let (item_stream_outcome, n) = item_stream_outcome.replace(2u32);

    assert_eq!(1u16, n);
    assert_eq!(
        ItemStreamOutcome::finished_with(2u32, vec![item_id!("mock")]),
        item_stream_outcome
    );
}

#[test]
fn replace_with() {
    let item_stream_outcome =
        ItemStreamOutcome::finished_with((1u16, "value_to_extract"), vec![item_id!("mock")]);

    let (item_stream_outcome, value) = item_stream_outcome.replace_with(|(n, value)| (n, value));

    assert_eq!("value_to_extract", value);
    assert_eq!(
        ItemStreamOutcome::finished_with(1u16, vec![item_id!("mock")]),
        item_stream_outcome
    );
}
