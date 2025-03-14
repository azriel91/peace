use std::{fmt, sync::Arc};

use leptos::{
    children::ToChildren,
    prelude::{AnyView, IntoAny, Render},
    IntoView,
};

/// Allows a consumer to pass in the view fragment for a
/// [`leptos_router::Route`].
///
/// # Design
///
/// In `leptos 0.6`, `leptos::ChildrenFn` is an alias for `Rc<_>`, so it cannot
/// be passed to `leptos_axum::Router::leptos_routes`'s `app_fn` which requires
/// `app_fn` to be `Clone`, so we need to create our own `ChildrenFn` which is
/// `Clone`.
///
/// When we migrate to `leptos 0.7`, `ChildrenFn` is an alias for `Arc<_>` so we
/// can use it directly.
#[derive(Clone)]
pub struct ChildrenFn(Arc<dyn Fn() -> AnyView + Send + Sync>);

impl ChildrenFn {
    /// Returns a new `ChildrenFn`;
    pub fn new<F, IV>(f: F) -> Self
    where
        F: Fn() -> IV + Send + Sync + 'static,
        IV: IntoView + 'static,
        <IV as Render>::State: 'static,
    {
        Self(Arc::new(move || f().into_view().into_any()))
    }

    /// Returns the underlying function.
    pub fn into_inner(self) -> Arc<dyn Fn() -> AnyView + Send + Sync> {
        self.0
    }

    /// Calls the inner function to render the view.
    pub fn call(&self) -> AnyView {
        (self.0)()
    }
}

impl fmt::Debug for ChildrenFn {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("ChildrenFn")
            .field(&"Arc<dyn Fn() -> AnyView + Send + Sync>")
            .finish()
    }
}

impl<F> ToChildren<F> for ChildrenFn
where
    F: Fn() -> AnyView + 'static + Send + Sync,
{
    #[inline]
    fn to_children(f: F) -> Self {
        ChildrenFn(Arc::new(f))
    }
}
