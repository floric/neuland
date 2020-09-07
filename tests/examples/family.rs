use nanoid::nanoid;
use neuland::model::Attributes;
use neuland::model::Graph;
use neuland::model::Node;

struct Family {
    graph: Graph,
    father: Node,
    mother: Node,
    son: Node,
    daughter: Node,
    grand_father: Node,
    grand_mother: Node,
}

#[test]
pub fn test_family_tree_creation() {
    let family = create_graph();

    assert_eq!(6, family.graph.node_count());
    assert_eq!(12, family.graph.edge_count());
    assert!(family
        .graph
        .has_relation("is-child", family.son.id(), family.father.id()));
    assert!(family
        .graph
        .has_relation("is-mother", family.mother.id(), family.daughter.id()));
    assert!(family
        .graph
        .has_relation("is-mother", family.grand_mother.id(), family.father.id()));
    assert!(family
        .graph
        .has_relation("is-father", family.grand_father.id(), family.father.id()));
    assert!(family
        .graph
        .has_relation("is-child", family.father.id(), family.grand_father.id()));
    assert!(family
        .graph
        .has_relation("is-child", family.father.id(), family.grand_mother.id()));
    assert!(!family
        .graph
        .has_relation("is-father", family.father.id(), family.mother.id()));
    assert!(!family
        .graph
        .has_relation("is-mother", family.daughter.id(), family.mother.id()));
}

#[test]
pub fn test_family_tree_find() {
    let family = create_graph();

    let children = family.graph.find_nodes_by_path(&vec!["is-child"]);
    let fathers = family.graph.find_nodes_by_path(&vec!["is-father"]);
    let mothers = family.graph.find_nodes_by_path(&vec!["is-mother"]);
    let grand_mothers = family
        .graph
        .find_nodes_by_path(&vec!["is-mother", "is-father"]);
    let unknowns = family.graph.find_nodes_by_path(&vec!["is-x"]);

    assert_eq!(children.len(), 3);
    assert_eq!(fathers.len(), 2);
    assert_eq!(mothers.len(), 2);
    assert_eq!(grand_mothers.len(), 1);
    assert!(unknowns.is_empty());
}

fn create_graph() -> Family {
    let mut graph = Graph::default();

    let father = graph.create_node(&nanoid!(), Attributes::default()).clone();
    let mother = graph.create_node(&nanoid!(), Attributes::default()).clone();
    let son = graph.create_node(&nanoid!(), Attributes::default()).clone();
    let daughter = graph.create_node(&nanoid!(), Attributes::default()).clone();
    let grand_father = graph.create_node(&nanoid!(), Attributes::default()).clone();
    let grand_mother = graph.create_node(&nanoid!(), Attributes::default()).clone();
    let children: Vec<&str> = vec![daughter.id(), son.id()];
    let parents: Vec<&str> = vec![father.id(), mother.id()];
    let grand_parents: Vec<&str> = vec![grand_father.id(), grand_mother.id()];
    graph.create_edges("is-child", Attributes::default(), son.id(), &parents);
    graph.create_edges("is-child", Attributes::default(), daughter.id(), &parents);
    graph.create_edges("is-father", Attributes::default(), father.id(), &children);
    graph.create_edges("is-mother", Attributes::default(), mother.id(), &children);
    graph.create_edges(
        "is-child",
        Attributes::default(),
        father.id(),
        &grand_parents,
    );
    graph
        .create_edge(
            &nanoid!(),
            "is-father",
            Attributes::default(),
            grand_father.id(),
            father.id(),
        )
        .ok();
    graph
        .create_edge(
            &nanoid!(),
            "is-mother",
            Attributes::default(),
            grand_mother.id(),
            father.id(),
        )
        .ok();

    Family {
        graph,
        father,
        mother,
        son,
        daughter,
        grand_father,
        grand_mother,
    }
}
