//! Peace framework web application lifecycle example
//!
//! This example demonstrates management of a web application's lifecycle. This
//! includes:
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
//! ## Initialize different deployment environments
//! ## Local development
//! app_cycle init dev --type development azriel91/web_app 0.1.1
//!
//! ## AWS -- defaults to reading ~/.aws/credentials
//! app_cycle init demo --type production azriel91/web_app 0.1.1
//!
//! ## Shows current environment
//! app_cycle profile
//!
//! app_cycle switch dev
//! app_cycle status
//! app_cycle desired
//! app_cycle diff
//! app_cycle deploy
//! ## make config changes on server / locally
//! app_cycle discover
//! app_cycle diff
//! app_cycle deploy # ensure compliance
//! app_cycle diff
//! app_cycle clean
//!
//! app_cycle switch demo
//! app_cycle status
//! app_cycle desired
//! app_cycle deploy
//! app_cycle clean
//!
//! ## `diff` defaults to current profile, current and desired state.
//! ## But we can tell it to diff between different profiles' current states.
//! app_cycle diff dev demo
//! ```

pub mod cmds;
pub mod flows;
pub mod item_specs;
pub mod model;
pub mod rt_model;
