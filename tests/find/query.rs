use neuland::model::Query;

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
