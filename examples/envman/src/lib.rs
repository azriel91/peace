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
//! envman init dev --type development azriel91/web_app 0.1.1
//!
//! ## AWS -- defaults to reading ~/.aws/credentials
//! envman init demo --type production azriel91/web_app 0.1.1
//!
//! ## Shows current environment
//! envman profile
//!
//! envman switch dev
//! envman status
//! envman goal
//! envman diff
//! envman deploy
//! ## make config changes on server / locally
//! envman discover
//! envman diff
//! envman deploy # ensure compliance
//! envman diff
//! envman clean
//!
//! envman switch demo
//! envman status
//! envman goal
//! envman deploy
//! envman clean
//!
//! ## `diff` defaults to current profile, current and goal state.
//! ## But we can tell it to diff between different profiles' current states.
//! envman diff dev demo
//! ```

cfg_if::cfg_if! {
    if #[cfg(feature = "flow_logic")] {
        pub mod cmds;
        pub mod flows;
        pub mod items;
        pub mod model;
        pub mod output;
        pub mod rt_model;
    }
}

cfg_if::cfg_if! {
    if #[cfg(feature = "web_components")] {
        pub mod web;
    }
}

cfg_if::cfg_if! {
    if #[cfg(all(feature = "web_components", feature = "hydrate"))] {
        use wasm_bindgen::prelude::wasm_bindgen;
        use leptos::*;

        use peace::webi_components::Home;

        #[wasm_bindgen]
        pub async fn hydrate() {

            // initializes logging using the `log` crate
            let _log = console_log::init_with_level(log::Level::Debug);
            console_error_panic_hook::set_once();

            leptos::mount_to_body(move || {
                view! {
                    <Home />
                }
            });
        }
    }
}
