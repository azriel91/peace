use std::{
    any::TypeId,
    fmt::{Debug, Display},
    marker::PhantomData,
};

use peace_core::ItemId;
use peace_data::{
    fn_graph::{
        resman::{BorrowFail, Ref},
        DataAccess, DataAccessDyn, Resources, TypeIds,
    },
    Data,
};
use peace_resources::{states::StatesSaved, type_reg::untagged::DataType};
use serde::Serialize;

/// The previously saved `T` state, if any.
#[derive(Debug)]
pub struct Saved<'borrow, T> {
    /// ID of the item the state should be retrieved for.
    item_id: &'borrow ItemId,
    /// The borrowed `StatesSaved`.
    states_saved: Option<Ref<'borrow, StatesSaved>>,
    /// Marker.
    marker: PhantomData<T>,
}

impl<'borrow, T> Saved<'borrow, T>
where
    T: Clone + Debug + DataType + Display + Serialize + Send + Sync + 'static,
{
    pub fn get(&'borrow self) -> Option<&'borrow T> {
        self.states_saved
            .as_ref()
            .and_then(|states_saved| states_saved.get(self.item_id))
    }
}

impl<'borrow, T> Data<'borrow> for Saved<'borrow, T>
where
    T: Debug + Send + Sync + 'static,
{
    fn borrow(item_id: &'borrow ItemId, resources: &'borrow Resources) -> Self {
        let states_saved = resources
            .try_borrow::<StatesSaved>()
            .map_err(|borrow_fail| match borrow_fail {
                e @ BorrowFail::ValueNotFound => e,
                BorrowFail::BorrowConflictImm | BorrowFail::BorrowConflictMut => {
                    panic!("Encountered {borrow_fail:?}")
                }
            })
            .ok();

        Self {
            item_id,
            states_saved,
            marker: PhantomData,
        }
    }
}

impl<'borrow, T> DataAccess for Saved<'borrow, T> {
    fn borrows() -> TypeIds
    where
        Self: Sized,
    {
        let mut type_ids = TypeIds::new();
        type_ids.push(TypeId::of::<StatesSaved>());
        type_ids
    }

    fn borrow_muts() -> TypeIds
    where
        Self: Sized,
    {
        TypeIds::new()
    }
}

impl<'borrow, T> DataAccessDyn for Saved<'borrow, T> {
    fn borrows(&self) -> TypeIds
    where
        Self: Sized,
    {
        let mut type_ids = TypeIds::new();
        type_ids.push(TypeId::of::<StatesSaved>());
        type_ids
    }

    fn borrow_muts(&self) -> TypeIds
    where
        Self: Sized,
    {
        TypeIds::new()
    }
}
