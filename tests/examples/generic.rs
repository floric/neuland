use nanoid::nanoid;
use neuland::{
    model::Attributes, model::Graph, query::matcher::eq_matcher::EqMatcher, query::matcher::Matcher,
};

#[test]
fn test_graph_creation() {
    let graph = Graph::default();

    assert_eq!(0, graph.node_count());
}

#[test]
fn test_node_creation() {
    let mut graph = Graph::default();
    let node_a = graph.create_default_node().id().to_string();
    let node_b = graph.create_default_node().id().to_string();

    assert_eq!(21, node_a.len());
    assert_eq!(21, node_b.len());
    assert_ne!(&node_a, &node_b);
    assert_eq!(2, graph.node_count());
    assert!(!graph.has_any_relation(&node_a, &node_b));
}

#[test]
fn test_edge_creation() {
    let mut graph = Graph::default();

    let node_a = graph.create_default_node().id().to_string();
    let node_b = graph.create_default_node().id().to_string();
    let edge_a_id = graph
        .create_edge(&nanoid!(), "rel-a", Attributes::default(), &node_a, &node_b)
        .unwrap();
    let edge_a = graph.get_edge(&edge_a_id).unwrap().clone();
    graph
        .create_edge(&nanoid!(), "rel-b", Attributes::default(), &node_a, &node_b)
        .unwrap();

    assert!(graph.has_any_relation(&node_a, &node_b));
    assert!(!graph.has_any_relation(&node_b, &node_a));
    assert!(!graph.has_any_relation(&node_a, &node_a));
    assert!(graph.has_relation("rel-a", &node_a, &node_b));
    assert!(graph.has_relation("rel-b", &node_a, &node_b));
    assert!(!graph.has_relation("rel-c", &node_a, &node_b));
    assert_eq!(21, edge_a_id.len());
    assert_eq!(&node_a, edge_a.from_id());
    assert_eq!(&node_b, edge_a.to_id());
    assert_eq!(2, graph.node_count());
    assert_eq!(2, graph.edge_count());
}

#[test]
fn test_edge_creation_only_once() {
    let mut graph = Graph::default();

    let node_a = graph.create_default_node().id().to_string();
    let node_b = graph.create_default_node().id().to_string();
    graph
        .create_edge(&nanoid!(), "rel-a", Attributes::default(), &node_a, &node_b)
        .unwrap();
    let edge_b_id = graph
        .create_edge(&nanoid!(), "rel-a", Attributes::default(), &node_a, &node_b)
        .unwrap_err();

    assert_eq!(edge_b_id, "Edge already exists");
}

#[test]
fn test_edge_retrieval() {
    let mut graph = Graph::default();

    let node_a = graph.create_default_node().id().to_string();
    let node_b = graph.create_default_node().id().to_string();
    let edge_a_id = graph
        .create_edge(&nanoid!(), "rel-a", Attributes::default(), &node_a, &node_b)
        .unwrap();
    let edge_a = graph.get_edge(&edge_a_id).unwrap();
    let saved_edge_a = graph.get_edge(edge_a.id()).unwrap();

    assert_eq!(saved_edge_a.id(), edge_a.id());
    assert!(graph.get_edge("abc").is_none());
}

#[test]
fn test_edge_removal() {
    let mut graph = Graph::default();

    let node_a = graph.create_default_node().id().to_string();
    let node_b = graph.create_default_node().id().to_string();
    let edge_id = graph
        .create_edge(&nanoid!(), "rel-a", Attributes::default(), &node_a, &node_b)
        .unwrap();
    let edge = graph.get_edge(&edge_id).unwrap().id().to_string();
    graph
        .create_edge(&nanoid!(), "rel-b", Attributes::default(), &node_a, &node_b)
        .unwrap();
    graph.remove_edge(&edge);

    assert!(!graph.has_relation("rel-a", &node_a, &node_b));
    assert!(graph.has_relation("rel-b", &node_a, &node_b));
    assert_eq!(2, graph.node_count());
    assert_eq!(1, graph.edge_count());
    assert!(graph.has_any_relation(&node_a, &node_b));
}

#[test]
fn test_edges_removal() {
    let mut graph = Graph::default();

    let node_a = graph.create_default_node().id().to_string();
    let node_b = graph.create_default_node().id().to_string();
    let edge_a_id = graph
        .create_edge(&nanoid!(), "rel-a", Attributes::default(), &node_a, &node_b)
        .unwrap();
    let edge_a = graph.get_edge(&edge_a_id).unwrap().id().to_string();
    let edge_b_id = graph
        .create_edge(&nanoid!(), "rel-b", Attributes::default(), &node_a, &node_b)
        .unwrap();
    let edge_b = graph.get_edge(&edge_b_id).unwrap().id().to_string();
    graph.remove_edge(&edge_a);
    graph.remove_edge(&edge_b);

    assert!(!graph.has_relation("rel-a", &node_a, &node_b));
    assert!(!graph.has_relation("rel-b", &node_a, &node_b));
    assert_eq!(2, graph.node_count());
    assert_eq!(0, graph.edge_count());
    assert!(!graph.has_any_relation(&node_a, &node_b));
}

