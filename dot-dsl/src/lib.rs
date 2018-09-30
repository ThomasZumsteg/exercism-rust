pub mod graph {
    use std::collections::HashMap;

    pub mod graph_items {
        pub mod node {
            use std::collections::HashMap;

            #[derive(PartialEq, Debug, Clone)]
            pub struct Node {
                name: String,
                attrs: HashMap<String, String>
            }
            impl Node {
                pub fn new(name: &str) -> Node {
                    Node { name: name.to_string(), attrs: HashMap::new() } 
                }
                pub fn with_attrs(mut self, attrs: &[(&str, &str); 1]) -> Self {
                    for &(name, value) in attrs {
                        self.attrs.insert(name.to_string(), value.to_string());
                    }
                    return self;
                }
            }
        }
        pub mod edge {
            #[derive(PartialEq, Debug, Clone)]
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
    #[derive(Clone)]
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

        pub fn with_nodes(mut self, nodes: &[graph_items::node::Node]) -> Self {
            for node in nodes {
                self.nodes.push(node.clone());
            }
            return self;
        }

        pub fn with_edges(&self, _: &[graph_items::edge::Edge]) -> Self {
            unimplemented!();
        }

        pub fn with_attrs(&self, _: &[(&str, &str)]) -> Self {
            unimplemented!();
        }
    }
}
