use std::marker::PhantomData;

use peace::cfg::{async_trait, state::Generated, TryFnSpec};

use crate::item_specs::peace_aws_iam_role::{
    model::ManagedPolicyAttachment, IamRoleData, IamRoleError, IamRoleState,
};

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

    async fn try_exec(data: IamRoleData<'_, Id>) -> Result<Option<Self::Output>, IamRoleError> {
        // Hack: Remove this when referential param values is implemented.
        if data.managed_policy_arn().is_none() {
            return Ok(None);
        }

        Self::exec(data).await.map(Some)
    }

    async fn exec(data: IamRoleData<'_, Id>) -> Result<Self::Output, IamRoleError> {
        let params = data.params();
        let name = params.name().to_string();
        let path = params.path().to_string();
        let managed_policy_attachment = ManagedPolicyAttachment::new(
            data.managed_policy_arn()
                // Hack: Remove this when referential param values is implemented.
                .map(|managed_policy_arn| Generated::Value(managed_policy_arn.to_string()))
                .unwrap_or(Generated::Tbd),
            true,
        );
        let role_id_and_arn = Generated::Tbd;

        Ok(IamRoleState::Some {
            name,
            path,
            role_id_and_arn,
            managed_policy_attachment,
        })
    }
}
