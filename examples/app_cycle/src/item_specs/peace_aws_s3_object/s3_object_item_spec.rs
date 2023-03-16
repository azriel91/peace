use std::marker::PhantomData;

use peace::{
    cfg::{async_trait, ItemSpec, ItemSpecId},
    resources::{resources::ts::Empty, Resources},
};

use crate::item_specs::peace_aws_s3_object::{
    S3ObjectApplyOpSpec, S3ObjectCleanOpSpec, S3ObjectError, S3ObjectState,
    S3ObjectStateCurrentFnSpec, S3ObjectStateDesiredFnSpec, S3ObjectStateDiff,
    S3ObjectStateDiffFnSpec,
};

/// Item spec to create an IAM S3 object and IAM role.
///
/// In sequence, this will:
///
/// * Create the IAM Role.
/// * Create the S3 object.
/// * Add the IAM role to the S3 object.
///
/// The `Id` type parameter is needed for each S3 object params to be a
/// distinct type.
///
/// # Type Parameters
///
/// * `Id`: A zero-sized type used to distinguish different S3 object parameters
///   from each other.
#[derive(Debug)]
pub struct S3ObjectItemSpec<Id> {
    /// ID of the S3 object item spec.
    item_spec_id: ItemSpecId,
    /// Marker for unique S3 object parameters type.
    marker: PhantomData<Id>,
}

impl<Id> S3ObjectItemSpec<Id> {
    /// Returns a new `S3ObjectItemSpec`.
    pub fn new(item_spec_id: ItemSpecId) -> Self {
        Self {
            item_spec_id,
            marker: PhantomData,
        }
    }
}

impl<Id> Clone for S3ObjectItemSpec<Id> {
    fn clone(&self) -> Self {
        Self {
            item_spec_id: self.item_spec_id.clone(),
            marker: PhantomData,
        }
    }
}

#[async_trait(?Send)]
impl<Id> ItemSpec for S3ObjectItemSpec<Id>
where
    Id: Send + Sync + 'static,
{
    type ApplyOpSpec = S3ObjectApplyOpSpec<Id>;
    type CleanOpSpec = S3ObjectCleanOpSpec<Id>;
    type Error = S3ObjectError;
    type State = S3ObjectState;
    type StateCurrentFnSpec = S3ObjectStateCurrentFnSpec<Id>;
    type StateDesiredFnSpec = S3ObjectStateDesiredFnSpec<Id>;
    type StateDiff = S3ObjectStateDiff;
    type StateDiffFnSpec = S3ObjectStateDiffFnSpec;

    fn id(&self) -> &ItemSpecId {
        &self.item_spec_id
    }

    async fn setup(&self, resources: &mut Resources<Empty>) -> Result<(), S3ObjectError> {
        if !resources.contains::<aws_sdk_s3::Client>() {
            let sdk_config = aws_config::load_from_env().await;
            resources.insert(sdk_config.region().cloned());
            let client = aws_sdk_s3::Client::new(&sdk_config);
            resources.insert(client);
        }
        Ok(())
    }
}
