use std::{
    future::{Future, IntoFuture},
    pin::Pin,
};

use peace::{
    cfg::Profile,
    resources::resources::ts::SetUp,
    rt_model::{output::OutputWrite, ItemSpecGraph, Workspace},
};
use peace_item_specs::{file_download::FileDownloadParams, tar_x::TarXParams};

use crate::{
    model::{AppCycleError, EnvType, WebAppFileId},
    rt_model::AppCycleCmdContext,
};

/// Builds a command context for the `app_cycle` example.
///
/// This registers the types for workspace, profile, and flow params.
#[derive(Debug)]
pub struct CmdContextBuilder<'ctx, O> {
    workspace: &'ctx Workspace,
    graph: &'ctx ItemSpecGraph<AppCycleError>,
    output: &'ctx mut O,
    web_app_file_download_params: Option<FileDownloadParams<WebAppFileId>>,
    web_app_tar_x_params: Option<TarXParams<WebAppFileId>>,
    profile_selection: ProfileSelection,
    env_type: Option<EnvType>,
}

impl<'ctx, O> CmdContextBuilder<'ctx, O>
where
    O: OutputWrite<AppCycleError>,
{
    pub fn new(
        workspace: &'ctx Workspace,
        graph: &'ctx ItemSpecGraph<AppCycleError>,
        output: &'ctx mut O,
    ) -> Self {
        Self {
            workspace,
            graph,
            output,
            web_app_file_download_params: None,
            web_app_tar_x_params: None,
            profile_selection: ProfileSelection::None,
            env_type: None,
        }
    }

    pub fn with_web_app_file_download_params(
        mut self,
        web_app_file_download_params: FileDownloadParams<WebAppFileId>,
    ) -> Self {
        self.web_app_file_download_params = Some(web_app_file_download_params);
        self
    }

    pub fn with_web_app_tar_x_params(
        mut self,
        web_app_tar_x_params: TarXParams<WebAppFileId>,
    ) -> Self {
        self.web_app_tar_x_params = Some(web_app_tar_x_params);
        self
    }

    pub fn with_profile(mut self, profile: Profile) -> Self {
        self.profile_selection = ProfileSelection::Selected(profile);
        self
    }

    pub fn with_profile_from_last_used(mut self) -> Self {
        self.profile_selection = ProfileSelection::FromWorkspaceParams;
        self
    }

    pub fn with_env_type(mut self, env_type: EnvType) -> Self {
        self.env_type = Some(env_type);
        self
    }

    /// Creates the `CmdContext`.
    pub async fn build(self) -> Result<AppCycleCmdContext<'ctx, O, SetUp>, AppCycleError> {
        let CmdContextBuilder {
            workspace,
            graph,
            output,
            web_app_file_download_params,
            web_app_tar_x_params,
            profile_selection,
            env_type,
        } = self;

        let cmd_context_builder =
            peace::rt_model::cmd::CmdContext::builder(workspace, graph, output)
                .with_workspace_param(
                    "web_app_file_download_params".to_string(),
                    web_app_file_download_params,
                )
                .with_workspace_param("web_app_tar_x_params".to_string(), web_app_tar_x_params)
                .with_profile_param("env_type".to_string(), env_type);

        // Profile is a workspace param, as it tells the command context which profile
        // to use.
        match profile_selection {
            ProfileSelection::None => {
                cmd_context_builder
                    .with_workspace_param("profile".to_string(), None::<Profile>)
                    .await
            }
            ProfileSelection::Selected(profile) => {
                cmd_context_builder
                    .with_workspace_param("profile".to_string(), Some(profile.clone()))
                    .with_profile(profile)
                    .await
            }
            ProfileSelection::FromWorkspaceParams => {
                cmd_context_builder
                    .with_workspace_param("profile".to_string(), None::<Profile>)
                    .with_profile_from_workspace_params("profile".to_string())
                    .await
            }
        }
    }
}

/// Future that returns the `CmdContext`.
///
/// This is boxed since [TAIT] is not yet available.
///
/// [TAIT]: https://rust-lang.github.io/impl-trait-initiative/explainer/tait.html
pub type CmdContextFuture<'ctx, O> =
    Pin<Box<dyn Future<Output = Result<AppCycleCmdContext<'ctx, O, SetUp>, AppCycleError>> + 'ctx>>;

impl<'ctx, O> IntoFuture for CmdContextBuilder<'ctx, O>
where
    O: OutputWrite<AppCycleError>,
{
    type IntoFuture = CmdContextFuture<'ctx, O>;
    type Output = <Self::IntoFuture as Future>::Output;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.build())
    }
}

#[derive(Debug)]
enum ProfileSelection {
    None,
    Selected(Profile),
    FromWorkspaceParams,
}
