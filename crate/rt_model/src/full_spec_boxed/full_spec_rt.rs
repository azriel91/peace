use std::fmt::Debug;

use fn_graph::{DataAccess, DataAccessDyn, Resources};
use peace_cfg::async_trait;

use crate::full_spec_boxed::{CleanOpSpecRt, EnsureOpSpecRt, StatusFnSpecRt};

/// Internal trait that erases the types from [`FullSpec`]
///
/// This exists so that different implementations of [`FullSpec`] can be held
/// under the same boxed trait.
///
/// [`FullSpec`]: peace_cfg::FullSpec
#[async_trait]
pub trait FullSpecRt<E>:
    Debug
    + DataAccess
    + DataAccessDyn
    + CleanOpSpecRt<Error = E>
    + EnsureOpSpecRt<Error = E>
    + StatusFnSpecRt<Error = E>
where
    E: Debug + std::error::Error,
{
    /// Initializes data for the operation's check and `exec` functions.
    async fn setup(&self, resources: &mut Resources) -> Result<(), E>;

    /// Runs [`FullSpec::StatusFnSpec`]`::`[`exec`].
    ///
    /// [`FullSpec::StatusFnSpec`]: peace_cfg::FullSpec::StatusFnSpec
    /// [`exec`]: peace_cfg::FnSpec::exec
    async fn status_fn_exec(&self, resources: &Resources) -> Result<(), E>;
}
