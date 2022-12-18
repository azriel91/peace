use std::path::PathBuf;

use peace::rt_model::OutputWrite;
use peace_item_specs::file_download::FileDownloadParams;
use semver::Version;
use url::Url;

use crate::{
    flows::AppInitFlow,
    model::{RepoSlug, AppCycleError},
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
    pub async fn run<O>(output: &mut O, slug: RepoSlug, version: Version) -> Result<(), AppCycleError>
    where
        O: OutputWrite<AppCycleError>,
    {
        let app_cycle_file_download_params = {
            let account = slug.account();
            let repo_name = slug.repo_name();
            #[cfg(target_family = "windows")]
            let file_ext = "zip";
            #[cfg(any(target_family = "unix", target_family = "wasm"))]
            let file_ext = "tar.gz";
            // windows:
            // https://github.com/azriel91/web_app/releases/download/0.1.0/app_cycle.zip
            //
            // linux:
            // https://github.com/azriel91/web_app/releases/download/0.1.0/app_cycle.tar.gz
            let src = {
                let url_candidate = format!(
                    "https://github.com/{account}/{repo_name}/releases/download/{version}/{repo_name}.{file_ext}"
                );
                Url::parse(&url_candidate).map_err(|error| AppCycleError::AppCycleUrlBuild {
                    url_candidate,
                    error,
                })?
            };
            let dest = PathBuf::from_iter([
                account,
                repo_name,
                &format!("{version}"),
                &format!("{repo_name}.{file_ext}"),
            ]);
            FileDownloadParams::new(src, dest)
        };
        AppInitFlow::run(output, app_cycle_file_download_params).await
    }
}
