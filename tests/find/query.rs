use neuland::{
    model::Graph, model::Query, query::matcher::eq_matcher::EqMatcher, query::matcher::Matcher,
};

use crate::utils::rnd::generate_random_graph;

#[test]
fn test_query_without_filters() {
    let graph = create_test_graph();
    let query = Query::new();
    let found_nodes = graph.find_nodes_by_query(&query);

    assert_eq!(graph.node_count(), 1000);
    assert_eq!(graph.edge_count(), 2000);
    assert_eq!(found_nodes.len(), 1000);
}

#[test]
fn test_query_with_filters_for_attributes() {
    let graph = create_test_graph();
    let matcher: Box<dyn Matcher> = Box::from(EqMatcher::new("123"));
    let mut query = Query::new();
    query.select().with_att("abc", matcher);
    let found_nodes = graph.find_nodes_by_query(&query);

    assert_eq!(found_nodes.len(), 213);
}

#[test]
fn test_query_with_filters_for_attributes_without_matches() {
    let graph = create_test_graph();
    let matcher_a: Box<dyn Matcher> = Box::from(EqMatcher::new("123"));
    let matcher_b: Box<dyn Matcher> = Box::from(EqMatcher::new("123"));
    let mut query = Query::new();
    query
        .select()
        .with_att("abc", matcher_a)
        .with_att("unknown", matcher_b);
    let found_nodes = graph.find_nodes_by_query(&query);

    assert_eq!(found_nodes.len(), 0);
}

#[test]
fn test_query_with_filters_for_attributes_with_few_matches() {
    let graph = create_test_graph();
    let matcher_a: Box<dyn Matcher> = Box::from(EqMatcher::new("123"));
    let matcher_b: Box<dyn Matcher> = Box::from(EqMatcher::new("456"));
    let mut query = Query::new();
    query
        .select()
        .with_att("abc", matcher_a)
        .with_att("xyz", matcher_b);
    let found_nodes = graph.find_nodes_by_query(&query);

    assert_eq!(found_nodes.len(), 25);
}
#[test]
fn test_query_with_filters_for_attributes_with_path_match() {
    let graph = create_test_graph();
    let random_edge_id = graph
        .find_nodes_by_query(&Query::new())
        .iter()
        .filter_map(|x| {
            let edges = graph.get_all_edges_from_node(x.id());
            if edges.is_empty() {
                return Option::None;
            }
            return Option::Some(edges);
        })
        .next()
        .unwrap()
        .first()
        .iter()
        .next()
        .unwrap()
        .relation();

    let mut query = Query::new();
    query.select().with_path(&[random_edge_id]);
    let found_nodes = graph.find_nodes_by_query(&query);

    assert_eq!(found_nodes.len(), 1);
}

fn create_test_graph() -> Graph {
    generate_random_graph(123456, 1000, 2000)
}
