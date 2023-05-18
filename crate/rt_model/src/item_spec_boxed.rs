//! Contains type-erased `ItemSpec` types and traits.
//!
//! Types and traits in this module don't reference any associated types from
//! the `ItemSpec`, allowing them to be passed around as common types at compile
//! time.
//!
//! For the logic that is aware of the type parameters, see the
//! [`item_spec_wrapper`] module and [`ItemSpecWrapper`] type.
//!
//! [`item_spec_wrapper`]: crate::item_spec_wrapper
//! [`ItemSpecWrapper`]: crate::ItemSpecWrapper

use std::{
    fmt::Debug,
    ops::{Deref, DerefMut},
};

use fn_graph::{DataAccessDyn, TypeIds};
use peace_cfg::ItemSpec;
use peace_params::Params;

use crate::{ItemSpecRt, ItemSpecWrapper};

/// Holds a type-erased `ItemSpecWrapper` in a `Box`.
///
/// # Type Parameters
///
/// * `E`: Application specific error type.
///
///     Notably, `E` here should be the application's error type, which is not
///     necessarily the item spec's error type (unless you have only one item
///     spec in the application).
#[derive(Debug)]
pub struct ItemSpecBoxed<E>(Box<dyn ItemSpecRt<E>>);

impl<E> Clone for ItemSpecBoxed<E> {
    fn clone(&self) -> Self {
        Self(dyn_clone::clone_box(self.0.as_ref()))
    }
}

impl<E> Deref for ItemSpecBoxed<E> {
    type Target = dyn ItemSpecRt<E>;

    fn deref(&self) -> &Self::Target {
        &*self.0
    }
}

impl<E> DerefMut for ItemSpecBoxed<E> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut *self.0
    }
}

impl<IS, E> From<IS> for ItemSpecBoxed<E>
where
    IS: Clone + Debug + ItemSpec + Send + Sync + 'static,
    <IS as ItemSpec>::Error: Send + Sync,
    E: Debug
        + Send
        + Sync
        + std::error::Error
        + From<<IS as ItemSpec>::Error>
        + From<crate::Error>
        + 'static,
    for<'params> <IS as ItemSpec>::Params<'params>:
        TryFrom<<<IS as ItemSpec>::Params<'params> as Params>::Partial>,
    for<'params> <IS::Params<'params> as Params>::Partial: From<IS::Params<'params>>,
{
    fn from(item_spec: IS) -> Self {
        Self(Box::new(ItemSpecWrapper::from(item_spec)))
    }
}

impl<E> DataAccessDyn for ItemSpecBoxed<E> {
    fn borrows(&self) -> TypeIds {
        DataAccessDyn::borrows(self.0.as_ref())
    }

    fn borrow_muts(&self) -> TypeIds {
        DataAccessDyn::borrow_muts(self.0.as_ref())
    }
}
