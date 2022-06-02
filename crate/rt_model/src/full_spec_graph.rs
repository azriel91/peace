use fn_graph::FnGraph;

use crate::FullSpecBoxed;

/// Graph of all [`FullSpec`]s.
///
/// [`FullSpec`]: peace_cfg::FullSpec
pub type FullSpecGraph<'op> = FnGraph<FullSpecBoxed<'op>>;
