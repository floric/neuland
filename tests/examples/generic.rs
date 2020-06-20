use neuland::model::Attributes;
use neuland::model::Graph;

#[test]
fn test_graph_creation() {
    let graph = Graph::default();

    assert_eq!(0, graph.node_count());
}

#[test]
fn test_node_creation() {
    let mut graph = Graph::default();
    let node_a = graph.create_node(Attributes::default()).clone();
    let node_b = graph.create_node(Attributes::default()).clone();

    assert_eq!(21, node_a.id().len());
    assert_eq!(21, node_b.id().len());
    assert_ne!(node_a.id(), node_b.id());
    assert_eq!(2, graph.node_count());
    assert!(!graph.has_any_relation(node_a.id(), node_b.id()));
}

#[test]
fn test_edge_creation() {
    let mut graph = Graph::default();

    let node_a = graph.create_node(Attributes::default()).clone();
    let node_b = graph.create_node(Attributes::default()).clone();
    let edge_a_id = graph
        .create_edge("rel-a", Attributes::default(), node_a.id(), node_b.id())
        .unwrap();
    let edge_a = graph.get_edge(&edge_a_id).unwrap().clone();
    graph
        .create_edge("rel-b", Attributes::default(), node_a.id(), node_b.id())
        .unwrap();

    assert!(graph.has_any_relation(node_a.id(), node_b.id()));
    assert!(!graph.has_any_relation(node_b.id(), node_a.id()));
    assert!(!graph.has_any_relation(node_a.id(), node_a.id()));
    assert!(graph.has_relation("rel-a", node_a.id(), node_b.id()));
    assert!(graph.has_relation("rel-b", node_a.id(), node_b.id()));
    assert!(!graph.has_relation("rel-c", node_a.id(), node_b.id()));
    assert_eq!(21, edge_a_id.len());
    assert_eq!(node_a.id(), edge_a.from_id());
    assert_eq!(node_b.id(), edge_a.to_id());
    assert_eq!(2, graph.node_count());
    assert_eq!(2, graph.edge_count());
}

#[test]
fn test_edge_creation_only_once() {
    let mut graph = Graph::default();

    let node_a = graph.create_node(Attributes::default()).clone();
    let node_b = graph.create_node(Attributes::default()).clone();
    graph
        .create_edge("rel-a", Attributes::default(), node_a.id(), node_b.id())
        .unwrap();
    let edge_b_id = graph
        .create_edge("rel-a", Attributes::default(), node_a.id(), node_b.id())
        .unwrap_err();

    assert_eq!(edge_b_id, "Edge already exists");
}

#[test]
fn test_edge_retrieval() {
    let mut graph = Graph::default();

    let node_a = graph.create_node(Attributes::default()).clone();
    let node_b = graph.create_node(Attributes::default()).clone();
    let edge_a_id = graph
        .create_edge("rel-a", Attributes::default(), node_a.id(), node_b.id())
        .unwrap();
    let edge_a = graph.get_edge(&edge_a_id).unwrap();
    let saved_edge_a = graph.get_edge(edge_a.id()).unwrap();

    assert_eq!(saved_edge_a.id(), edge_a.id());
    assert!(graph.get_edge("abc").is_none());
}

#[test]
fn test_edge_removal() {
    let mut graph = Graph::default();

    let node_a = graph.create_node(Attributes::default()).clone();
    let node_b = graph.create_node(Attributes::default()).clone();
    let edge_id = graph
        .create_edge("rel-a", Attributes::default(), node_a.id(), node_b.id())
        .unwrap();
    let edge = graph.get_edge(&edge_id).unwrap().clone();
    graph
        .create_edge("rel-b", Attributes::default(), node_a.id(), node_b.id())
        .unwrap();
    graph.remove_edge(edge.id());

    assert!(!graph.has_relation("rel-a", node_a.id(), node_b.id()));
    assert!(graph.has_relation("rel-b", node_a.id(), node_b.id()));
    assert_eq!(2, graph.node_count());
    assert_eq!(1, graph.edge_count());
    assert!(graph.has_any_relation(node_a.id(), node_b.id()));
}

