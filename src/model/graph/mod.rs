mod filter;
mod util;

use super::Attributes;
use super::Edge;
use super::Node;
use filter::find_by_attributes;
use std::collections::HashMap;
use std::option::Option;

#[derive(Default)]
pub struct Graph {
    nodes: HashMap<String, Node>,
    edges: HashMap<String, Edge>,
    relations: HashMap<String, HashMap<String, String>>,
}

impl Graph {
    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    pub fn get_node(&self, id: &str) -> Option<&Node> {
        self.nodes.get(id)
    }

    pub fn create_node(&mut self, attributes: Attributes) -> &Node {
        let n = Node::new(attributes);
        let node_id = n.id().clone();
        self.nodes.insert(node_id.clone(), n);
        self.get_node(&node_id).unwrap()
    }

    pub fn find_edges_by_attributes<F>(&self, key: &str, matcher: F) -> Vec<&Edge>
    where
        F: Fn(&&String) -> bool,
    {
        find_by_attributes(self.edges.values(), key, matcher)
    }

    pub fn find_nodes_by_attributes<F>(&self, key: &str, matcher: F) -> Vec<&Node>
    where
        F: Fn(&&String) -> bool,
    {
        find_by_attributes(self.nodes.values(), key, matcher)
    }
}

impl Graph {
    pub fn edge_count(&self) -> usize {
        self.edges.len()
    }

    pub fn get_edge(&self, id: &str) -> Option<&Edge> {
        self.edges.get(id)
    }

    pub fn create_edge(
        &mut self,
        relation: &str,
        attributes: Attributes,
        source_id: &str,
        destination_id: &str,
    ) -> &Edge {
        let e = Edge::new(
            String::from(relation),
            String::from(source_id),
            String::from(destination_id),
            attributes,
        );
        let edge_id = e.id().clone();
        self.edges.insert(edge_id.clone(), e);
        let edge = self.edges.get(&edge_id).unwrap();

        self.relations
            .entry(util::build_relation_key(source_id, destination_id))
            .or_insert_with(HashMap::new)
            .insert(String::from(relation), edge_id);

        edge
    }

    pub fn remove_edge(&mut self, edge_id: &str) {
        let removed_edge = self.edges.remove(edge_id).unwrap();
        let relation_key = util::build_relation_key(removed_edge.from_id(), removed_edge.to_id());
        let relations = self.relations.get_mut(&relation_key).unwrap();
        relations.remove(removed_edge.relation());
        if relations.is_empty() {
            self.relations.remove(&relation_key);
        }
    }

    pub fn has_any_relation(&self, source_id: &str, destination_id: &str) -> bool {
        self.relations
            .get(&util::build_relation_key(source_id, destination_id))
            .map(|n| -> bool { !n.is_empty() })
            .unwrap_or(false)
    }

    pub fn has_relation(&self, relation: &str, source_id: &str, destination_id: &str) -> bool {
        self.relations
            .get(&util::build_relation_key(source_id, destination_id))
            .map(|n| -> bool { n.contains_key(relation) })
            .unwrap_or(false)
    }
}
