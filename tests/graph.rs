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
    let edge_a = graph
        .create_edge("rel-a", Attributes::default(), node_a.id(), node_b.id())
        .clone();
    graph.create_edge("rel-b", Attributes::default(), node_a.id(), node_b.id());

    assert!(graph.has_any_relation(node_a.id(), node_b.id()));
    assert!(!graph.has_any_relation(node_b.id(), node_a.id()));
    assert!(!graph.has_any_relation(node_a.id(), node_a.id()));
    assert!(graph.has_relation("rel-a", node_a.id(), node_b.id()));
    assert!(graph.has_relation("rel-b", node_a.id(), node_b.id()));
    assert!(!graph.has_relation("rel-c", node_a.id(), node_b.id()));
    assert_eq!(21, edge_a.id().len());
    assert_eq!(node_a.id(), edge_a.from_id());
    assert_eq!(node_b.id(), edge_a.to_id());
    assert_eq!(2, graph.node_count());
    assert_eq!(2, graph.edge_count());
}

#[test]
fn test_edge_removal() {
    let mut graph = Graph::default();

    let node_a = graph.create_node(Attributes::default()).clone();
    let node_b = graph.create_node(Attributes::default()).clone();
    let edge = graph
        .create_edge("rel-a", Attributes::default(), node_a.id(), node_b.id())
        .clone();
    graph.create_edge("rel-b", Attributes::default(), node_a.id(), node_b.id());
    graph.remove_edge(edge.id());

    assert!(!graph.has_relation("rel-a", node_a.id(), node_b.id()));
    assert!(graph.has_relation("rel-b", node_a.id(), node_b.id()));
    assert_eq!(2, graph.node_count());
    assert_eq!(1, graph.edge_count());
    assert!(graph.has_any_relation(node_a.id(), node_b.id()));
}