#[test]
fn test_edges_removal() {
    let mut graph = Graph::default();

    let node_a = graph.create_node(Attributes::default()).clone();
    let node_b = graph.create_node(Attributes::default()).clone();
    let edge_a_id = graph
        .create_edge("rel-a", Attributes::default(), node_a.id(), node_b.id())
        .unwrap();
    let edge_a = graph.get_edge(&edge_a_id).unwrap().clone();
    let edge_b_id = graph
        .create_edge("rel-b", Attributes::default(), node_a.id(), node_b.id())
        .unwrap();
    let edge_b = graph.get_edge(&edge_b_id).unwrap().clone();
    graph.remove_edge(edge_a.id());
    graph.remove_edge(edge_b.id());

    assert!(!graph.has_relation("rel-a", node_a.id(), node_b.id()));
    assert!(!graph.has_relation("rel-b", node_a.id(), node_b.id()));
    assert_eq!(2, graph.node_count());
    assert_eq!(0, graph.edge_count());
    assert!(!graph.has_any_relation(node_a.id(), node_b.id()));
}

#[test]
fn test_find_nodes_by_attributes() {
    let mut graph = Graph::default();
    let mut attributes_a = Attributes::default();
    attributes_a.set("test", String::from("a"));
    let mut attributes_b = Attributes::default();
    attributes_b.set("test", String::from("b"));
    let node_a = graph.create_node(attributes_a).clone();
    let node_b = graph.create_node(attributes_b).clone();
    let node_c = graph.create_node(Attributes::default()).clone();

    let matches = graph.find_nodes_by_attributes("test", |x| *x == "a");
    assert_eq!(1, matches.len());
    assert!(matches.contains(&&node_a));
    assert!(!matches.contains(&&node_b));
    assert!(!matches.contains(&&node_c));
}

#[test]
fn test_find_nodes_by_attributes_with_other_value() {
    let mut graph = Graph::default();
    let mut attributes = Attributes::default();
    attributes.set("test", String::from("a"));
    graph.create_node(attributes);

    let matches = graph.find_nodes_by_attributes("test", |x| *x == "b");
    assert!(matches.is_empty());
}

#[test]
fn test_find_nodes_by_attributes_with_no_result() {
    let mut graph = Graph::default();
    graph.create_node(Attributes::default());

    let matches = graph.find_nodes_by_attributes("x", |x| *x == "a");
    assert!(matches.is_empty());
}

#[test]
fn test_find_edges_by_attributes() {
    let mut graph = Graph::default();
    let mut attributes = Attributes::default();
    attributes.set("test", String::from("a"));
    let node_a = graph.create_node(Attributes::default()).clone();
    let node_b = graph.create_node(Attributes::default()).clone();

    let edge_id = graph
        .create_edge("a", attributes, node_a.id(), node_b.id())
        .unwrap();

    let matches = graph.find_edges_by_attributes("test", |x| *x == "a");
    assert!(!matches.is_empty());
    assert!(matches.contains(&graph.get_edge(&edge_id).unwrap()));
}

#[test]
fn test_find_nodes_by_empty_path() {
    let mut graph = Graph::default();
    let node_a = graph.create_node(Attributes::default()).clone();
    let node_b = graph.create_node(Attributes::default()).clone();
    graph
        .create_edge("is-some", Attributes::default(), node_a.id(), node_b.id())
        .clone()
        .ok();

    let matches = graph.find_nodes_by_path(vec![]);

    assert!(matches.is_empty());
}

#[test]
fn test_find_nodes_by_unknown_path() {
    let mut graph = Graph::default();
    let node_a = graph.create_node(Attributes::default()).clone();
    let node_b = graph.create_node(Attributes::default()).clone();
    graph
        .create_edge("is-some", Attributes::default(), node_a.id(), node_b.id())
        .clone()
        .ok();
    let matches = graph.find_nodes_by_path(vec!["is-unknown"]);

    assert!(matches.is_empty());
}

#[test]
fn test_find_nodes_by_path_in_cycles() {
    let mut graph = Graph::default();
    let node_a = graph.create_node(Attributes::default()).clone();
    let node_b = graph.create_node(Attributes::default()).clone();
    let node_c = graph.create_node(Attributes::default()).clone();
    graph
        .create_edge("is-a", Attributes::default(), node_a.id(), node_b.id())
        .ok();
    graph
        .create_edge("is-a", Attributes::default(), node_b.id(), node_c.id())
        .ok();
    graph
        .create_edge("is-a", Attributes::default(), node_c.id(), node_a.id())
        .ok();

    let matches = graph.find_nodes_by_path(vec!["is-a"]);
    let cyclic_matches = graph.find_nodes_by_path(vec!["is-a", "is-a", "is-a", "is-a"]);
    let twice_cyclic_matches =
        graph.find_nodes_by_path(vec!["is-a", "is-a", "is-a", "is-a", "is-a", "is-b"]);

    assert_eq!(matches.len(), 3);
    assert_eq!(cyclic_matches.len(), 3);
    assert!(twice_cyclic_matches.is_empty());
}
