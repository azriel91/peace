use std::fmt;

use peace::cfg::state::Generated;
use serde::{Deserialize, Serialize};

use crate::item_specs::peace_aws_instance_profile::model::InstanceProfileIdAndArn;

/// Instance profile state.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub enum InstanceProfileState {
    /// Instance profile does not exist.
    None,
    /// Instance profile exists.
    Some {
        /// Instance profile name.
        ///
        /// Alphanumeric characters and `_+=,.@-` are allowed.
        ///
        /// TODO: newtype + proc macro.
        name: String,
        /// String that begins and ends with a forward slash.
        ///
        /// Defaults to `/`.
        ///
        /// e.g. `/demo/`
        #[serde(default = "path_default")]
        path: String,
        /// The stable and unique IDs identifying the instance profile.
        instance_profile_id_and_arn: Generated<InstanceProfileIdAndArn>,
        /// Whether the role has been associated with the instance profile.
        role_associated: bool,
    },
}

fn path_default() -> String {
    String::from("/")
}

impl fmt::Display for InstanceProfileState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::None => "does not exist".fmt(f),
            Self::Some {
                name,
                path,
                instance_profile_id_and_arn,
                role_associated,
            } => {
                let instance_profile_exists = match instance_profile_id_and_arn {
                    Generated::Tbd => String::from("should exist"),
                    Generated::Value(instance_profile_id_and_arn) => {
                        let instance_profile_id = instance_profile_id_and_arn.id();
                        format!("exists with id {instance_profile_id}")
                    }
                };
                let role_associated = if *role_associated {
                    "associated with same named role"
                } else {
                    "but role not associated"
                };

                write!(
                    f,
                    "{path}{name} instance_profile {instance_profile_exists}, {role_associated}"
                )
            }
        }
    }
}
