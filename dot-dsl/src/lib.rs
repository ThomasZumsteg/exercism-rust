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
            use std::collections::HashMap;

            #[derive(PartialEq, Debug, Clone)]
            pub struct Edge {
                from: String,
                to: String,
                attrs: HashMap<String, String>
            }
            impl Edge {
                pub fn new(from: &str, to: &str) -> Self {
                    return Edge {
                        from: from.to_string(),
                        to: to.to_string(),
                        attrs: HashMap::new() };
                }
                pub fn with_attrs(mut self, attrs: &[(&str, &str); 1]) -> Self {
                    for &(key, val) in attrs {
                        self.attrs.insert(key.to_string(), val.to_string());
                    }
                    return self;
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

        pub fn with_edges(mut self, edges: &[graph_items::edge::Edge]) -> Self {
            for edge in edges {
                self.edges.push(edge.clone());
            }
            return self;
        }

        pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
            for &(key, val) in attrs {
                self.attrs.insert(key.to_string(), val.to_string());
            }
            return self;
        }
    }
}
