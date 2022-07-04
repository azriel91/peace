use fn_graph::Resources;
use peace_cfg::async_trait;

/// Type-erased trait corresponding to [`FullSpec::StatusFnSpec`].
///
/// Implementation of this will fetch Data from [`Resources`], then call the
/// corresponding [`FnSpec`] method.
///
/// [`FullSpec::StatusFnSpec`]: peace_cfg::FullSpec::StatusFnSpec
/// [`FnSpec`]: peace_cfg::FnSpec
#[async_trait]
pub trait StatusFnSpecRt<'op> {
    /// Error returned when any of the functions of this operation err.
    type Error: std::error::Error;

    /// Transforms the current state to the desired state.
    async fn exec(&self, resources: &'op Resources) -> Result<(), Self::Error>
    where
        'op: 'async_trait;
}
