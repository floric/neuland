use neuland::io::graphml::import;
use std::fs;
use std::{collections::HashSet, path::PathBuf};

#[test]
fn test_import_generic_dataset() {
    let path = fs::canonicalize(&PathBuf::from("./tests/resources/test.graphml")).unwrap();
    let graph = import(&(path.to_str().unwrap())).unwrap();
    let node_a = graph.get_node("n0").unwrap();
    let node_b = graph.get_node("n1").unwrap();
    let nodes_with_four_nested_children =
        graph.find_nodes_by_path(vec!["unknown", "unknown", "unknown", "unknown"]);
    let edges = graph.get_edges_from_node("n0", "unknown");
    let edge_ids = edges.iter().map(|x| x.id()).collect::<HashSet<_>>();

    assert!(node_a.attributes().get("d0").is_none());
    assert!(node_b.attributes().get("d0").is_none());
    assert_eq!(graph.node_count(), 6);
    assert_eq!(graph.edge_count(), 7);
    assert_eq!(nodes_with_four_nested_children.first().unwrap().id(), "n0");
    assert_eq!(edge_ids.len(), 2);
    assert!(edge_ids.contains(&String::from("e0")));
    assert!(edge_ids.contains(&String::from("e1")));
}

#[test]
fn test_import_airplanes_dataset() {
    let path = fs::canonicalize(&PathBuf::from("./tests/resources/airlines.graphml")).unwrap();
    let graph = import(&(path.to_str().unwrap())).unwrap();

    assert_eq!(graph.node_count(), 235);
    assert_eq!(graph.edge_count(), 2101);
}
