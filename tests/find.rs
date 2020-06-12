use neuland::model::Attributes;
use neuland::model::Graph;

#[test]
fn test_find_by_attributes() {
    let mut graph = Graph::default();
    let mut attributes_a = Attributes::default();
    attributes_a.set("test", String::from("a"));
    let mut attributes_b = Attributes::default();
    attributes_b.set("test", String::from("b"));
    let node_a = graph.create_node(attributes_a).clone();
    let node_b = graph.create_node(attributes_b).clone();
    let node_c = graph.create_node(Attributes::default()).clone();

    let matches = graph.find_by_attribute("test", "a");
    assert_eq!(1, matches.len());
    assert!(matches.contains(&&node_a));
    assert!(!matches.contains(&&node_b));
    assert!(!matches.contains(&&node_c));
}

#[test]
fn test_find_by_attributes_with_other_value() {
    let mut graph = Graph::default();
    let mut attributes = Attributes::default();
    attributes.set("test", String::from("a"));
    graph.create_node(attributes);

    let matches = graph.find_by_attribute("test", "b");
    assert!(matches.is_empty());
}

#[test]
fn test_find_by_attributes_with_no_result() {
    let mut graph = Graph::default();
    graph.create_node(Attributes::default());

    let matches = graph.find_by_attribute("x", "a");
    assert!(matches.is_empty());
}

#[test]
fn test_find_by_matcher() {
    let mut graph = Graph::default();
    let mut attributes_a = Attributes::default();
    attributes_a.set("test", String::from("a"));
    let mut attributes_b = Attributes::default();
    attributes_b.set("test", String::from("b"));
    let node_a = graph.create_node(attributes_a).clone();
    let node_b = graph.create_node(attributes_b).clone();
    let node_c = graph.create_node(Attributes::default()).clone();

    let matches = graph.find_by_matcher("test", |x| x.starts_with("a"));
    assert_eq!(1, matches.len());
    assert!(matches.contains(&&node_a));
    assert!(!matches.contains(&&node_b));
    assert!(!matches.contains(&&node_c));
}
