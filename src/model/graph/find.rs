use super::Graph;
use crate::{
    model::{attributes::HasAttributes, Node},
    query::matcher::Matcher,
};

pub fn find_by_attributes<'a, T, I>(items: I, key: &str, matcher: &dyn Matcher) -> Vec<&'a T>
where
    T: HasAttributes,
    I: Iterator<Item = &'a T>,
{
    items
        .filter(|node| {
            node.attributes()
                .get(key)
                .filter(|x| matcher.apply(x))
                .is_some()
        })
        .collect()
}

pub fn find_nodes_by_path_internal<'a, I>(graph: &Graph, nodes: I, path: &[&str]) -> Vec<&'a Node>
where
    I: Iterator<Item = &'a Node>,
{
    match path.split_first() {
        Some((relation, remaining_relations)) => nodes
            .filter(|x| {
                let found_edges = graph.get_edges_from_node(x.id(), relation);
                if found_edges.is_empty() {
                    return false;
                } else if remaining_relations.is_empty() {
                    return true;
                }

                found_edges.iter().any(|e| {
                    let a = graph.get_node(e.to_id()).unwrap();
                    !find_nodes_by_path_internal(graph, vec![a.clone()].iter(), remaining_relations)
                        .is_empty()
                })
            })
            .collect(),
        None => vec![],
    }
}
