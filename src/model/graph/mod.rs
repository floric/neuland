mod find;
mod util;

use crate::query::matcher::Matcher;

use super::{attributes::HasAttributes, Attributes, Edge, Node, Query};
use array_tool::vec::Intersect;
use find::{find_by_attributes, find_nodes_by_path_internal};
use nanoid::nanoid;
use std::collections::HashMap;

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

    pub fn create_default_node(&mut self) -> &Node {
        self.create_node(&nanoid!(), Attributes::default())
    }

    pub fn create_node(&mut self, id: &str, attributes: Attributes) -> &Node {
        let n = Node::new(id, attributes);
        let node_id = n.id().to_string();
        self.nodes.insert(node_id.to_string(), n);
        self.get_node(&node_id).unwrap()
    }

    pub fn find_edges_by_attributes(&self, key: &str, matcher: &dyn Matcher) -> Vec<&Edge> {
        find_by_attributes(self.edges.values(), key, matcher)
    }

    pub fn find_nodes_by_attributes(&self, key: &str, matcher: &dyn Matcher) -> Vec<&Node> {
        find_by_attributes(self.nodes.values(), key, matcher)
    }

    pub fn find_nodes_by_path(&self, path: &[&str]) -> Vec<&Node> {
        find_nodes_by_path_internal(self, self.nodes.values(), path)
    }

    pub fn find_nodes_by_query(&self, query: &Query) -> Vec<&Node> {
        query
            .attributes()
            .iter()
            .map(|x| Option::Some(self.find_nodes_by_attributes(x.0, x.1.as_ref())))
            .fold(
                Option::None,
                |a: Option<Vec<&Node>>, b: Option<Vec<&Node>>| {
                    if a.is_some() && b.is_some() {
                        return Option::Some(a.unwrap().intersect(b.unwrap()));
                    }
                    a.or(b)
                },
            )
            .unwrap_or_else(|| vec![])
    }

    pub fn attributes_of_node_mut(&mut self, id: &str) -> Option<&mut Attributes> {
        let node = self.nodes.get_mut(id);
        node.map(|f| f.attributes_mut())
    }

    pub fn attributes_of_edge_mut(&mut self, id: &str) -> Option<&mut Attributes> {
        let edge = self.edges.get_mut(id);
        edge.map(|e| e.attributes_mut())
    }

    pub fn get_edges_from_node(&self, node_id: &str, relation: &str) -> Vec<&Edge> {
        self.edges
            .values()
            .filter(|e| e.relation() == relation && e.from_id() == node_id)
            .collect()
    }

    pub fn edge_count(&self) -> usize {
        self.edges.len()
    }

    pub fn get_edge(&self, id: &str) -> Option<&Edge> {
        self.edges.get(id)
    }

    pub fn create_default_edge<'a>(
        &mut self,
        relation: &str,
        source_id: &str,
        destination_id: &str,
    ) -> Result<String, &'a str> {
        self.create_edge(
            &nanoid!(),
            relation,
            Attributes::default(),
            source_id,
            destination_id,
        )
    }

    pub fn create_edge<'a>(
        &mut self,
        id: &str,
        relation: &str,
        attributes: Attributes,
        source_id: &str,
        destination_id: &str,
    ) -> Result<String, &'a str> {
        if self.has_relation(relation, source_id, destination_id) {
            return Err("Edge already exists");
        }

        let e = Edge::new(
            String::from(id),
            String::from(relation),
            String::from(source_id),
            String::from(destination_id),
            attributes,
        );
        let edge_id = e.id().to_string();
        self.edges.insert(edge_id.to_string(), e);

        self.relations
            .entry(util::build_relation_key(source_id, destination_id))
            .or_insert_with(HashMap::new)
            .insert(String::from(relation), edge_id.to_string());

        Ok(edge_id)
    }

    pub fn create_edges<'a>(
        &mut self,
        relation: &str,
        attributes: Attributes,
        source_id: &str,
        destination_ids: &[&str],
    ) -> Vec<Result<String, &'a str>> {
        destination_ids
            .iter()
            .map(|destination| {
                self.create_edge(
                    &nanoid!(),
                    relation,
                    attributes.clone(),
                    source_id,
                    destination,
                )
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
