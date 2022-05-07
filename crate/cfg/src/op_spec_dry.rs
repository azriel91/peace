use async_trait::async_trait;

use crate::OpSpec;

/// An [`OpSpec`] that supports dry run.
#[async_trait]
pub trait OpSpecDry: OpSpec {
    /// Dry run execution that does not actually alter state.
    ///
    /// This is a safe operation to show what would happen if the operation is
    /// executed.
    async fn exec_dry() -> Result<Self::Output, Self::Error>;
}
