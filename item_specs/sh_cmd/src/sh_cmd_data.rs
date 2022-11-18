#[cfg(target_arch = "wasm32")]
use peace::rt_model::Storage;

use peace::data::{Data, R};

use crate::ShCmdParams;

/// Data used to run a shell command.
///
/// # Type Parameters
///
/// * `Id`: A zero-sized type used to distinguish different command execution
///   parameters from each other.
#[derive(Data, Debug)]
pub struct ShCmdData<'op, Id>
where
    Id: Send + Sync + 'static,
{
    /// Parameters to determine what shell command to run.
    sh_cmd_params: R<'op, ShCmdParams<Id>>,

    /// For wasm, we write to web storage through the `Storage` object.
    ///
    /// Presumably we should be able to use this for `NativeStorage` as well.
    #[cfg(target_arch = "wasm32")]
    storage: R<'op, Storage>,
}

impl<'op, Id> ShCmdData<'op, Id>
where
    Id: Send + Sync + 'static,
{
    #[cfg(not(target_arch = "wasm32"))]
    pub fn new(sh_cmd_params: R<'op, ShCmdParams<Id>>) -> Self {
        Self { sh_cmd_params }
    }

    #[cfg(target_arch = "wasm32")]
    pub fn new(sh_cmd_params: R<'op, ShCmdParams<Id>>, storage: R<'op, Storage>) -> Self {
        Self {
            sh_cmd_params,
            storage,
        }
    }

    pub fn sh_cmd_params(&self) -> &ShCmdParams<Id> {
        &self.sh_cmd_params
    }

    #[cfg(target_arch = "wasm32")]
    pub fn storage(&self) -> &Storage {
        &*self.storage
    }
}
