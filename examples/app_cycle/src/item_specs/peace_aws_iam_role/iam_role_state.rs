use std::fmt;

use peace::cfg::state::Generated;
use serde::{Deserialize, Serialize};

use crate::item_specs::peace_aws_iam_role::model::{ManagedPolicyAttachment, RoleIdAndArn};

/// IAM role state.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub enum IamRoleState {
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
        /// The stable and unique IDs identifying the role.
        role_id_and_arn: Generated<RoleIdAndArn>,
        /// Managed policy to attach to the role.
        managed_policy_attachment: ManagedPolicyAttachment,
    },
}

fn path_default() -> String {
    String::from("/")
}

impl fmt::Display for IamRoleState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::None => "does not exist".fmt(f),
            Self::Some {
                name,
                path,
                role_id_and_arn,
                managed_policy_attachment,
            } => {
                let role_exists = match role_id_and_arn {
                    Generated::Tbd => String::from("should exist"),
                    Generated::Value(role_id_and_arn) => {
                        let role_id = role_id_and_arn.id();
                        format!("exists with id {role_id}")
                    }
                };
                let managed_policy_attached = if managed_policy_attachment.attached() {
                    "attached with same named managed policy"
                } else {
                    "but managed policy not attached"
                };

                write!(
                    f,
                    "{path}{name} role {role_exists}, {managed_policy_attached}"
                )
            }
        }
    }
}
