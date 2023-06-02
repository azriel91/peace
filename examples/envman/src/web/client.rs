cfg_if::cfg_if! {
    if #[cfg(feature = "hydrate")] {
        use wasm_bindgen::prelude::wasm_bindgen;
        use leptos::*;

        // use crate::{flows::AppUploadFlow, web::components::Home};

        #[wasm_bindgen]
        pub async fn hydrate() {
            // initializes logging using the `log` crate
            let _log = console_log::init_with_level(log::Level::Debug);
            console_error_panic_hook::set_once();

            leptos::mount_to_body(move |cx| {
                view! {
                    cx,
                    <div>rara</div>
                }
            });
        }
    }
}
