//! Contains type-erased `Step` types and traits.
//!
//! Types and traits in this module don't reference any associated types from
//! the `Step`, allowing them to be passed around as common types at compile
//! time.
//!
//! For the logic that is aware of the type parameters, see the
//! [`step_wrapper`] module and [`StepWrapper`] type.
//!
//! [`step_wrapper`]: crate::step_wrapper
//! [`StepWrapper`]: crate::StepWrapper

use std::{
    fmt::Debug,
    ops::{Deref, DerefMut},
};

use peace_cfg::Step;
use peace_data::fn_graph::{DataAccessDyn, TypeIds};
use peace_params::Params;

use crate::{StepRt, StepWrapper};

/// Holds a type-erased `StepWrapper` in a `Box`.
///
/// # Type Parameters
///
/// * `E`: Application specific error type.
///
///     Notably, `E` here should be the application's error type, which is not
///     necessarily the step's error type (unless you have only one step
///     spec in the application).
#[derive(Debug)]
pub struct StepBoxed<E>(Box<dyn StepRt<E>>);

impl<E> Clone for StepBoxed<E> {
    fn clone(&self) -> Self {
        Self(dyn_clone::clone_box(self.0.as_ref()))
    }
}

impl<E> Deref for StepBoxed<E> {
    type Target = dyn StepRt<E>;

    fn deref(&self) -> &Self::Target {
        &*self.0
    }
}

impl<E> DerefMut for StepBoxed<E> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut *self.0
    }
}

impl<E> PartialEq for StepBoxed<E>
where
    E: 'static,
{
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&*other.0)
    }
}

impl<E> Eq for StepBoxed<E> where E: 'static {}

impl<I, E> From<I> for StepBoxed<E>
where
    I: Clone + Debug + Step + Send + Sync + 'static,
    <I as Step>::Error: Send + Sync,
    E: Debug
        + Send
        + Sync
        + std::error::Error
        + From<<I as Step>::Error>
        + From<crate::Error>
        + 'static,
    for<'params> <I as Step>::Params<'params>:
        TryFrom<<<I as Step>::Params<'params> as Params>::Partial>,
    for<'params> <I::Params<'params> as Params>::Partial: From<I::Params<'params>>,
{
    fn from(step: I) -> Self {
        Self(Box::new(StepWrapper::from(step)))
    }
}

impl<E> DataAccessDyn for StepBoxed<E> {
    fn borrows(&self) -> TypeIds {
        DataAccessDyn::borrows(self.0.as_ref())
    }

    fn borrow_muts(&self) -> TypeIds {
        DataAccessDyn::borrow_muts(self.0.as_ref())
    }
}
