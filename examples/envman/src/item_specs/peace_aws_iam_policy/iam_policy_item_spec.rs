use std::marker::PhantomData;

use peace::{
    cfg::{async_trait, ItemSpec, ItemSpecId, OpCtx},
    resources::{resources::ts::Empty, Resources},
};

use crate::item_specs::peace_aws_iam_policy::{
    model::ManagedPolicyArn, IamPolicyApplyOpSpec, IamPolicyData, IamPolicyError, IamPolicyState,
    IamPolicyStateCurrentFnSpec, IamPolicyStateDesiredFnSpec, IamPolicyStateDiff,
    IamPolicyStateDiffFnSpec,
};

/// Item spec to create an IAM instance profile and IAM role.
///
/// In sequence, this will:
///
/// * Create the IAM Role.
/// * Create the instance profile.
/// * Add the IAM role to the instance profile.
///
/// The `Id` type parameter is needed for each instance profile params to be a
/// distinct type.
///
/// # Type Parameters
///
/// * `Id`: A zero-sized type used to distinguish different instance profile
///   parameters from each other.
#[derive(Debug)]
pub struct IamPolicyItemSpec<Id> {
    /// ID of the instance profile item spec.
    item_spec_id: ItemSpecId,
    /// Marker for unique instance profile parameters type.
    marker: PhantomData<Id>,
}

impl<Id> IamPolicyItemSpec<Id> {
    /// Returns a new `IamPolicyItemSpec`.
    pub fn new(item_spec_id: ItemSpecId) -> Self {
        Self {
            item_spec_id,
            marker: PhantomData,
        }
    }
}

impl<Id> Clone for IamPolicyItemSpec<Id> {
    fn clone(&self) -> Self {
        Self {
            item_spec_id: self.item_spec_id.clone(),
            marker: PhantomData,
        }
    }
}

#[async_trait(?Send)]
impl<Id> ItemSpec for IamPolicyItemSpec<Id>
where
    Id: Send + Sync + 'static,
{
    type ApplyOpSpec = IamPolicyApplyOpSpec<Id>;
    type Data<'op> = IamPolicyData<'op, Id>;
    type Error = IamPolicyError;
    type State = IamPolicyState;
    type StateDiff = IamPolicyStateDiff;
    type StateDiffFnSpec = IamPolicyStateDiffFnSpec;

    fn id(&self) -> &ItemSpecId {
        &self.item_spec_id
    }

    async fn setup(&self, resources: &mut Resources<Empty>) -> Result<(), IamPolicyError> {
        if !resources.contains::<aws_sdk_iam::Client>() {
            let sdk_config = aws_config::load_from_env().await;
            let client = aws_sdk_iam::Client::new(&sdk_config);
            resources.insert(client);
        }
        // Hack: Remove this when referential param values is implemented.
        resources.insert(Option::<ManagedPolicyArn<Id>>::None);
        Ok(())
    }

    async fn try_state_current(
        op_ctx: OpCtx<'_>,
        data: IamPolicyData<'_, Id>,
    ) -> Result<Option<Self::State>, IamPolicyError> {
        IamPolicyStateCurrentFnSpec::try_state_current(op_ctx, data).await
    }

    async fn state_current(
        op_ctx: OpCtx<'_>,
        data: IamPolicyData<'_, Id>,
    ) -> Result<Self::State, IamPolicyError> {
        IamPolicyStateCurrentFnSpec::state_current(op_ctx, data).await
    }

    async fn try_state_desired(
        op_ctx: OpCtx<'_>,
        data: IamPolicyData<'_, Id>,
    ) -> Result<Option<Self::State>, IamPolicyError> {
        IamPolicyStateDesiredFnSpec::try_state_desired(op_ctx, data).await
    }

    async fn state_desired(
        op_ctx: OpCtx<'_>,
        data: IamPolicyData<'_, Id>,
    ) -> Result<Self::State, IamPolicyError> {
        IamPolicyStateDesiredFnSpec::state_desired(op_ctx, data).await
    }

    async fn state_clean(_: Self::Data<'_>) -> Result<Self::State, IamPolicyError> {
        Ok(IamPolicyState::None)
    }
}
