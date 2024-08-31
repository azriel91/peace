//! Contains type-erased `Item` types and traits.
//!
//! Types and traits in this module don't reference any associated types from
//! the `Item`, allowing them to be passed around as common types at compile
//! time.
//!
//! For the logic that is aware of the type parameters, see the
//! [`item_wrapper`] module and [`ItemWrapper`] type.
//!
//! [`item_wrapper`]: crate::item_wrapper
//! [`ItemWrapper`]: crate::ItemWrapper

use std::{
    fmt::Debug,
    ops::{Deref, DerefMut},
};

use peace_cfg::Item;
use peace_data::fn_graph::{DataAccessDyn, TypeIds};
use peace_params::{Params, ParamsMergeExt};

use crate::{ItemRt, ItemWrapper};

/// Holds a type-erased `ItemWrapper` in a `Box`.
///
/// # Type Parameters
///
/// * `E`: Application specific error type.
///
///     Notably, `E` here should be the application's error type, which is not
///     necessarily the item's error type (unless you have only one item
///     spec in the application).
#[derive(Debug)]
pub struct ItemBoxed<E>(Box<dyn ItemRt<E>>);

impl<E> Clone for ItemBoxed<E> {
    fn clone(&self) -> Self {
        Self(dyn_clone::clone_box(self.0.as_ref()))
    }
}

impl<E> Deref for ItemBoxed<E> {
    type Target = dyn ItemRt<E>;

    fn deref(&self) -> &Self::Target {
        &*self.0
    }
}

impl<E> DerefMut for ItemBoxed<E> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut *self.0
    }
}

impl<E> PartialEq for ItemBoxed<E>
where
    E: 'static,
{
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&*other.0)
    }
}

impl<E> Eq for ItemBoxed<E> where E: 'static {}

impl<I, E> From<I> for ItemBoxed<E>
where
    I: Clone + Debug + Item + Send + Sync + 'static,
    <I as Item>::Error: Send + Sync,
    E: Debug
        + Send
        + Sync
        + std::error::Error
        + From<<I as Item>::Error>
        + From<crate::Error>
        + 'static,
    for<'params> <I as Item>::Params<'params>:
        ParamsMergeExt + TryFrom<<<I as Item>::Params<'params> as Params>::Partial>,
    for<'params> <<I as Item>::Params<'params> as Params>::Partial: From<
        <<I as Item>::Params<'params> as TryFrom<
            <<I as Item>::Params<'params> as Params>::Partial,
        >>::Error,
    >,
    for<'params> <I::Params<'params> as Params>::Partial: From<I::Params<'params>>,
{
    fn from(item: I) -> Self {
        Self(Box::new(ItemWrapper::from(item)))
    }
}

impl<E> DataAccessDyn for ItemBoxed<E> {
    fn borrows(&self) -> TypeIds {
        DataAccessDyn::borrows(self.0.as_ref())
    }

    fn borrow_muts(&self) -> TypeIds {
        DataAccessDyn::borrow_muts(self.0.as_ref())
    }
}
