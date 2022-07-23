//! Contains type-erased `FullSpec` types and traits.
//!
//! Types and traits in this module don't reference any associated types from
//! the `FullSpec`, allowing them to be passed around as common types at compile
//! time.
//!
//! For the logic that is aware of the type parameters, see the
//! [`full_spec_wrapper`] module and [`FullSpecWrapper`] type.
//!
//! [`full_spec_wrapper`]: crate::full_spec_wrapper
//! [`FullSpecWrapper`]: crate::FullSpecWrapper

use std::{
    fmt::Debug,
    ops::{Deref, DerefMut},
};

use fn_graph::{DataAccessDyn, TypeIds};
use peace_cfg::{FnSpec, FullSpec, State};
use peace_diff::Diff;
use serde::{de::DeserializeOwned, Serialize};

use crate::{FullSpecRt, FullSpecWrapper};

/// Holds a type-erased `FullSpecWrapper` in a `Box`.
///
/// # Type Parameters
///
/// * `E`: Application specific error type.
#[derive(Debug)]
pub struct FullSpecBoxed<E>(Box<dyn FullSpecRt<E>>)
where
    E: std::error::Error;

impl<E> Deref for FullSpecBoxed<E>
where
    E: std::error::Error,
{
    type Target = dyn FullSpecRt<E>;

    // https://github.com/rust-lang/rust-clippy/issues/9101
    #[allow(clippy::explicit_auto_deref)]
    fn deref(&self) -> &Self::Target {
        &*self.0
    }
}

impl<E> DerefMut for FullSpecBoxed<E>
where
    E: std::error::Error,
{
    // https://github.com/rust-lang/rust-clippy/issues/9101
    #[allow(clippy::explicit_auto_deref)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut *self.0
    }
}

impl<
    FS,
    E,
    StateLogical,
    StatePhysical,
    StatusFnSpec,
    StatusDesiredFnSpec,
    EnsureOpSpec,
    CleanOpSpec,
> From<FS> for FullSpecBoxed<E>
where
    FS: Debug
        + FullSpec<
            Error = E,
            StateLogical = StateLogical,
            StatePhysical = StatePhysical,
            StatusFnSpec = StatusFnSpec,
            StatusDesiredFnSpec = StatusDesiredFnSpec,
            EnsureOpSpec = EnsureOpSpec,
            CleanOpSpec = CleanOpSpec,
        > + Send
        + Sync
        + 'static,
    E: Debug + Send + Sync + std::error::Error + 'static,
    StateLogical: Clone + Debug + Diff + Serialize + DeserializeOwned + Send + Sync + 'static,
    StatePhysical: Clone + Debug + Serialize + DeserializeOwned + Send + Sync + 'static,
    StatusFnSpec: Debug
        + FnSpec<Error = E, Output = State<StateLogical, StatePhysical>>
        + Send
        + Sync
        + 'static,
    StatusDesiredFnSpec: Debug + FnSpec<Error = E, Output = StateLogical> + Send + Sync + 'static,
    EnsureOpSpec: Debug
        + peace_cfg::EnsureOpSpec<
            Error = E,
            StateLogical = StateLogical,
            StatePhysical = StatePhysical,
        > + Send
        + Sync
        + 'static,
    CleanOpSpec: Debug
        + peace_cfg::CleanOpSpec<
            Error = E,
            StateLogical = StateLogical,
            StatePhysical = StatePhysical,
        > + Send
        + Sync
        + 'static,
{
    fn from(full_spec: FS) -> Self {
        Self(Box::new(FullSpecWrapper::from(full_spec)))
    }
}

impl<E> DataAccessDyn for FullSpecBoxed<E>
where
    E: std::error::Error,
{
    fn borrows(&self) -> TypeIds {
        DataAccessDyn::borrows(self.0.as_ref())
    }

    fn borrow_muts(&self) -> TypeIds {
        DataAccessDyn::borrow_muts(self.0.as_ref())
    }
}
