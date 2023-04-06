use std::marker::PhantomData;

use peace::cfg::{state::Generated, OpCtx};

use crate::item_specs::peace_aws_instance_profile::{
    InstanceProfileData, InstanceProfileError, InstanceProfileState,
};

/// Reads the desired state of the instance profile state.
#[derive(Debug)]
pub struct InstanceProfileStateDesiredFnSpec<Id>(PhantomData<Id>);

impl<Id> InstanceProfileStateDesiredFnSpec<Id>
where
    Id: Send + Sync + 'static,
{
    pub async fn try_state_desired(
        op_ctx: OpCtx<'_>,
        instance_profile_data: InstanceProfileData<'_, Id>,
    ) -> Result<Option<InstanceProfileState>, InstanceProfileError> {
        Self::state_desired(op_ctx, instance_profile_data)
            .await
            .map(Some)
    }

    pub async fn state_desired(
        _op_ctx: OpCtx<'_>,
        instance_profile_data: InstanceProfileData<'_, Id>,
    ) -> Result<InstanceProfileState, InstanceProfileError> {
        let params = instance_profile_data.params();
        let name = params.name().to_string();
        let path = params.path().to_string();
        let role_associated = params.role_associate();
        let instance_profile_id_and_arn = Generated::Tbd;

        Ok(InstanceProfileState::Some {
            name,
            path,
            instance_profile_id_and_arn,
            role_associated,
        })
    }
}
