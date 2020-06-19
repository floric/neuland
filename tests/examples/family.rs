use neuland::model::Attributes;
use neuland::model::Graph;
use neuland::model::Node;

#[test]
pub fn test_family_tree_creation() {
    let (graph, father, mother, son, daughter) = create_graph();

    assert_eq!(6, graph.node_count());
    assert_eq!(10, graph.edge_count());
    assert!(graph.has_relation("is-child", son.id(), father.id()));
    assert!(graph.has_relation("is-mother", mother.id(), daughter.id()));
    assert!(!graph.has_relation("is-father", father.id(), mother.id()));
    assert!(!graph.has_relation("is-mother", daughter.id(), mother.id()));
}

#[test]
pub fn test_family_tree_find() {
    let (graph, _, _, _, _) = create_graph();

    let children = graph.find_nodes_by_path(vec!["is-child"]);
    let fathers = graph.find_nodes_by_path(vec!["is-father"]);
    let grand_mothers = graph.find_nodes_by_path(vec!["is-mother", "is-father"]);
    let unknowns = graph.find_nodes_by_path(vec!["is-x"]);

    assert_eq!(children.len(), 2);
    assert_eq!(fathers.len(), 2);
    assert_eq!(grand_mothers.len(), 1);
    assert!(unknowns.is_empty());
}

fn create_graph() -> (Graph, Node, Node, Node, Node) {
    let mut graph = Graph::default();

    let father = graph.create_node(Attributes::default()).clone();
    let mother = graph.create_node(Attributes::default()).clone();
    let son = graph.create_node(Attributes::default()).clone();
    let daughter = graph.create_node(Attributes::default()).clone();
    let grand_father = graph.create_node(Attributes::default()).clone();
    let grand_mother = graph.create_node(Attributes::default()).clone();
    let children: Vec<&str> = vec![daughter.id(), son.id()];
    let parents: Vec<&str> = vec![father.id(), mother.id()];
    graph.create_edges("is-child", Attributes::default(), son.id(), &parents);
    graph.create_edges("is-child", Attributes::default(), daughter.id(), &parents);
    graph.create_edges("is-father", Attributes::default(), father.id(), &children);
    graph.create_edges("is-mother", Attributes::default(), mother.id(), &children);
    graph
        .create_edge(
            "is-father",
            Attributes::default(),
            grand_father.id(),
            father.id(),
        )
        .ok();
    graph
        .create_edge(
            "is-mother",
            Attributes::default(),
            grand_mother.id(),
            father.id(),
        )
        .ok();

    (graph, father, mother, son, daughter)
}
