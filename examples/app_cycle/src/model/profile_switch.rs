use peace::cfg::Profile;

use crate::model::EnvType;

/// Whether to switch to an existing profile or to a new one.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ProfileSwitch {
    /// Switch to an existing profile.
    ToExisting {
        /// The existing profile to switch to.
        profile: Profile,
    },
    /// Switch to a newly created profile.
    CreateNew {
        /// The existing profile to switch to.
        profile: Profile,
        /// Type of environment: development or production.
        env_type: EnvType,
    },
}