#[test]
fn test_find_nodes_by_attributes() {
    let mut graph = Graph::default();
    let mut attributes_a = Attributes::default();
    attributes_a.set("test", "a");
    let mut attributes_b = Attributes::default();
    attributes_b.set("test", "b");
    let node_a = graph.create_node(&nanoid!(), attributes_a).clone();
    let node_b = graph.create_node(&nanoid!(), attributes_b).clone();
    let node_c = graph.create_default_node().clone();

    let matcher: &dyn Matcher = &EqMatcher::new("a");
    let matches = graph.find_nodes_by_attributes("test", matcher);
    assert_eq!(1, matches.len());
    assert!(matches.contains(&&node_a));
    assert!(!matches.contains(&&node_b));
    assert!(!matches.contains(&&node_c));
}

#[test]
fn test_find_nodes_by_attributes_with_other_value() {
    let mut graph = Graph::default();
    let mut attributes = Attributes::default();
    attributes.set("test", "a");
    graph.create_node(&nanoid!(), attributes);

    let matcher: &dyn Matcher = &EqMatcher::new("b");
    let matches = graph.find_nodes_by_attributes("test", matcher);
    assert!(matches.is_empty());
}

#[test]
fn test_find_nodes_by_attributes_with_no_result() {
    let mut graph = Graph::default();
    graph.create_default_node();

    let matcher: &dyn Matcher = &EqMatcher::new("a");
    let matches = graph.find_nodes_by_attributes("x", matcher);
    assert!(matches.is_empty());
}

#[test]
fn test_find_edges_by_attributes() {
    let mut graph = Graph::default();
    let mut attributes = Attributes::default();
    attributes.set("test", "a");
    let node_a = graph.create_default_node().id().to_string();
    let node_b = graph.create_default_node().id().to_string();

    let edge_id = graph
        .create_edge(&nanoid!(), "a", attributes, &node_a, &node_b)
        .unwrap();

    let matcher: &dyn Matcher = &EqMatcher::new("a");
    let matches = graph.find_edges_by_attributes("test", matcher);
    assert!(!matches.is_empty());
    assert!(matches.contains(&graph.get_edge(&edge_id).unwrap()));
}

#[test]
fn test_find_nodes_by_empty_path() {
    let mut graph = Graph::default();
    let node_a = graph.create_default_node().id().to_string();
    let node_b = graph.create_default_node().id().to_string();
    graph
        .create_edge(
            &nanoid!(),
            "is-some",
            Attributes::default(),
            &node_a,
            &node_b,
        )
        .ok();

    let matches = graph.find_nodes_by_path(&vec![]);

    assert!(matches.is_empty());
}

#[test]
fn test_find_nodes_by_unknown_path() {
    let mut graph = Graph::default();
    let node_a = graph.create_default_node().id().to_string();
    let node_b = graph.create_default_node().id().to_string();
    graph
        .create_edge(
            &nanoid!(),
            "is-some",
            Attributes::default(),
            &node_a,
            &node_b,
        )
        .ok();
    let matches = graph.find_nodes_by_path(&vec!["is-unknown".to_string()]);

    assert!(matches.is_empty());
}

#[test]
fn test_find_nodes_by_path_in_cycles() {
    let mut graph = Graph::default();
    let node_a = graph.create_default_node().id().to_string();
    let node_b = graph.create_default_node().id().to_string();
    let node_c = graph.create_default_node().id().to_string();
    graph
        .create_edge(&nanoid!(), "is-a", Attributes::default(), &node_a, &node_b)
        .ok();
    graph
        .create_edge(&nanoid!(), "is-a", Attributes::default(), &node_b, &node_c)
        .ok();
    graph
        .create_edge(&nanoid!(), "is-a", Attributes::default(), &node_c, &node_a)
        .ok();

    let matches = graph.find_nodes_by_path(&vec!["is-a".to_string()]);
    let cyclic_matches = graph.find_nodes_by_path(&vec![
        "is-a".to_string(),
        "is-a".to_string(),
        "is-a".to_string(),
        "is-a".to_string(),
    ]);
    let twice_cyclic_matches = graph.find_nodes_by_path(&vec![
        "is-a".to_string(),
        "is-a".to_string(),
        "is-a".to_string(),
        "is-a".to_string(),
        "is-a".to_string(),
        "is-b".to_string(),
    ]);

    assert_eq!(matches.len(), 3);
    assert_eq!(cyclic_matches.len(), 3);
    assert!(twice_cyclic_matches.is_empty());
}
