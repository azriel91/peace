use std::marker::PhantomData;

use peace::cfg::{state::Generated, FnCtx};

use crate::item_specs::peace_aws_instance_profile::{
    InstanceProfileData, InstanceProfileError, InstanceProfileParams, InstanceProfileState,
};

/// Reads the desired state of the instance profile state.
#[derive(Debug)]
pub struct InstanceProfileStateDesiredFn<Id>(PhantomData<Id>);

impl<Id> InstanceProfileStateDesiredFn<Id>
where
    Id: Send + Sync + 'static,
{
    pub async fn try_state_desired(
        fn_ctx: FnCtx<'_>,
        params_partial: Option<&InstanceProfileParams<Id>>,
        data: InstanceProfileData<'_, Id>,
    ) -> Result<Option<InstanceProfileState>, InstanceProfileError> {
        if let Some(params) = params_partial {
            Self::state_desired(fn_ctx, params, data).await.map(Some)
        } else {
            Ok(None)
        }
    }

    pub async fn state_desired(
        _fn_ctx: FnCtx<'_>,
        params: &InstanceProfileParams<Id>,
        _data: InstanceProfileData<'_, Id>,
    ) -> Result<InstanceProfileState, InstanceProfileError> {
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
