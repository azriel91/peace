use peace::{
    cfg::{profile, Profile},
    rt_model::output::OutputWrite,
};
use semver::Version;
use url::Url;

use crate::model::{AppCycleError, EnvType, RepoSlug};

use super::ProfileInitCmd;

/// Default development profile.
const DEV_PROFILE: Profile = profile!("dev");

/// Takes app init parameters and runs the [`AppInitFlow`].
#[derive(Debug)]
pub struct AppInitCmd;

impl AppInitCmd {
    /// Takes app init parameters and runs the [`AppInitFlow`].
    ///
    /// # Parameters
    ///
    /// * `output`: Output to write the execution outcome.
    /// * `slug`: Username and repository of the application to download.
    /// * `version`: Version of the application to download.
    /// * `url`: URL to override where to download the application from.
    pub async fn run<O>(
        output: &mut O,
        slug: &RepoSlug,
        version: &Version,
        url: Option<Url>,
    ) -> Result<(), AppCycleError>
    where
        O: OutputWrite<AppCycleError>,
    {
        ProfileInitCmd::run(
            output,
            DEV_PROFILE,
            EnvType::Development,
            slug,
            version,
            url,
        )
        .await?;

        Ok(())
    }
}
