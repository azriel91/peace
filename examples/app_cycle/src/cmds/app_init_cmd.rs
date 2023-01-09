use std::path::PathBuf;

use peace::rt_model::output::OutputWrite;
use peace_item_specs::{file_download::FileDownloadParams, tar_x::TarXParams};
use semver::Version;
use url::Url;

use crate::{
    flows::AppInitFlow,
    model::{AppCycleError, RepoSlug, WebAppFileId},
};

/// Takes app init parameters and runs the [`AppInitFlow`].
#[derive(Debug)]
pub struct AppInitCmd;

impl AppInitCmd {
    /// Takes app init parameters and runs the [`AppInitFlow`].
    ///
    /// # Parameters
    ///
    /// * `slug`: Username and repository of the application to download.
    /// * `semver`: Version of the application to download.
    pub async fn run<O>(
        output: &mut O,
        slug: RepoSlug,
        version: Version,
        url: Option<Url>,
    ) -> Result<(), AppCycleError>
    where
        O: OutputWrite<AppCycleError>,
    {
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
            let dest = web_app_download_dir;

            TarXParams::<WebAppFileId>::new(tar_path, dest)
        };

        AppInitFlow::run(output, web_app_file_download_params, web_app_tar_x_params).await
    }
}
