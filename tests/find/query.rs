use neuland::{model::Query, query::matcher::eq_matcher::EqMatcher, query::matcher::Matcher};

use crate::utils::rnd::generate_random_graph;

#[test]
fn test_query_without_filters() {
    let graph = generate_random_graph(1000, 2000);
    let query = Query::select();
    let found_nodes = graph.find_nodes_by_query(&query);

    assert_eq!(graph.node_count(), 1000);
    assert_eq!(graph.edge_count(), 2000);
    assert_eq!(found_nodes.len(), 0);
}

#[test]
fn test_query_with_filters_for_attributes() {
    let graph = generate_random_graph(1000, 2000);
    let matcher: Box<dyn Matcher> = Box::from(EqMatcher::new("123"));
    let mut query = Query::select();
    query.with_att("abc", matcher);
    let found_nodes = graph.find_nodes_by_query(&query);

    assert_eq!(graph.node_count(), 1000);
    assert_eq!(graph.edge_count(), 2000);
    assert_eq!(found_nodes.len(), 213);
}

#[test]
fn test_query_with_filters_for_attributes_without_matches() {
    let graph = generate_random_graph(1000, 2000);
    let matcher_a: Box<dyn Matcher> = Box::from(EqMatcher::new("123"));
    let matcher_b: Box<dyn Matcher> = Box::from(EqMatcher::new("123"));
    let mut query = Query::select();
    query.with_att("abc", matcher_a);
    query.with_att("unknown", matcher_b);
    let found_nodes = graph.find_nodes_by_query(&query);

    assert_eq!(graph.node_count(), 1000);
    assert_eq!(graph.edge_count(), 2000);
    assert_eq!(found_nodes.len(), 0);
}

#[test]
fn test_query_with_filters_for_attributes_with_few_matches() {
    let graph = generate_random_graph(1000, 2000);
    let matcher_a: Box<dyn Matcher> = Box::from(EqMatcher::new("123"));
    let matcher_b: Box<dyn Matcher> = Box::from(EqMatcher::new("456"));
    let mut query = Query::select();
    query.with_att("abc", matcher_a);
    query.with_att("xyz", matcher_b);
    let found_nodes = graph.find_nodes_by_query(&query);

    assert_eq!(graph.node_count(), 1000);
    assert_eq!(graph.edge_count(), 2000);
    assert_eq!(found_nodes.len(), 25);
}
