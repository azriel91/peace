use std::fmt::Debug;

use fn_graph::DataAccessDyn;

use crate::full_spec_boxed::{CleanOpSpecRt, EnsureOpSpecRt, StatusOpSpecRt};

/// Internal trait that erases the types from [`FullSpec`]
///
/// This exists so that different implementations of [`FullSpec`] can be held
/// under the same boxed trait.
///
/// [`FullSpec`]: peace_cfg::FullSpec
pub trait FullSpecRt<'op>:
    Debug + DataAccessDyn + CleanOpSpecRt<'op> + EnsureOpSpecRt<'op> + StatusOpSpecRt<'op>
{
}
