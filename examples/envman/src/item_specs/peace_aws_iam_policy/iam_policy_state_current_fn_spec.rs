use std::marker::PhantomData;

use aws_sdk_iam::{error::GetPolicyErrorKind, types::SdkError};
use peace::cfg::{async_trait, state::Generated, OpCtx, TryFnSpec};

use crate::item_specs::peace_aws_iam_policy::{
    model::{ManagedPolicyArn, PolicyIdArnVersion},
    IamPolicyData, IamPolicyError, IamPolicyState,
};

#[cfg(feature = "output_progress")]
use peace::cfg::progress::ProgressMsgUpdate;

/// Reads the current state of the instance profile state.
#[derive(Debug)]
pub struct IamPolicyStateCurrentFnSpec<Id>(PhantomData<Id>);

impl<Id> IamPolicyStateCurrentFnSpec<Id> {
    /// Finds a policy with the given name and path.
    pub(crate) async fn policy_find(
        #[cfg(not(feature = "output_progress"))] _op_ctx: OpCtx<'_>,
        #[cfg(feature = "output_progress")] op_ctx: OpCtx<'_>,
        client: &aws_sdk_iam::Client,
        name: &str,
        path: &str,
    ) -> Result<Option<(String, String)>, IamPolicyError> {
        #[cfg(feature = "output_progress")]
        let progress_sender = &op_ctx.progress_sender;
        #[cfg(feature = "output_progress")]
        progress_sender.tick(ProgressMsgUpdate::Set(String::from("listing policies")));
        let list_policies_output = client
            .list_policies()
            .scope(aws_sdk_iam::model::PolicyScopeType::Local)
            .path_prefix(path)
            .send()
            .await
            .map_err(|error| {
                #[cfg(feature = "error_reporting")]
                let (aws_desc, aws_desc_span) = crate::item_specs::aws_error_desc!(&error);

                IamPolicyError::PoliciesListError {
                    path: path.to_string(),
                    #[cfg(feature = "error_reporting")]
                    aws_desc,
                    #[cfg(feature = "error_reporting")]
                    aws_desc_span,
                    error,
                }
            })?;
        #[cfg(feature = "output_progress")]
        progress_sender.tick(ProgressMsgUpdate::Set(String::from("finding policy")));
        let policy_id_arn_version = list_policies_output
            .policies()
            .and_then(|policies| {
                policies.iter().find(|policy| {
                    let name_matches = policy
                        .policy_name()
                        .filter(|policy_name| *policy_name == name)
                        .is_some();
                    let path_matches = policy
                        .path()
                        .filter(|policy_path| *policy_path == path)
                        .is_some();

                    name_matches && path_matches
                })
            })
            .map(|policy| {
                let policy_id = policy
                    .policy_id()
                    .expect("Expected policy id to be Some.")
                    .to_string();
                let policy_arn = policy.arn().expect("Expected ARN to be Some.").to_string();
                (policy_id, policy_arn)
            });

        #[cfg(feature = "output_progress")]
        {
            let message = if policy_id_arn_version.is_some() {
                "policy found"
            } else {
                "policy not found"
            };
            progress_sender.tick(ProgressMsgUpdate::Set(String::from(message)));
        }

        Ok(policy_id_arn_version)
    }
}

