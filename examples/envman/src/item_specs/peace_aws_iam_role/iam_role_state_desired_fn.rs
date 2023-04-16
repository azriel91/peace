use std::marker::PhantomData;

use peace::cfg::{state::Generated, FnCtx};

use crate::item_specs::peace_aws_iam_role::{
    model::ManagedPolicyAttachment, IamRoleData, IamRoleError, IamRoleParams, IamRoleState,
};

/// Reads the desired state of the instance profile state.
#[derive(Debug)]
pub struct IamRoleStateDesiredFn<Id>(PhantomData<Id>);

impl<Id> IamRoleStateDesiredFn<Id>
where
    Id: Send + Sync,
{
    pub async fn try_state_desired(
        fn_ctx: FnCtx<'_>,
        params_partial: Option<&IamRoleParams<Id>>,
        data: IamRoleData<'_, Id>,
    ) -> Result<Option<IamRoleState>, IamRoleError> {
        if let Some(params) = params_partial {
            Self::state_desired(fn_ctx, params, data).await.map(Some)
        } else {
            Ok(None)
        }
    }

    pub async fn state_desired(
        _fn_ctx: FnCtx<'_>,
        params: &IamRoleParams<Id>,
        data: IamRoleData<'_, Id>,
    ) -> Result<IamRoleState, IamRoleError> {
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
