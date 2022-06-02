use fn_graph::DataAccessDyn;

/// Internal trait that erases the types from [`FullSpec`]
///
/// This exists so that different implementations of [`FullSpec`] can be held
/// under the same boxed trait.
///
/// [`FullSpec`]: peace_cfg::FullSpec
pub(super) trait FullSpecRt<'op>: DataAccessDyn {}
