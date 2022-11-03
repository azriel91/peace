use std::marker::PhantomData;

#[nougat::gat(Data)]
use peace::cfg::FnSpec;
use peace::cfg::{async_trait, nougat};

use crate::{TarXData, TarXError, TarXState};

/// Status desired `FnSpec` for the tar to extract.
#[derive(Debug)]
pub struct TarXStateDesiredFnSpec<Id>(PhantomData<Id>);

#[async_trait(?Send)]
#[nougat::gat]
impl<Id> FnSpec for TarXStateDesiredFnSpec<Id>
where
    Id: Send + Sync + 'static,
{
    type Data<'op> = TarXData<'op, Id>
        where Self: 'op;
    type Error = TarXError;
    type Output = TarXState;

    async fn exec(_tar_x_data: TarXData<'_, Id>) -> Result<Self::Output, TarXError> {
        todo!()
    }
}
