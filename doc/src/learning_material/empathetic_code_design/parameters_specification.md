# Parameters Specification

Specify *where to get the value* for each item spec input. The value may not necessarily exist until the flow is executed.

```dot process
digraph {
    graph [
        margin    = 0.0
        penwidth  = 0
        nodesep   = 0.0
        ranksep   = 0.02
        bgcolor   = "transparent"
        fontname  = "helvetica"
        fontcolor = "#7f7f7f"
        rankdir   = LR
        // splines   = curved
    ]
    node [
        fontcolor = "#111111"
        fontname  = "monospace"
        fontsize  = 12
        shape     = "circle"
        style     = "filled"
        width     = 0.3
        height    = 0.3
        margin    = 0.04
        color     = "#9999aa"
        fillcolor = "#ddddf5"
    ]
    edge [
        arrowsize = 0.7
        color     = "#7f7f7f"
        fontcolor = "#7f7f7f"
    ]

    subgraph cluster_a {
        a [label = <<b>a</b>>]
        a_text [shape="plain" style="" fontcolor="#7f7f7f" label = <file<br/>download>]

        subgraph cluster_a_params {
            a_params_src [
                label     = "src"
                margin    = 0.0
                fontsize  = 8
                width     = 0.36
                height    = 0.19
                shape     = "rectangle"
                style     = "filled,rounded"
                fillcolor = "#ffaaff"
                color     = "#773377"
            ]

            a_params_dest [
                label     = "dest"
                margin    = 0.0
                fontsize  = 8
                width     = 0.36
                height    = 0.19
                shape     = "rectangle"
                style     = "filled,rounded"
                fillcolor = "#ffaaff"
                color     = "#773377"
            ]

            a_state [
                label     = <file_state<br align="left"/>&nbsp;&nbsp;..<br align="left"/>>
                margin    = 0.05
                fontsize  = 8
                shape     = "rectangle"
                style     = "filled,rounded"
                fillcolor = "#ffffaa"
                color     = "#667722"
            ]

        }

        a_params_src -> a_state [style = invis]
        a_params_dest -> a_state [style = invis]
        a -> a_state [style = invis]
    }

    subgraph cluster_b {
        b [label = <<b>b</b>>]
        b_text [shape="plain" style="" fontcolor="#7f7f7f" label = <server<br/>instance>]

        subgraph cluster_b_params {
            b_params_image [
                label     = "image"
                margin    = 0.0
                fontsize  = 8
                width     = 0.36
                height    = 0.19
                shape     = "rectangle"
                style     = "filled,rounded"
                fillcolor = "#ffaaff"
                color     = "#773377"
            ]

            b_params_size [
                label     = "size"
                margin    = 0.0
                fontsize  = 8
                width     = 0.36
                height    = 0.19
                shape     = "rectangle"
                style     = "filled,rounded"
                fillcolor = "#ffaaff"
                color     = "#773377"
            ]

            b_state [
                label     = <server<br align="left"/>&nbsp;&nbsp;..<br align="left"/>>
                margin    = 0.05
                fontsize  = 8
                shape     = "rectangle"
                style     = "filled,rounded"
                fillcolor = "#ffffaa"
                color     = "#667722"
            ]
        }

        b_params_image -> b_state [style = invis]
        b_params_size -> b_state [style = invis]
        b -> b_state [style = invis]
    }

    subgraph cluster_c {
        c [label = <<b>c</b>>]
        c_text [shape="plain" style="" fontcolor="#7f7f7f" label = <file<br/>upload>]

        subgraph cluster_c_params {
            c_params_src_path [
                label     = "src"
                margin    = 0.0
                fontsize  = 8
                width     = 0.36
                height    = 0.19
                shape     = "rectangle"
                style     = "filled,rounded"
                fillcolor = "#ffaaff"
                color     = "#773377"
            ]

            c_params_dest_ip [
                label     = "ip"
                margin    = 0.0
                fontsize  = 8
                width     = 0.36
                height    = 0.19
                shape     = "rectangle"
                style     = "filled,rounded"
                fillcolor = "#ffaaff"
                color     = "#773377"
            ]

            c_params_dest_path [
                label     = "path"
                margin    = 0.0
                fontsize  = 8
                width     = 0.36
                height    = 0.19
                shape     = "rectangle"
                style     = "filled,rounded"
                fillcolor = "#ffaaff"
                color     = "#773377"
            ]

            c_state [
                label     = <remote_file<br align="left"/>&nbsp;&nbsp;..<br align="left"/>>
                margin    = 0.05
                fontsize  = 8
                shape     = "rectangle"
                style     = "filled,rounded"
                fillcolor = "#ffffaa"
                color     = "#667722"
            ]
        }

        c_params_src_path -> c_state [style = invis]
        c_params_dest_ip -> c_state [style = invis]
        c_params_dest_path -> c_state [style = invis]
        c -> c_state [style = invis]
    }

    a -> b [minlen = 9, weight = 5]
    b -> c [minlen = 9, weight = 5]

    subgraph params {
        label = "Pre-execution\nknown values"
        cluster = false

        node [
            shape    = none
            fontsize = 8
            margin   = 0.05
            height    = 0.1
            style    = ""
        ]
        edge [
            color     = "#4455ff"
            arrowhead = "vee"
            style     = dashed
        ]

        web_app_url;
        web_app_path;
        instance_image;
        instance_size;
        app_upload_path;

        web_app_url -> a_params_src [constraint = true]
        web_app_path -> a_params_dest [constraint = true]

        {
            edge [
                color     = "#cc7744"
            ]
            instance_image -> b_params_image [constraint = true, minlen = 10]
            instance_size -> b_params_size [constraint = true, minlen = 10]
        }

        {
            edge [
                color     = "#449988"
            ]
            a_state -> c_params_src_path [constraint = false, style = invis]
            b_state -> c_params_dest_ip [constraint = true]
        }
        web_app_path -> c_params_src_path [constraint = true, color = "#44dd77", minlen = 19]
        app_upload_path -> c_params_dest_path [constraint = true, color = "#44dd77", minlen = 19]
    }
}
```

