use neuland::model::Attributes;

#[test]
fn test_attributes_creation() {
    let attributes = Attributes::default();

    assert_eq!(0, attributes.value_count());
}

#[test]
fn test_set_and_get_value() {
    let mut attributes = Attributes::default();
    attributes.set("a", String::from("b"));

    assert_eq!(attributes.get("a").unwrap(), "b");
}

#[test]
fn test_unknown_value() {
    let mut attributes = Attributes::default();
    attributes.set("x", String::from("y"));

    assert!(attributes.get("a").is_none());
}

#[test]
fn test_remove_value() {
    let mut attributes = Attributes::default();
    attributes.set("a", String::from("b"));
    attributes.remove("a");

    assert!(attributes.get("a").is_none());
}
