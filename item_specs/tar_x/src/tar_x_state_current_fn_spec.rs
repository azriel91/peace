use std::marker::PhantomData;

use peace::cfg::{async_trait, state::Nothing, FnSpec, State};

use crate::{TarXData, TarXError, TarXState};

/// Status `FnSpec` for the tar to extract.
#[derive(Debug)]
pub struct TarXStateCurrentFnSpec<Id>(PhantomData<Id>);

#[async_trait(?Send)]
impl<Id> FnSpec for TarXStateCurrentFnSpec<Id>
where
    Id: Send + Sync + 'static,
{
    type Data<'op> = TarXData<'op, Id>;
    type Error = TarXError;
    type Output = State<TarXState, Nothing>;

    async fn exec(_tar_x_data: TarXData<'_, Id>) -> Result<Self::Output, TarXError> {
        todo!()
    }
}
