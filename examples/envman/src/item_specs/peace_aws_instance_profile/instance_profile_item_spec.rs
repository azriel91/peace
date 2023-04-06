use std::marker::PhantomData;

use peace::{
    cfg::{async_trait, ItemSpec, ItemSpecId, OpCtx},
    resources::{resources::ts::Empty, Resources},
};

use crate::item_specs::peace_aws_instance_profile::{
    InstanceProfileApplyOpSpec, InstanceProfileData, InstanceProfileError, InstanceProfileState,
    InstanceProfileStateCurrentFn, InstanceProfileStateDesiredFn, InstanceProfileStateDiff,
    InstanceProfileStateDiffFnSpec,
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
pub struct InstanceProfileItemSpec<Id> {
    /// ID of the instance profile item spec.
    item_spec_id: ItemSpecId,
    /// Marker for unique instance profile parameters type.
    marker: PhantomData<Id>,
}

impl<Id> InstanceProfileItemSpec<Id> {
    /// Returns a new `InstanceProfileItemSpec`.
    pub fn new(item_spec_id: ItemSpecId) -> Self {
        Self {
            item_spec_id,
            marker: PhantomData,
        }
    }
}

impl<Id> Clone for InstanceProfileItemSpec<Id> {
    fn clone(&self) -> Self {
        Self {
            item_spec_id: self.item_spec_id.clone(),
            marker: PhantomData,
        }
    }
}

#[async_trait(?Send)]
impl<Id> ItemSpec for InstanceProfileItemSpec<Id>
where
    Id: Send + Sync + 'static,
{
    type ApplyOpSpec = InstanceProfileApplyOpSpec<Id>;
    type Data<'op> = InstanceProfileData<'op, Id>;
    type Error = InstanceProfileError;
    type State = InstanceProfileState;
    type StateDiff = InstanceProfileStateDiff;
    type StateDiffFnSpec = InstanceProfileStateDiffFnSpec;

    fn id(&self) -> &ItemSpecId {
        &self.item_spec_id
    }

    async fn setup(&self, resources: &mut Resources<Empty>) -> Result<(), InstanceProfileError> {
        if !resources.contains::<aws_sdk_iam::Client>() {
            let sdk_config = aws_config::load_from_env().await;
            let client = aws_sdk_iam::Client::new(&sdk_config);
            resources.insert(client);
        }
        Ok(())
    }

    async fn try_state_current(
        op_ctx: OpCtx<'_>,
        data: InstanceProfileData<'_, Id>,
    ) -> Result<Option<Self::State>, InstanceProfileError> {
        InstanceProfileStateCurrentFn::try_state_current(op_ctx, data).await
    }

    async fn state_current(
        op_ctx: OpCtx<'_>,
        data: InstanceProfileData<'_, Id>,
    ) -> Result<Self::State, InstanceProfileError> {
        InstanceProfileStateCurrentFn::state_current(op_ctx, data).await
    }

    async fn try_state_desired(
        op_ctx: OpCtx<'_>,
        data: InstanceProfileData<'_, Id>,
    ) -> Result<Option<Self::State>, InstanceProfileError> {
        InstanceProfileStateDesiredFn::try_state_desired(op_ctx, data).await
    }

    async fn state_desired(
        op_ctx: OpCtx<'_>,
        data: InstanceProfileData<'_, Id>,
    ) -> Result<Self::State, InstanceProfileError> {
        InstanceProfileStateDesiredFn::state_desired(op_ctx, data).await
    }

    async fn state_clean(_: Self::Data<'_>) -> Result<Self::State, InstanceProfileError> {
        Ok(InstanceProfileState::None)
    }
}
