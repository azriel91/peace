//! Peace framework web application lifecycle example
//!
//! This example is designed to demonstrate the full lifecycle of a web
//! application. This includes:
//!
//! 1. Building the application.
//! 2. Starting / stopping the application in development.
//! 3. Deploying / upgrading / removing the application in test servers.
//! 4. Configuration management of the application.
//! 5. Deploying / upgrading / removing the application in live servers.
//! 6. Diffing the application and configuration across environments.
//! 7. Creating a replica environment from an existing environment.
//!
//! Example commands:
//!
//! ```bash
//! ## Download the application
//! web_app init azriel91/web_app 0.1.0
//!   ## windows:
//!   ## https://github.com/azriel91/web_app/releases/download/0.1.0/web_app.zip
//!   ## linux:
//!   ## https://github.com/azriel91/web_app/releases/download/0.1.0/web_app.tar.gz
//!
//! ## Initialize different deployment environments
//! ## Local development
//! web_app profile init \
//!   --name dev \
//!   --type development
//!
//! ## AWS -- defaults to reading ~/.aws/credentials
//! web_app profile init \
//!   --name demo \
//!   --type production
//!
//! ## Shows current environment
//! web_app profile
//!
//! web_app switch dev
//! web_app status
//! web_app desired
//! web_app diff
//! web_app push
//! ## make config changes on server / locally
//! web_app fetch
//! web_app diff
//! web_app push # ensure compliance
//! web_app fetch
//! web_app diff
//! web_app pull # store changes
//! web_app clean
//!
//! web_app switch demo
//! web_app status
//! web_app desired
//! web_app push
//! web_app clean
//!
//! ## `diff` defaults to current profile, current and desired state.
//! ## But we can tell it to diff between different profiles' current states.
//! web_app diff dev demo
//! ```

#[cfg(not(target_arch = "wasm32"))]
pub use crate::cli_args::{CliArgs, WebAppCommand};
pub use crate::{
    env_type::EnvType, env_type_parse_error::EnvTypeParseError, web_app_error::WebAppError,
};

mod web_app_error;

#[cfg(not(target_arch = "wasm32"))]
mod cli_args;
mod env_type;
mod env_type_parse_error;
