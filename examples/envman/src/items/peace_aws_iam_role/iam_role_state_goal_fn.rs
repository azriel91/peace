use std::marker::PhantomData;

use peace::{
    cfg::{state::Generated, FnCtx},
    params::Params,
};

use crate::items::peace_aws_iam_role::{
    model::ManagedPolicyAttachment, IamRoleData, IamRoleError, IamRoleParams, IamRoleState,
};

/// Reads the goal state of the instance profile state.
#[derive(Debug)]
pub struct IamRoleStateGoalFn<Id>(PhantomData<Id>);

impl<Id> IamRoleStateGoalFn<Id>
where
    Id: Send + Sync,
{
    pub async fn try_state_goal(
        _fn_ctx: FnCtx<'_>,
        params_partial: &<IamRoleParams<Id> as Params>::Partial,
        _data: IamRoleData<'_, Id>,
    ) -> Result<Option<IamRoleState>, IamRoleError> {
        let name = params_partial.name();
        let path = params_partial.path();
        if let Some((name, path)) = name.zip(path) {
            Self::state_goal_internal(
                name.to_string(),
                path.to_string(),
                params_partial
                    .managed_policy_arn()
                    .map(|managed_policy_arn| managed_policy_arn.to_string()),
            )
            .await
            .map(Some)
        } else {
            Ok(None)
        }
    }

    pub async fn state_goal(
        _fn_ctx: FnCtx<'_>,
        params: &IamRoleParams<Id>,
        _data: IamRoleData<'_, Id>,
    ) -> Result<IamRoleState, IamRoleError> {
        let name = params.name().to_string();
        let path = params.path().to_string();
        let managed_policy_arn = Some(params.managed_policy_arn().to_string());

        Self::state_goal_internal(name, path, managed_policy_arn).await
    }

    async fn state_goal_internal(
        name: String,
        path: String,
        managed_policy_arn: Option<String>,
    ) -> Result<IamRoleState, IamRoleError> {
        let managed_policy_attachment = ManagedPolicyAttachment::new(
            managed_policy_arn
                // Hack: Remove this when referential param values is implemented.
                .map(Generated::Value)
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
