pub mod graph {
    use std::collections::HashMap;

    pub mod graph_items {
        pub mod node {
            #[derive(PartialEq, Debug)]
            pub struct Node;
            impl Node {
                pub fn new(_: &str) -> Node {
                    unimplemented!();
                }
                pub fn with_attrs(&self, _: &[(&str, &str); 1]) -> Self {
                    unimplemented!();
                }
            }
        }
        pub mod edge {
            #[derive(PartialEq, Debug)]
            pub struct Edge;
            impl Edge {
                pub fn new(_: &str, _: &str) -> Self {
                    unimplemented!();
                }
                pub fn with_attrs(&self, _: &[(&str, &str); 1]) -> Self {
                    unimplemented!();
                }
            }
        }
    }
    pub struct Graph {
        pub nodes: Vec<graph_items::node::Node>,
        pub edges: Vec<graph_items::edge::Edge>,
        pub attrs: HashMap<String, String>
    }

    impl Graph {
        pub fn new() -> Self {
            Graph {
                nodes: Vec::new(),
                edges: Vec::new(),
                attrs: HashMap::new() }
        }

        pub fn with_edges(&self, _: &[graph_items::edge::Edge]) -> Self {
            unimplemented!();
        }

        pub fn with_nodes(&self, _: &[graph_items::node::Node]) -> Self {
            unimplemented!();
        }

        pub fn with_attrs(&self, _: &[(&str, &str)]) -> Self {
            unimplemented!();
        }
    }
}
