use neuland::{io::graphml::import, query::matcher::Matcher};
use neuland::{model::attributes::HasAttributes, query::matcher::eq_matcher::EqMatcher};
use std::fs;
use std::{collections::HashSet, path::PathBuf};
#[test]
fn test_import_generic_dataset() {
    let path = fs::canonicalize(&PathBuf::from("./tests/resources/test.graphml")).unwrap();
    let graph = import(&(path.to_str().unwrap())).unwrap();
    let node_a = graph.get_node("n0").unwrap();
    let node_b = graph.get_node("n1").unwrap();
    let nodes_with_four_nested_children = graph.find_nodes_by_path(&vec![
        "unknown".to_string(),
        "unknown".to_string(),
        "unknown".to_string(),
        "unknown".to_string(),
    ]);
    let edges = graph.get_edges_from_node("n0", "unknown");
    let edge_ids = edges.iter().map(|x| x.id()).collect::<HashSet<_>>();
    let edge_a = graph.get_edge("e1").unwrap();

    assert_eq!(node_a.attributes().get("d0").unwrap(), "green");
    assert_eq!(
        graph
            .get_node("n5")
            .unwrap()
            .attributes()
            .get("d0")
            .unwrap(),
        "turquoise"
    );
    assert!(node_b.attributes().get("d0").is_none());
    assert_eq!(graph.node_count(), 6);
    assert_eq!(graph.edge_count(), 7);
    assert_eq!(nodes_with_four_nested_children.first().unwrap().id(), "n0");
    assert_eq!(edge_ids.len(), 2);
    assert!(edge_ids.contains(&String::from("e0")));
    assert!(edge_ids.contains(&String::from("e1")));
    assert_eq!(edge_a.attributes().get("d1").unwrap(), "1.0");
}

#[test]
fn test_import_airplanes_dataset() {
    let path = fs::canonicalize(&PathBuf::from("./tests/resources/airlines.graphml")).unwrap();
    let graph = import(&(path.to_str().unwrap())).unwrap();

    let route_matcher: &dyn Matcher = &EqMatcher::new("route");
    let airport_matcher: &dyn Matcher = &EqMatcher::new("airport");
    let route_edges = graph.find_edges_by_attributes("labelE", route_matcher);
    let airport_nodes = graph.find_nodes_by_attributes("type", airport_matcher);

    assert_eq!(graph.node_count(), 47);
    assert_eq!(graph.edge_count(), 1386);
    assert_eq!(route_edges.len(), 1386);
    assert_eq!(airport_nodes.len(), 46);
    assert_eq!(
        graph
            .get_node("0")
            .unwrap()
            .attributes()
            .get("code")
            .unwrap(),
        "0.84"
    );
}
