use std::marker::PhantomData;

use aws_sdk_iam::{error::GetRoleErrorKind, types::SdkError};
use peace::cfg::{async_trait, state::Generated, TryFnSpec};

use crate::item_specs::peace_aws_iam_role::{
    model::{ManagedPolicyAttachment, RoleIdAndArn},
    IamRoleData, IamRoleError, IamRoleState,
};

/// Reads the current state of the instance profile state.
#[derive(Debug)]
pub struct IamRoleStateCurrentFnSpec<Id>(PhantomData<Id>);

#[async_trait(?Send)]
impl<Id> TryFnSpec for IamRoleStateCurrentFnSpec<Id>
where
    Id: Send + Sync + 'static,
{
    type Data<'op> = IamRoleData<'op, Id>;
    type Error = IamRoleError;
    type Output = IamRoleState;

    async fn try_exec(data: IamRoleData<'_, Id>) -> Result<Option<Self::Output>, IamRoleError> {
        // Hack: Remove this when referential param values is implemented.
        if data.managed_policy_arn().is_none() {
            return Ok(None);
        }

        Self::exec(data).await.map(Some)
    }

    async fn exec(data: IamRoleData<'_, Id>) -> Result<Self::Output, IamRoleError> {
        let client = data.client();
        let name = data.params().name();
        let path = data.params().path();
        let managed_policy_arn = data
            .managed_policy_arn()
            // Hack: Remove this when referential param values is implemented.
            .expect("IAM Role item spec: Expected ManagedPolicyArn to be Some.");

        let get_role_result = client.get_role().role_name(name).send().await;
        let role_opt = match get_role_result {
            Ok(get_role_output) => {
                let role = get_role_output
                    .role()
                    .expect("Expected Role to exist when get_role is successful");

                let role_name = role
                    .role_name()
                    .expect("Expected role name to be Some when get_role is successful.")
                    .to_string();
                let role_path = role
                    .path()
                    .expect("Expected path to be Some when get_role is successful.")
                    .to_string();
                let role_id = role
                    .role_id()
                    .expect("Expected role id to be Some when get_role is successful.")
                    .to_string();
                let role_arn = role
                    .arn()
                    .expect("Expected role ARN to be Some when get_role is successful.")
                    .to_string();

                let role_id_and_arn = RoleIdAndArn::new(role_id, role_arn);

                Some((role_name, role_path, role_id_and_arn))
            }
            Err(error) => match &error {
                SdkError::ServiceError(service_error) => match service_error.err().kind {
                    GetRoleErrorKind::NoSuchEntityException(_) => None,
                    _ => {
                        return Err(IamRoleError::RoleGetError {
                            role_name: name.to_string(),
                            error,
                        });
                    }
                },
                SdkError::ConstructionFailure(_)
                | SdkError::TimeoutError(_)
                | SdkError::DispatchFailure(_)
                | SdkError::ResponseError(_)
                | _ => {
                    return Err(IamRoleError::RoleGetError {
                        role_name: name.to_string(),
                        error,
                    });
                }
            },
        };

        let list_attached_role_policies_output = client
            .list_attached_role_policies()
            .role_name(name)
            .path_prefix(path)
            .send()
            .await
            .map_err(|error| IamRoleError::ManagedPoliciesListError {
                role_name: name.to_string(),
                role_path: path.to_string(),
                error,
            })?;
        let attached = list_attached_role_policies_output
            .attached_policies()
            .and_then(|attached_policies| {
                attached_policies.iter().find_map(|attached_policy| {
                    attached_policy
                        .policy_arn()
                        .map(|policy_arn| policy_arn == managed_policy_arn.arn())
                })
            })
            .unwrap_or(false);
        let managed_policy_attachment =
            ManagedPolicyAttachment::new(managed_policy_arn.to_string(), attached);

        match role_opt {
            None => Ok(IamRoleState::None),
            Some((role_name, role_path, role_id_and_arn)) => {
                assert_eq!(name, role_name);
                assert_eq!(path, role_path);

                let state_current = IamRoleState::Some {
                    name: name.to_string(),
                    path: path.to_string(),
                    role_id_and_arn: Generated::Value(role_id_and_arn),
                    managed_policy_attachment,
                };

                Ok(state_current)
            }
        }
    }
}
