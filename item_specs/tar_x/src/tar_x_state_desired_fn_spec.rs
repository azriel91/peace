use std::marker::PhantomData;

use peace::cfg::{async_trait, FnSpec};

use crate::{TarXData, TarXError, TarXState};

/// Status desired `FnSpec` for the tar to extract.
#[derive(Debug)]
pub struct TarXStateDesiredFnSpec<Id>(PhantomData<Id>);

#[async_trait(?Send)]
impl<Id> FnSpec for TarXStateDesiredFnSpec<Id>
where
    Id: Send + Sync + 'static,
{
    type Data<'op> = TarXData<'op, Id>;
    type Error = TarXError;
    type Output = TarXState;

    async fn exec(_tar_x_data: TarXData<'_, Id>) -> Result<Self::Output, TarXError> {
        todo!()
    }
}
