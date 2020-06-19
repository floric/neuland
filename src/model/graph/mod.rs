mod find;
mod util;

use super::{Attributes, Edge, Node};
use find::find_by_attributes;
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

    pub fn find_nodes_by_path(&self, path: Vec<&str>) -> Vec<&Node> {
        self.find_nodes_by_path_internal(self.nodes.values(), path)
    }

    fn find_nodes_by_path_internal<'a, I>(&self, nodes: I, path: Vec<&str>) -> Vec<&'a Node>
    where
        I: Iterator<Item = &'a Node>,
    {
        let x = path.split_first();
        if x.is_none() {
            return vec![];
        }

        let (relation, remaining_relations) = x.unwrap();

        nodes
            .filter(|x| {
                let found_edges = self.get_edges_from_node(x.id(), relation);
                if found_edges.is_empty() {
                    return false;
                } else if remaining_relations.is_empty() {
                    return true;
                }

                found_edges.iter().any(|e| {
                    let a = self.get_node(e.to_id()).unwrap();
                    !self
                        .find_nodes_by_path_internal(
                            vec![a.clone()].iter(),
                            Vec::from(remaining_relations),
                        )
                        .is_empty()
                })
            })
            .collect()
    }

    pub fn get_edges_from_node(&self, node_id: &str, relation: &str) -> Vec<&Edge> {
        self.edges
            .values()
            .filter(|e| e.relation() == relation && e.from_id() == node_id)
            .collect()
    }
}

impl Graph {
    pub fn edge_count(&self) -> usize {
        self.edges.len()
    }

    pub fn get_edge(&self, id: &str) -> Option<&Edge> {
        self.edges.get(id)
    }

    pub fn create_edge<'a>(
        &mut self,
        relation: &str,
        attributes: Attributes,
        source_id: &str,
        destination_id: &str,
    ) -> Result<String, &'a str> {
        if self.has_relation(relation, source_id, destination_id) {
            return Err("Edge already exists");
        }

        let e = Edge::new(
            String::from(relation),
            String::from(source_id),
            String::from(destination_id),
            attributes,
        );
        let edge_id = e.id().clone();
        self.edges.insert(edge_id.clone(), e);

        self.relations
            .entry(util::build_relation_key(source_id, destination_id))
            .or_insert_with(HashMap::new)
            .insert(String::from(relation), edge_id.clone());

        Ok(edge_id)
    }

    pub fn create_edges<'a>(
        &mut self,
        relation: &str,
        attributes: Attributes,
        source_id: &str,
        destination_ids: &Vec<&str>,
    ) -> Vec<Result<String, &'a str>> {
        destination_ids
            .iter()
            .map(|destination| {
                self.create_edge(relation, attributes.clone(), source_id, destination)
            })
            .collect()
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
