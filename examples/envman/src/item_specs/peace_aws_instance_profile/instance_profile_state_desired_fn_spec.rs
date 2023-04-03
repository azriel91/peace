use std::marker::PhantomData;

use peace::cfg::{async_trait, state::Generated, OpCtx, TryFnSpec};

use crate::item_specs::peace_aws_instance_profile::{
    InstanceProfileData, InstanceProfileError, InstanceProfileState,
};

/// Reads the desired state of the instance profile state.
#[derive(Debug)]
pub struct InstanceProfileStateDesiredFnSpec<Id>(PhantomData<Id>);

#[async_trait(?Send)]
impl<Id> TryFnSpec for InstanceProfileStateDesiredFnSpec<Id>
where
    Id: Send + Sync + 'static,
{
    type Data<'op> = InstanceProfileData<'op, Id>;
    type Error = InstanceProfileError;
    type Output = InstanceProfileState;

    async fn try_exec(
        op_ctx: OpCtx<'_>,
        instance_profile_data: InstanceProfileData<'_, Id>,
    ) -> Result<Option<Self::Output>, InstanceProfileError> {
        Self::exec(op_ctx, instance_profile_data).await.map(Some)
    }

    async fn exec(
        _op_ctx: OpCtx<'_>,
        instance_profile_data: InstanceProfileData<'_, Id>,
    ) -> Result<Self::Output, InstanceProfileError> {
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
