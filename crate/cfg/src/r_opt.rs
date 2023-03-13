use peace_data::fn_graph::R;

/// Type alias for `R<'_, Option<T>>`.
pub type ROpt<'borrow, T> = R<'borrow, Option<T>>;
