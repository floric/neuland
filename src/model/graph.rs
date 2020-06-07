use super::Attribute;
use super::Edge;
use super::Node;
use std::collections::HashMap;
use std::collections::HashSet;

pub struct Graph {
    nodes: HashMap<String, Node>,
    edges: HashMap<String, Edge>,
    connections: HashMap<String, String>,
}

impl Graph {
    pub fn new() -> Graph {
        Graph {
            nodes: HashMap::new(),
            edges: HashMap::new(),
            connections: HashMap::new(),
        }
    }

    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    pub fn get_edge(&self, id: &str) -> Option<&Edge> {
        self.edges.get(id)
    }

    pub fn get_node(&self, id: &str) -> Option<&Node> {
        self.nodes.get(id)
    }

    pub fn create_node(&mut self, attributes: HashSet<Attribute>) -> &Node {
        let n = Node::new(attributes);
        let node_id = n.id().clone();
        self.nodes.insert(n.id().clone(), n);
        self.get_node(&node_id).unwrap()
    }

    pub fn create_edge(
        &mut self,
        attributes: HashSet<Attribute>,
        source_id: &str,
        destination_id: &str,
    ) -> &Edge {
        let e = Edge::new(
            String::from(source_id),
            String::from(destination_id),
            attributes,
        );
        let edge_id = e.id().clone();
        self.edges.insert(e.id().clone(), e);

        let edge = self.edges.get(&edge_id).unwrap();

        self.connections
            .insert(format!("{}-{}", source_id, destination_id), edge_id);

        edge
    }
}
