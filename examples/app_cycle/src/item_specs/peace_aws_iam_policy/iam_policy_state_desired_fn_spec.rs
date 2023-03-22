use std::marker::PhantomData;

use peace::cfg::{async_trait, state::Generated, OpCtx, TryFnSpec};

use crate::item_specs::peace_aws_iam_policy::{IamPolicyData, IamPolicyError, IamPolicyState};

/// Reads the desired state of the instance profile state.
#[derive(Debug)]
pub struct IamPolicyStateDesiredFnSpec<Id>(PhantomData<Id>);

#[async_trait(?Send)]
impl<Id> TryFnSpec for IamPolicyStateDesiredFnSpec<Id>
where
    Id: Send + Sync + 'static,
{
    type Data<'op> = IamPolicyData<'op, Id>;
    type Error = IamPolicyError;
    type Output = IamPolicyState;

    async fn try_exec(
        op_ctx: OpCtx<'_>,
        iam_policy_data: IamPolicyData<'_, Id>,
    ) -> Result<Option<Self::Output>, IamPolicyError> {
        Self::exec(op_ctx, iam_policy_data).await.map(Some)
    }

    async fn exec(
        _op_ctx: OpCtx<'_>,
        iam_policy_data: IamPolicyData<'_, Id>,
    ) -> Result<Self::Output, IamPolicyError> {
        let params = iam_policy_data.params();
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
