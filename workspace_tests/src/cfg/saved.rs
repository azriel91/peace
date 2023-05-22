use std::any::TypeId;

use peace::{
    cfg::{accessors::Saved, item_id, ItemId},
    data::{fn_graph::Resources, Data, DataAccess, DataAccessDyn, TypeIds},
    resources::{internal::StatesMut, states::StatesSaved},
};

const ITEM_SPEC_ID_TEST: &ItemId = &item_id!("item_id_test");
const ITEM_SPEC_ID_OTHER: &ItemId = &item_id!("item_id_other");

#[test]
fn retrieves_state_for_item() {
    let mut resources = Resources::new();
    let states_saved = {
        let mut states_mut = StatesMut::new();
        states_mut.insert(ITEM_SPEC_ID_TEST.clone(), 123u8);

        StatesSaved::from(states_mut)
    };
    resources.insert(states_saved);

    let saved = Saved::<'_, u8>::borrow(ITEM_SPEC_ID_TEST, &resources);

    assert_eq!(Some(123), saved.get().copied());
}

#[test]
fn does_not_retrieve_state_for_item_other() {
    let mut resources = Resources::new();
    let states_saved = {
        let mut states_mut = StatesMut::new();
        states_mut.insert(ITEM_SPEC_ID_OTHER.clone(), 123u8);

        StatesSaved::from(states_mut)
    };
    resources.insert(states_saved);

    let saved = Saved::<'_, u8>::borrow(ITEM_SPEC_ID_TEST, &resources);

    assert_eq!(None, saved.get().copied());
}

#[test]
fn data_access_borrows_returns_states_saved_type_id() {
    let mut type_ids = TypeIds::new();
    type_ids.push(TypeId::of::<StatesSaved>());

    assert_eq!(type_ids, <Saved::<'_, u8> as DataAccess>::borrows());
}

#[test]
fn data_access_borrow_muts_is_empty() {
    let type_ids = TypeIds::new();

    assert_eq!(type_ids, <Saved::<'_, u8> as DataAccess>::borrow_muts());
}

#[test]
fn data_access_dyn_borrows_returns_states_saved_type_id() {
    let mut resources = Resources::new();
    let states_saved = StatesSaved::from(StatesMut::new());
    resources.insert(states_saved);
    let saved = Saved::<'_, u8>::borrow(ITEM_SPEC_ID_TEST, &resources);

    let mut type_ids = TypeIds::new();
    type_ids.push(TypeId::of::<StatesSaved>());

    assert_eq!(
        type_ids,
        <Saved::<'_, u8> as DataAccessDyn>::borrows(&saved)
    );
}

#[test]
fn data_access_dyn_borrow_muts_is_empty() {
    let mut resources = Resources::new();
    let states_saved = StatesSaved::from(StatesMut::new());
    resources.insert(states_saved);
    let saved = Saved::<'_, u8>::borrow(ITEM_SPEC_ID_TEST, &resources);

    let type_ids = TypeIds::new();

    assert_eq!(
        type_ids,
        <Saved::<'_, u8> as DataAccessDyn>::borrow_muts(&saved)
    );
}

#[test]
fn debug() {
    let mut resources = Resources::new();
    let states_saved = {
        let mut states_mut = StatesMut::new();
        states_mut.insert(ITEM_SPEC_ID_TEST.clone(), 123u8);

        StatesSaved::from(states_mut)
    };
    resources.insert(states_saved);

    let saved = Saved::<'_, u8>::borrow(ITEM_SPEC_ID_TEST, &resources);
    assert_eq!(
        r#"Saved { item_id: ItemId("item_id_test"), states_saved: Some(Ref { inner: States({ItemId("item_id_test"): TypedValue { type: "u8", value: 123 }}, PhantomData<peace_resources::states::ts::Saved>) }), marker: PhantomData<u8> }"#,
        format!("{saved:?}")
    );

    let saved = Saved::<'_, u8>::borrow(ITEM_SPEC_ID_OTHER, &resources);
    assert_eq!(
        r#"Saved { item_id: ItemId("item_id_other"), states_saved: Some(Ref { inner: States({ItemId("item_id_test"): TypedValue { type: "u8", value: 123 }}, PhantomData<peace_resources::states::ts::Saved>) }), marker: PhantomData<u8> }"#,
        format!("{saved:?}")
    );
}
