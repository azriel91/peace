use std::path::PathBuf;

use peace::{
    cfg::{flow_id, item_spec_id, FlowId, ItemSpecId},
    rt_model::{Flow, ItemSpecGraphBuilder},
};
use peace_item_specs::{
    file_download::{FileDownloadItemSpec, FileDownloadParams},
    tar_x::{TarXItemSpec, TarXParams},
};
use semver::Version;
use url::Url;

use crate::model::{AppCycleError, RepoSlug, WebAppFileId};

/// Flow to deploy a web application.
#[derive(Debug)]
pub struct EnvDeployFlow;

impl EnvDeployFlow {
    /// Returns the `Flow` graph.
    pub async fn flow() -> Result<Flow<AppCycleError>, AppCycleError> {
        let flow = {
            let flow_id = flow_id!("env_deploy");
            let graph = {
                let mut graph_builder = ItemSpecGraphBuilder::<AppCycleError>::new();

                let web_app_download_id = graph_builder.add_fn(
                    FileDownloadItemSpec::<WebAppFileId>::new(item_spec_id!("web_app_download"))
                        .into(),
                );
                let web_app_extract_id = graph_builder.add_fn(
                    TarXItemSpec::<WebAppFileId>::new(item_spec_id!("web_app_extract")).into(),
                );

                graph_builder.add_edge(web_app_download_id, web_app_extract_id)?;
                graph_builder.build()
            };

            Flow::new(flow_id, graph)
        };

        Ok(flow)
    }

    /// Returns the params needed for this flow.
    pub fn params(
        slug: &RepoSlug,
        version: &Version,
        url: Option<Url>,
    ) -> Result<(FileDownloadParams<WebAppFileId>, TarXParams<WebAppFileId>), AppCycleError> {
        let account = slug.account();
        let repo_name = slug.repo_name();
        let web_app_download_dir = PathBuf::from_iter([account, repo_name, &format!("{version}")]);

        let web_app_file_download_params = {
            #[cfg(target_family = "windows")]
            let file_ext = "zip";
            #[cfg(any(target_family = "unix", target_family = "wasm"))]
            let file_ext = "tar";
            // windows:
            // https://github.com/azriel91/web_app/releases/download/0.1.0/web_app.zip
            //
            // linux:
            // https://github.com/azriel91/web_app/releases/download/0.1.0/web_app.tar
            let src = {
                match url {
                    Some(url) => url,
                    None => {
                        let url_candidate = format!(
                            "https://github.com/{account}/{repo_name}/releases/download/{version}/{repo_name}.{file_ext}"
                        );
                        Url::parse(&url_candidate).map_err(|error| {
                            AppCycleError::AppCycleUrlBuild {
                                url_candidate,
                                error,
                            }
                        })?
                    }
                }
            };
            let dest = web_app_download_dir.join(format!("{repo_name}.{file_ext}"));
            FileDownloadParams::new(
                src,
                dest,
                #[cfg(target_arch = "wasm32")]
                peace_item_specs::file_download::StorageForm::Base64,
            )
        };
        let web_app_tar_x_params = {
            let tar_path = web_app_file_download_params.dest().to_path_buf();
            let dest = web_app_download_dir.join("extracted");

            TarXParams::<WebAppFileId>::new(tar_path, dest)
        };

        Ok((web_app_file_download_params, web_app_tar_x_params))
    }
}
