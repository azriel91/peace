use std::marker::PhantomData;

use aws_sdk_iam::{error::GetRoleErrorKind, types::SdkError};
use peace::cfg::{async_trait, state::Generated, OpCtx, TryFnSpec};

use crate::item_specs::peace_aws_iam_role::{
    model::{ManagedPolicyAttachment, RoleIdAndArn},
    IamRoleData, IamRoleError, IamRoleState,
};

#[cfg(feature = "output_progress")]
use peace::cfg::progress::ProgressMsgUpdate;

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

    async fn try_exec(
        op_ctx: OpCtx<'_>,
        data: IamRoleData<'_, Id>,
    ) -> Result<Option<Self::Output>, IamRoleError> {
        Self::exec(op_ctx, data).await.map(Some)
    }

    async fn exec(
        op_ctx: OpCtx<'_>,
        data: IamRoleData<'_, Id>,
    ) -> Result<Self::Output, IamRoleError> {
        let client = data.client();
        let name = data.params().name();
        let path = data.params().path();

        #[cfg(not(feature = "output_progress"))]
        let _op_ctx = op_ctx;
        #[cfg(feature = "output_progress")]
        let progress_sender = &op_ctx.progress_sender;
        #[cfg(feature = "output_progress")]
        progress_sender.tick(ProgressMsgUpdate::Set(String::from("fetching role")));
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
                _ => {
                    return Err(IamRoleError::RoleGetError {
                        role_name: name.to_string(),
                        error,
                    });
                }
            },
        };

        match role_opt {
            None => {
                #[cfg(feature = "output_progress")]
                progress_sender.tick(ProgressMsgUpdate::Set(String::from("policy not fetched")));
                Ok(IamRoleState::None)
            }
            Some((role_name, role_path, role_id_and_arn)) => {
                assert_eq!(name, role_name);
                assert_eq!(path, role_path);

                #[cfg(feature = "output_progress")]
                progress_sender.tick(ProgressMsgUpdate::Set(String::from("policy fetched")));

                #[cfg(feature = "output_progress")]
                progress_sender.tick(ProgressMsgUpdate::Set(String::from(
                    "listing attached policies",
                )));
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
                #[cfg(feature = "output_progress")]
                progress_sender.tick(ProgressMsgUpdate::Set(String::from(
                    "filtering attached policies",
                )));
                let managed_policy_attachment = list_attached_role_policies_output
                    .attached_policies()
                    .and_then(|attached_policies| {
                        attached_policies.iter().find_map(|attached_policy| {
                            let policy_name = attached_policy
                                .policy_name()
                                .expect("Expected policy_name to be Some for any attached policy.");

                            if policy_name == name {
                                Some(
                                    attached_policy
                                        .policy_arn()
                                        .expect(
                                            "Expected policy_arn to be Some for \
                                            any attached policy.",
                                        )
                                        .to_string(),
                                )
                            } else {
                                None
                            }
                        })
                    })
                    .map(|managed_policy_arn| {
                        ManagedPolicyAttachment::new(Generated::Value(managed_policy_arn), true)
                    })
                    .unwrap_or(ManagedPolicyAttachment::new(Generated::Tbd, false));
                #[cfg(feature = "output_progress")]
                {
                    let message = if managed_policy_attachment.attached() {
                        "policy attached"
                    } else {
                        "policy not attached"
                    };
                    progress_sender.tick(ProgressMsgUpdate::Set(String::from(message)));
                }

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
