use fn_graph::FnGraphBuilder;

use crate::FullSpecBoxed;

/// Graph of all [`FullSpec`]s.
///
/// [`FullSpec`]: peace_cfg::FullSpec
pub type FullSpecGraphBuilder<E> = FnGraphBuilder<FullSpecBoxed<E>>;
