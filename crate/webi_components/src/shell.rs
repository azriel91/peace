use leptos::{
    hydration::{AutoReload, HydrationScripts},
    prelude::{ElementChild, GlobalAttributes, IntoView, LeptosOptions},
    view,
};
use leptos_meta::{MetaTags, Stylesheet, Title};

use crate::{App, ChildrenFn};

/// Main shell for server side rendered app.
///
/// # Parameters
///
/// * `app_name`: The name that `leptos` will compile the server side binary to.
///   Usually the crate name, but may be changed by the `output-name` key in
///   `Cargo.toml`.
/// * `options`: The `LeptosOptions` from
///   `leptos::prelude::get_configuration(None)?.leptos_options`.
pub fn Shell(app_name: String, options: LeptosOptions, app_home: ChildrenFn) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <AutoReload options=options.clone() />
                <HydrationScripts options />
                <MetaTags />

                <Title text=format!("{app_name} • peace")/>

                // injects a stylesheet into the document <head>
                // id=leptos means cargo-leptos will hot-reload this stylesheet
                <Stylesheet id="leptos" href=format!("/pkg/{app_name}.css") />
            </head>
            <body>
                <App app_home />
            </body>
        </html>
    }
}
