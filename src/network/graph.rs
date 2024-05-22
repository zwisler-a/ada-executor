use std::collections::HashMap;
use uuid::Uuid;
use crate::network::node::Node;

struct Graph {
    nodes: HashMap<Uuid, Node>,
}

impl Graph {
    fn new() -> Self {
        Graph {
            nodes: HashMap::new(),
        }
    }

    fn add_node<F>(&mut self, node: Node)
        where
            F: Fn() + 'static + Send + Sync,
    {
        self.nodes.insert(node.id, node);
    }
}