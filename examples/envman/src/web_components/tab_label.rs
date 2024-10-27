use leptos::{component, ev::Event, html, view, Callable, Callback, IntoView, NodeRef};

/// The label that users click to switch to that tab.
#[component]
pub fn TabLabel(
    tab_group_name: &'static str,
    tab_id: &'static str,
    #[prop(default = "")] label: &'static str,
    #[prop(default = "")] class: &'static str,
    #[prop(default = false)] checked: bool,
    #[prop(into, optional, default = None)] on_change: Option<Callback<Event>>,
    #[prop(into, optional, default = leptos::create_node_ref::<html::Input>())] node_ref: NodeRef<
        html::Input,
    >,
) -> impl IntoView {
    let tab_classes = format!(
        "\
        peer/{tab_id} \
        hidden \
        "
    );

    let label_classes = format!(
        "\
        {class} \
        \
        inline-block \
        h-7 \
        lg:h-9 \
        px-2.5 \
        \
        cursor-pointer \
        border-t \
        border-x \
        border-slate-400 \
        \
        peer-checked/{tab_id}:border-t-4 \
        peer-checked/{tab_id}:border-t-blue-500 \
    "
    );

    #[cfg(not(target_arch = "wasm32"))]
    let _node_ref = node_ref;

    view! {
        <input
            type="radio"
            name=tab_group_name
            id=tab_id
            class=tab_classes
            checked=checked
            on:change={move |ev| { if let Some(on_change) = on_change { on_change.call(ev) } }}
            node_ref=node_ref
        />
        <label for=tab_id class=label_classes>
            {label}
        </label>
    }
}