#[async_trait(?Send)]
impl<Id> TryFnSpec for IamPolicyStateCurrentFnSpec<Id>
where
    Id: Send + Sync + 'static,
{
    type Data<'op> = IamPolicyData<'op, Id>;
    type Error = IamPolicyError;
    type Output = IamPolicyState;

    async fn try_exec(
        op_ctx: OpCtx<'_>,
        data: IamPolicyData<'_, Id>,
    ) -> Result<Option<Self::Output>, IamPolicyError> {
        Self::exec(op_ctx, data).await.map(Some)
    }

    async fn exec(
        op_ctx: OpCtx<'_>,
        mut data: IamPolicyData<'_, Id>,
    ) -> Result<Self::Output, IamPolicyError> {
        let client = data.client();
        let name = data.params().name();
        let path = data.params().path();

        let policy_id_arn_version = Self::policy_find(op_ctx, client, name, path).await?;

        if let Some((policy_id, policy_arn)) = policy_id_arn_version {
            #[cfg(feature = "output_progress")]
            let progress_sender = &op_ctx.progress_sender;
            #[cfg(feature = "output_progress")]
            progress_sender.tick(ProgressMsgUpdate::Set(String::from("fetching policy")));

            let get_policy_result = client.get_policy().policy_arn(&policy_arn).send().await;
            let (policy_name, policy_path, policy_id_arn_version) = match get_policy_result {
                Ok(get_policy_output) => {
                    #[cfg(feature = "output_progress")]
                    progress_sender.tick(ProgressMsgUpdate::Set(String::from("policy fetched")));

                    let policy = get_policy_output
                        .policy()
                        .expect("Expected Policy to exist when get_policy is successful");

                    let policy_name = policy
                        .policy_name()
                        .expect("Expected policy name to be Some when get_policy is successful.")
                        .to_string();
                    let policy_path = policy
                        .path()
                        .expect("Expected path to be Some when get_policy is successful.")
                        .to_string();
                    let policy_id = policy
                        .policy_id()
                        .expect("Expected policy id to be Some when get_policy is successful.")
                        .to_string();
                    let policy_arn = policy
                        .arn()
                        .expect("Expected policy ARN to be Some when get_policy is successful.")
                        .to_string();
                    let policy_version = policy
                        .default_version_id()
                        .expect(
                            "Expected policy default version to be Some when \
                            get_policy is successful.",
                        )
                        .to_string();

                    let policy_id_arn_version =
                        PolicyIdArnVersion::new(policy_id, policy_arn, policy_version);

                    (policy_name, policy_path, policy_id_arn_version)
                }
                Err(error) => {
                    #[cfg(feature = "output_progress")]
                    progress_sender
                        .tick(ProgressMsgUpdate::Set(String::from("policy not fetched")));

                    #[cfg(feature = "error_reporting")]
                    let (aws_desc, aws_desc_span) = crate::item_specs::aws_error_desc!(&error);

                    match &error {
                        SdkError::ServiceError(service_error) => match service_error.err().kind {
                            GetPolicyErrorKind::NoSuchEntityException(_) => {
                                return Err(IamPolicyError::PolicyNotFoundAfterList {
                                    policy_name: name.to_string(),
                                    policy_path: path.to_string(),
                                    policy_id: policy_id.to_string(),
                                    policy_arn: policy_arn.to_string(),
                                    #[cfg(feature = "error_reporting")]
                                    aws_desc,
                                    #[cfg(feature = "error_reporting")]
                                    aws_desc_span,
                                });
                            }
                            _ => {
                                return Err(IamPolicyError::PolicyGetError {
                                    policy_name: name.to_string(),
                                    policy_path: path.to_string(),
                                    #[cfg(feature = "error_reporting")]
                                    aws_desc,
                                    #[cfg(feature = "error_reporting")]
                                    aws_desc_span,
                                    error,
                                });
                            }
                        },
                        _ => {
                            return Err(IamPolicyError::PolicyGetError {
                                policy_name: name.to_string(),
                                policy_path: path.to_string(),
                                #[cfg(feature = "error_reporting")]
                                aws_desc,
                                #[cfg(feature = "error_reporting")]
                                aws_desc_span,
                                error,
                            });
                        }
                    }
                }
            };

            #[cfg(feature = "output_progress")]
            progress_sender.tick(ProgressMsgUpdate::Set(String::from(
                "fetching policy version",
            )));
            let get_policy_version_output = client
                .get_policy_version()
                .policy_arn(policy_arn)
                .version_id(policy_id_arn_version.version())
                .send()
                .await
                .map_err(|error| {
                    #[cfg(feature = "error_reporting")]
                    let (aws_desc, aws_desc_span) = crate::item_specs::aws_error_desc!(&error);

                    IamPolicyError::PolicyVersionGetError {
                        policy_name: policy_name.clone(),
                        policy_path: policy_path.clone(),
                        #[cfg(feature = "error_reporting")]
                        aws_desc,
                        #[cfg(feature = "error_reporting")]
                        aws_desc_span,
                        error,
                    }
                })?;
            #[cfg(feature = "output_progress")]
            progress_sender.tick(ProgressMsgUpdate::Set(String::from(
                "policy version fetched",
            )));
            let policy_document = get_policy_version_output
                .policy_version()
                .and_then(|policy_version| policy_version.document())
                .map(|url_encoded_document| {
                    urlencoding::decode(url_encoded_document)
                        .map(|document| document.to_string())
                        .map_err(|error| IamPolicyError::PolicyDocumentNonUtf8 {
                            policy_name: policy_name.clone(),
                            policy_path: policy_path.clone(),
                            url_encoded_document: url_encoded_document.to_string(),
                            error,
                        })
                })
                .expect("Expected policy version document to exist.")?;

            // Hack: Remove this when referential param values is implemented.
            **data.managed_policy_arn_mut() = Some(ManagedPolicyArn::new(
                policy_id_arn_version.arn().to_string(),
            ));

            let state_current = IamPolicyState::Some {
                name: policy_name,
                path: policy_path,
                policy_document,
                policy_id_arn_version: Generated::Value(policy_id_arn_version),
            };

            Ok(state_current)
        } else {
            Ok(IamPolicyState::None)
        }
    }
}
