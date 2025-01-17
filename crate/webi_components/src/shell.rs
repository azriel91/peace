use leptos::{
    hydration::{AutoReload, HydrationScripts},
    prelude::{ElementChild, GlobalAttributes, IntoView, LeptosOptions},
    view,
};
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};

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
    // Provides context that manages stylesheets, titles, meta tags, etc.
    //
    // Normally this is in the `App` component, but for `peace_webi_components`, we
    // also include it in the `Shell` because the `Title` component calls
    // `leptos_meta::use_head()`, which logs a debug message about the meta context
    // not being provided.
    provide_meta_context();

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
