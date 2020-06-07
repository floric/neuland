#[cfg(test)]
mod graph {
    use gdb::model::Graph;
    use std::collections::HashSet;

    #[test]
    fn test_graph_creation() {
        let graph = Graph::new();

        assert_eq!(0, graph.node_count(), "Graph should be empty");
    }

    #[test]
    fn test_node_creation() {
        let mut graph = Graph::new();
        let node_a = graph.create_node(HashSet::new()).clone();
        let node_b = graph.create_node(HashSet::new()).clone();

        assert_eq!(21, node_a.id().len());
        assert_eq!(21, node_b.id().len());
        assert_ne!(node_a.id(), node_b.id());
        assert_eq!(2, graph.node_count(), "Graph should have two nodes");
    }

    #[test]
    fn test_edge_creation() {
        let mut graph = Graph::new();

        let node_a = graph.create_node(HashSet::new()).clone();
        let node_b = graph.create_node(HashSet::new()).clone();
        let edge = graph
            .create_edge(HashSet::new(), node_a.id(), node_b.id())
            .clone();

        assert_eq!(21, edge.id().len());
        assert_eq!(node_a.id(), edge.from_id());
        assert_eq!(node_b.id(), edge.to_id());
        assert_eq!(2, graph.node_count(), "Graph should have two nodes");
    }
}
