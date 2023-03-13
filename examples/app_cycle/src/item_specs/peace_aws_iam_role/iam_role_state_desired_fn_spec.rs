use std::marker::PhantomData;

use peace::cfg::{async_trait, state::Generated, TryFnSpec};

use crate::item_specs::peace_aws_iam_role::{IamRoleData, IamRoleError, IamRoleState};

/// Reads the desired state of the instance profile state.
#[derive(Debug)]
pub struct IamRoleStateDesiredFnSpec<Id>(PhantomData<Id>);

#[async_trait(?Send)]
impl<Id> TryFnSpec for IamRoleStateDesiredFnSpec<Id>
where
    Id: Send + Sync + 'static,
{
    type Data<'op> = IamRoleData<'op, Id>;
    type Error = IamRoleError;
    type Output = IamRoleState;

    async fn try_exec(
        iam_role_data: IamRoleData<'_, Id>,
    ) -> Result<Option<Self::Output>, IamRoleError> {
        Self::exec(iam_role_data).await.map(Some)
    }

    async fn exec(iam_role_data: IamRoleData<'_, Id>) -> Result<Self::Output, IamRoleError> {
        let params = iam_role_data.params();
        let name = params.name().to_string();
        let path = params.path().to_string();
        let role_id_and_arn = Generated::Tbd;

        Ok(IamRoleState::Some {
            name,
            path,
            role_id_and_arn,
        })
    }
}
