use std::marker::PhantomData;

use peace::cfg::{state::Generated, FnCtx};

use crate::item_specs::peace_aws_iam_policy::{
    IamPolicyData, IamPolicyError, IamPolicyParams, IamPolicyState,
};

/// Reads the desired state of the instance profile state.
#[derive(Debug)]
pub struct IamPolicyStateDesiredFn<Id>(PhantomData<Id>);

impl<Id> IamPolicyStateDesiredFn<Id>
where
    Id: Send + Sync,
{
    pub async fn try_state_desired(
        fn_ctx: FnCtx<'_>,
        params_partial: Option<&IamPolicyParams<Id>>,
        data: IamPolicyData<'_, Id>,
    ) -> Result<Option<IamPolicyState>, IamPolicyError> {
        if let Some(params) = params_partial {
            Self::state_desired(fn_ctx, params, data).await.map(Some)
        } else {
            Ok(None)
        }
    }

    pub async fn state_desired(
        _fn_ctx: FnCtx<'_>,
        params: &IamPolicyParams<Id>,
        _data: IamPolicyData<'_, Id>,
    ) -> Result<IamPolicyState, IamPolicyError> {
        let name = params.name().to_string();
        let path = params.path().to_string();
        let policy_document = params.policy_document().to_string();
        let policy_id_arn_version = Generated::Tbd;

        Ok(IamPolicyState::Some {
            name,
            path,
            policy_document,
            policy_id_arn_version,
        })
    }
}
