use peace_data::fn_graph::W;

/// Type alias for `W<'_, Option<T>>`.
pub type WOpt<'borrow, T> = W<'borrow, Option<T>>;
