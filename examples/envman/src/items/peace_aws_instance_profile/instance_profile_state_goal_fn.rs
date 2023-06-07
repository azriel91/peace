use std::marker::PhantomData;

use peace::{
    cfg::{state::Generated, FnCtx},
    params::Params,
};

use crate::items::peace_aws_instance_profile::{
    InstanceProfileData, InstanceProfileError, InstanceProfileParams, InstanceProfileState,
};

/// Reads the goal state of the instance profile state.
#[derive(Debug)]
pub struct InstanceProfileStateGoalFn<Id>(PhantomData<Id>);

impl<Id> InstanceProfileStateGoalFn<Id>
where
    Id: Send + Sync + 'static,
{
    pub async fn try_state_goal(
        _fn_ctx: FnCtx<'_>,
        params_partial: &<InstanceProfileParams<Id> as Params>::Partial,
        _data: InstanceProfileData<'_, Id>,
    ) -> Result<Option<InstanceProfileState>, InstanceProfileError> {
        let name = params_partial.name();
        let path = params_partial.path();
        let role_associate = params_partial.role_associate();
        if let Some(((name, path), role_associated)) = name.zip(path).zip(role_associate) {
            Self::state_goal_internal(name.to_string(), path.to_string(), *role_associated)
                .await
                .map(Some)
        } else {
            Ok(None)
        }
    }

    pub async fn state_goal(
        _fn_ctx: FnCtx<'_>,
        params: &InstanceProfileParams<Id>,
        _data: InstanceProfileData<'_, Id>,
    ) -> Result<InstanceProfileState, InstanceProfileError> {
        let name = params.name().to_string();
        let path = params.path().to_string();
        let role_associated = params.role_associate();

        Self::state_goal_internal(name, path, role_associated).await
    }

    async fn state_goal_internal(
        name: String,
        path: String,
        role_associated: bool,
    ) -> Result<InstanceProfileState, InstanceProfileError> {
        let instance_profile_id_and_arn = Generated::Tbd;

        Ok(InstanceProfileState::Some {
            name,
            path,
            instance_profile_id_and_arn,
            role_associated,
        })
    }
}
