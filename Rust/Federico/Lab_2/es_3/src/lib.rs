pub mod graph {
    use std::collections::HashMap;
    use graph_items::edge::Edge;
    use graph_items::node::Node;


    pub struct Graph{
        pub attrs: HashMap<String, String>,
        pub nodes: Vec<Node>,
        pub edges: Vec<Edge>
    }

    impl Graph {
        pub fn new() -> Self {
            Self{
                attrs: HashMap::new(),
                nodes: vec![],
                edges: vec![]
            }
        }
        pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
            for (key, value ) in attrs{
                self.attrs.insert(key.to_string(), value.to_string());
            }
            self
        }

        pub fn with_nodes(mut self, nodes: &[Node]) -> Self {
            self.nodes.extend_from_slice(nodes);
            self
        }

        pub fn with_edges(mut self, edges: &[Edge]) -> Self {
            self.edges.extend_from_slice(edges);
            self
        }

        pub fn get_attr(&self, key: &str) -> Option<&str> {
            match self.attrs.get(key){
                Some(val) => {
                    Some(val.as_str())
                }
                None => {
                    None
                }
            }
        }

        pub fn get_node(&self, node_name: &str) -> Option<&Node> {
            for (i, node) in self.nodes.iter().enumerate(){
                if node.name == node_name {
                    return  self.nodes.get(i);
                }
            }
            None
        }

    }


    pub mod graph_items {
        pub mod edge {

            use std::collections::HashMap;

            #[derive(Clone, Debug, PartialEq)]
            pub struct Edge{
                pub attrs: HashMap<String, String>,
                pub node_1: String,
                pub node_2: String
            }

            impl Edge {
                pub fn new(node_1_name: &str, node_2_name: &str) -> Self {
                    Self{
                        attrs: HashMap::new(),
                        node_1: node_1_name.to_string(),
                        node_2: node_2_name.to_string()
                    }
                }
                pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
                    for (key, value ) in attrs{
                        self.attrs.insert(key.to_string(), value.to_string());
                    }
                    self
                }

                pub fn get_attr(&self, key: &str) -> Option<&str> {
                    match self.attrs.get(key){
                        Some(val) => {
                            Some(val.as_str())
                        }
                        None => {
                            None
                        }
                    }
                }

            }


        }

        pub mod node {
            use std::collections::HashMap;

            #[derive(Clone, Debug, PartialEq)]
            pub struct Node{
                pub attrs: HashMap<String, String>,
                pub name: String
            }

            impl Node {

                pub fn new(name: &str) -> Self {
                    Self {
                        attrs: HashMap::new(),
                        name: name.to_string()
                    }
                }

                pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
                    for (key, value ) in attrs{
                        self.attrs.insert(key.to_string(), value.to_string());
                    }
                    self
                }

                pub fn get_attr(&self, key: &str) -> Option<&str> {
                    match self.attrs.get(key){
                        Some(val) => {
                            Some(val.as_str())
                        }
                        None => {
                            None
                        }
                    }
                }

            }

        }
    }


}
