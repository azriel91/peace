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

pub use crate::web_app_error::WebAppError;

mod web_app_error;