```rust ,ignore
let file_download_params = FileDownloadParams::<WebApp> {
    src: Url::parse("https://example.com/web_app.tar")?,
    dest: PathBuf::from("/tmp/path/to/app.tar"),
    marker: PhantomData,
};

let server_instance_params = ServerInstanceParams::<WebApp> {
    image: ImageId::new("img-12345"),
    size: InstSize::XLarge,
    marker: PhantomData,
};

let file_upload_params = FileUploadParams::<WebApp> {
    src: PathBuf::from("/tmp/path/to/app.tar")
    dest_path: PathBuf::from("/opt/peace/demo/app")
    dest_ip: !?, /* IpAddr: get the server IP from `b` */
};
```

## Deferred Values

```rust ,ignore
let file_upload_params_spec = FileUploadParamsSpec::<WebApp>::builder()
    .with_src(PathBuf::from("/tmp/path/to/app.tar"))
    .with_path(PathBuf::from("/opt/peace/demo/app"))
    .with_ip_from_map(|server: &Server<WebApp>| {
        *server.ip() // type safe!
    })
    .build();
#
# let file_download_params_spec = file_download_params.into();
# let server_instance_params_spec = server_instance_params.into();
```

---

### Command Context

```rust ,ignore
let mut cmd_context = CmdContext::builder()
    .with_flow(&flow);
    .with_item_spec_params::<FileDownloadItemSpec::<WebApp>>(
        item_spec_id!("a"),
        file_download_params.into(),
    )
    .with_item_spec_params::<ServerInstanceParams::<WebApp>>(
        item_spec_id!("b"),
        server_instance_params.into(),
    )
    .with_item_spec_params::<FileUploadParams::<WebApp>>(
        item_spec_id!("c"),
        file_upload_params_spec,
    )
    .build();
```
