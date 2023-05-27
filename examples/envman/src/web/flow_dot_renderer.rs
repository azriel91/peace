use peace::{cfg::ItemId, rt_model::Flow};

/// Renders a `Flow` as a GraphViz Dot diagram.
///
/// This is currently a mashed together implementation. A proper implementation
/// would require investigation into:
///
/// * Whether colours and border sizes should be part of a theme object that we
///   take in.
/// * Whether colours and border sizes can be configured through CSS classes,
///   and within the generated dot source, we only apply those classes.
#[derive(Debug)]
pub struct FlowDotRenderer {
    edge_color: &'static str,
    node_text_color: &'static str,
    plain_text_color: &'static str,
}

impl FlowDotRenderer {
    pub fn new() -> Self {
        Self {
            edge_color: "#7f7f7f",
            node_text_color: "#111111",
            plain_text_color: "#7f7f7f",
        }
    }

    pub fn dot<E>(&self, flow: &Flow<E>) -> String
    where
        E: 'static,
    {
        let graph_attrs = self.graph_attrs();
        let node_attrs = self.node_attrs();
        let edge_attrs = self.edge_attrs();

        let item_clusters = flow
            .graph()
            .iter()
            .map(|item| self.item_cluster(item.id()))
            .collect::<Vec<String>>()
            .join("\n");

        let edges = flow
            .graph()
            .raw_edges()
            .iter()
            .map(|edge| {
                let src_item = &flow.graph()[edge.source()];
                let src_item_id = src_item.id();
                let target_item = &flow.graph()[edge.target()];
                let target_item_id = target_item.id();
                self.edge(src_item_id, target_item_id)
            })
            .collect::<Vec<String>>()
            .join("\n");

        format!(
            "digraph G {{
                {graph_attrs}
                {node_attrs}
                {edge_attrs}

                {item_clusters}

                {edges}
            }}"
        )
    }

    fn graph_attrs(&self) -> String {
        let plain_text_color = self.plain_text_color;
        format!(
            "\
                graph [\n\
                    margin    = 0.0\n\
                    penwidth  = 0\n\
                    nodesep   = 0.0\n\
                    ranksep   = 0.02\n\
                    bgcolor   = \"transparent\"\n\
                    fontname  = \"helvetica\"\n\
                    fontcolor = \"{plain_text_color}\"\n\
                    rankdir   = LR\n\
                ]\n\
            "
        )
    }

    fn node_attrs(&self) -> String {
        let node_text_color = self.node_text_color;
        format!(
            "\
                node [\n\
                    fontcolor = \"{node_text_color}\"\n\
                    fontname  = \"monospace\"\n\
                    fontsize  = 12\n\
                    shape     = \"circle\"\n\
                    style     = \"filled\"\n\
                    width     = 0.3\n\
                    height    = 0.3\n\
                    margin    = 0.04\n\
                    color     = \"#9999aa\"\n\
                    fillcolor = \"#ddddf5\"\n\
                ]\n\
            "
        )
    }

    fn edge_attrs(&self) -> String {
        let edge_color = self.edge_color;
        let plain_text_color = self.plain_text_color;
        format!(
            "\
                edge [\n\
                    arrowsize = 0.7\n\
                    color     = \"{edge_color}\"\n\
                    fontcolor = \"{plain_text_color}\"\n\
                ]\n\
            "
        )
    }

    fn item_cluster(&self, item_id: &ItemId) -> String {
        let plain_text_color = self.plain_text_color;
        format!(
            r#"
            subgraph cluster_{item_id} {{
                {item_id} [label = ""]
                {item_id}_text [
                    shape="plain"
                    style=""\
                    fontcolor="{plain_text_color}"
                    label = <<table
                        border="0"
                        cellborder="0"
                        cellpadding="0">
                        <tr>
                            <td><font point-size="15">📥</font></td>
                            <td balign="left">{item_id}</td>
                        </tr>
                    </table>>
                ]
            }}
        "#
        )
    }

    fn edge(&self, src_item_id: &ItemId, target_item_id: &ItemId) -> String {
        format!(r#"{src_item_id} -> {target_item_id} [minlen = 9]"#)
    }
}
