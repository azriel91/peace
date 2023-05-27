use dioxus::prelude::*;

pub fn Home(cx: Scope) -> Element {
    cx.render(rsx!(
        head {}
        body {
            section {
                class: "p-10",
                "Home page"
            }
        }
    ))
}
