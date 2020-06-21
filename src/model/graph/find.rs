use super::Graph;
use crate::model::{attributes::HasAttributes, Node};

pub fn find_by_attributes<'a, T, I, F>(items: I, key: &str, matcher: F) -> Vec<&'a T>
where
    T: HasAttributes,
    I: Iterator<Item = &'a T>,
    F: Fn(&&'a String) -> bool,
{
    items
        .filter(|node| node.attributes().get(key).filter(&matcher).is_some())
        .collect()
}

pub fn find_nodes_by_path_internal<'a, I>(graph: &Graph, nodes: I, path: Vec<&str>) -> Vec<&'a Node>
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
            let found_edges = graph.get_edges_from_node(x.id(), relation);
            if found_edges.is_empty() {
                return false;
            } else if remaining_relations.is_empty() {
                return true;
            }

            found_edges.iter().any(|e| {
                let a = graph.get_node(e.to_id()).unwrap();
                !find_nodes_by_path_internal(
                    graph,
                    vec![a.clone()].iter(),
                    Vec::from(remaining_relations),
                )
                .is_empty()
            })
        })
        .collect()
}
