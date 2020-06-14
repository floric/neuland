use neuland::model::Attributes;
use neuland::model::Graph;

#[test]
pub fn test_family_tree_creation() {
    let mut graph = Graph::default();

    let father = graph.create_node(Attributes::default()).clone();
    let mother = graph.create_node(Attributes::default()).clone();
    let son = graph.create_node(Attributes::default()).clone();
    let daughter = graph.create_node(Attributes::default()).clone();
    let children: Vec<&str> = vec![daughter.id(), son.id()];
    let parents: Vec<&str> = vec![father.id(), mother.id()];
    graph.create_edges("is-child", Attributes::default(), son.id(), &parents);
    graph.create_edges("is-child", Attributes::default(), daughter.id(), &parents);
    graph.create_edges("is-father", Attributes::default(), father.id(), &children);
    graph.create_edges("is-mother", Attributes::default(), mother.id(), &children);

    assert_eq!(4, graph.node_count());
    assert_eq!(8, graph.edge_count());
    assert!(graph.has_relation("is-child", son.id(), father.id()));
    assert!(graph.has_relation("is-mother", mother.id(), daughter.id()));
    assert!(!graph.has_relation("is-father", father.id(), mother.id()));
    assert!(!graph.has_relation("is-mother", daughter.id(), mother.id()));
}
