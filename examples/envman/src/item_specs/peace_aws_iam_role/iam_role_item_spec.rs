use std::marker::PhantomData;

use peace::{
    cfg::{async_trait, ItemSpec, ItemSpecId},
    resources::{resources::ts::Empty, Resources},
};

use crate::item_specs::peace_aws_iam_role::{
    IamRoleApplyOpSpec, IamRoleData, IamRoleError, IamRoleState, IamRoleStateCurrentFnSpec,
    IamRoleStateDesiredFnSpec, IamRoleStateDiff, IamRoleStateDiffFnSpec,
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
pub struct IamRoleItemSpec<Id> {
    /// ID of the instance profile item spec.
    item_spec_id: ItemSpecId,
    /// Marker for unique instance profile parameters type.
    marker: PhantomData<Id>,
}

impl<Id> IamRoleItemSpec<Id> {
    /// Returns a new `IamRoleItemSpec`.
    pub fn new(item_spec_id: ItemSpecId) -> Self {
        Self {
            item_spec_id,
            marker: PhantomData,
        }
    }
}

impl<Id> Clone for IamRoleItemSpec<Id> {
    fn clone(&self) -> Self {
        Self {
            item_spec_id: self.item_spec_id.clone(),
            marker: PhantomData,
        }
    }
}

#[async_trait(?Send)]
impl<Id> ItemSpec for IamRoleItemSpec<Id>
where
    Id: Send + Sync + 'static,
{
    type ApplyOpSpec = IamRoleApplyOpSpec<Id>;
    type Data<'op> = IamRoleData<'op, Id>;
    type Error = IamRoleError;
    type State = IamRoleState;
    type StateCurrentFnSpec = IamRoleStateCurrentFnSpec<Id>;
    type StateDesiredFnSpec = IamRoleStateDesiredFnSpec<Id>;
    type StateDiff = IamRoleStateDiff;
    type StateDiffFnSpec = IamRoleStateDiffFnSpec;

    fn id(&self) -> &ItemSpecId {
        &self.item_spec_id
    }

    async fn setup(&self, resources: &mut Resources<Empty>) -> Result<(), IamRoleError> {
        if !resources.contains::<aws_sdk_iam::Client>() {
            let sdk_config = aws_config::load_from_env().await;
            let client = aws_sdk_iam::Client::new(&sdk_config);
            resources.insert(client);
        }
        Ok(())
    }

    async fn state_clean(_: Self::Data<'_>) -> Result<Self::State, IamRoleError> {
        Ok(IamRoleState::None)
    }
}