use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};

#[derive(Default, Clone, Eq)]
pub struct Attributes {
    values: HashMap<String, String>,
}

impl Attributes {
    pub fn value_count(&self) -> usize {
        self.values.len()
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.values.get(key)
    }

    pub fn set(&mut self, key: &str, value: &str) {
        self.values.insert(key.to_string(), String::from(value));
    }

    pub fn remove(&mut self, key: &str) {
        self.values.remove(key);
    }

    pub fn keys(&self) -> HashSet<&String> {
        self.values.keys().collect::<HashSet<_>>()
    }
}

pub trait HasAttributes {
    fn attributes_mut(&mut self) -> &mut Attributes;

    fn attributes(&self) -> &Attributes;
}

impl Hash for Attributes {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        for att in self.values.iter() {
            att.hash(hasher);
        }
    }
}

impl PartialEq for Attributes {
    fn eq(&self, other: &Self) -> bool {
        other.values.eq(&self.values)
    }
}

#[cfg(test)]
mod tests {
    use super::Attributes;

    #[test]
    fn test_attributes_creation() {
        let attributes = Attributes::default();

        assert_eq!(0, attributes.value_count());
    }

    #[test]
    fn test_set_and_get_value() {
        let mut attributes = Attributes::default();
        attributes.set("a", "b");

        assert_eq!(attributes.get("a").unwrap(), "b");
    }

    #[test]
    fn test_unknown_value() {
        let mut attributes = Attributes::default();
        attributes.set("x", "y");

        assert!(attributes.get("a").is_none());
    }

    #[test]
    fn test_remove_value() {
        let mut attributes = Attributes::default();
        attributes.set("a", "b");
        attributes.remove("a");

        assert!(attributes.get("a").is_none());
    }
}
