use std::marker::PhantomData;

use peace::{
    cfg::{state::Generated, FnCtx},
    params::Params,
};

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
        _fn_ctx: FnCtx<'_>,
        params_partial: &<IamPolicyParams<Id> as Params>::Partial,
        _data: IamPolicyData<'_, Id>,
    ) -> Result<Option<IamPolicyState>, IamPolicyError> {
        let name = params_partial.name();
        let path = params_partial.path();
        let policy_document = params_partial.policy_document();

        if let Some(((name, path), policy_document)) = name.zip(path).zip(policy_document) {
            Self::state_desired_internal(
                name.to_string(),
                path.to_string(),
                policy_document.to_string(),
            )
            .await
            .map(Some)
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

        Self::state_desired_internal(name, path, policy_document).await
    }

    async fn state_desired_internal(
        name: String,
        path: String,
        policy_document: String,
    ) -> Result<IamPolicyState, IamPolicyError> {
        let policy_id_arn_version = Generated::Tbd;

        Ok(IamPolicyState::Some {
            name,
            path,
            policy_document,
            policy_id_arn_version,
        })
    }
}
