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
    fmt::{self, Debug},
    ops::{Deref, DerefMut},
};

use fn_graph::{DataAccessDyn, TypeIds};
use peace_cfg::{ItemSpec, State, TryFnSpec};
use serde::{de::DeserializeOwned, Serialize};

use crate::{ItemSpecRt, ItemSpecWrapper};

/// Holds a type-erased `ItemSpecWrapper` in a `Box`.
///
/// # Type Parameters
///
/// * `E`: Application specific error type.
#[derive(Debug)]
pub struct ItemSpecBoxed<E>(Box<dyn ItemSpecRt<E>>);

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

impl<
    IS,
    E,
    StateLogical,
    StatePhysical,
    StateDiff,
    StateCurrentFnSpec,
    StateDesiredFnSpec,
    StateDiffFnSpec,
    EnsureOpSpec,
    CleanOpSpec,
> From<IS> for ItemSpecBoxed<E>
where
    IS: Debug
        + ItemSpec<
            StateLogical = StateLogical,
            StatePhysical = StatePhysical,
            StateDiff = StateDiff,
            StateCurrentFnSpec = StateCurrentFnSpec,
            StateDesiredFnSpec = StateDesiredFnSpec,
            StateDiffFnSpec = StateDiffFnSpec,
            EnsureOpSpec = EnsureOpSpec,
            CleanOpSpec = CleanOpSpec,
        > + Send
        + Sync
        + 'static,
    <IS as ItemSpec>::Error: Send + Sync,
    E: Debug
        + Send
        + Sync
        + std::error::Error
        + From<<IS as ItemSpec>::Error>
        + From<crate::Error>
        + 'static,
    StateLogical:
        Clone + Debug + fmt::Display + Serialize + DeserializeOwned + Send + Sync + 'static,
    StatePhysical:
        Clone + Debug + fmt::Display + Serialize + DeserializeOwned + Send + Sync + 'static,
    StateDiff: Clone + Debug + fmt::Display + Serialize + DeserializeOwned + Send + Sync + 'static,
    StateCurrentFnSpec: Debug
        + TryFnSpec<Error = <IS as ItemSpec>::Error, Output = State<StateLogical, StatePhysical>>
        + Send
        + Sync
        + 'static,
    StateDesiredFnSpec: Debug
        + TryFnSpec<Error = <IS as ItemSpec>::Error, Output = State<StateLogical, StatePhysical>>
        + Send
        + Sync
        + 'static,
    StateDiffFnSpec: Debug
        + peace_cfg::StateDiffFnSpec<
            Error = <IS as ItemSpec>::Error,
            StateLogical = StateLogical,
            StatePhysical = StatePhysical,
            StateDiff = StateDiff,
        > + Send
        + Sync
        + 'static,
    EnsureOpSpec: Debug
        + peace_cfg::EnsureOpSpec<
            Error = <IS as ItemSpec>::Error,
            StateLogical = StateLogical,
            StatePhysical = StatePhysical,
            StateDiff = StateDiff,
        > + Send
        + Sync
        + 'static,
    CleanOpSpec: Debug
        + peace_cfg::CleanOpSpec<
            Error = <IS as ItemSpec>::Error,
            StateLogical = StateLogical,
            StatePhysical = StatePhysical,
        > + Send
        + Sync
        + 'static,
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
