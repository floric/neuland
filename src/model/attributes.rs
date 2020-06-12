use std::collections::HashMap;
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

    pub fn set(&mut self, key: &str, value: String) {
        self.values.insert(key.to_string(), value);
    }

    pub fn remove(&mut self, key: &str) {
        self.values.remove(key);
    }
}

pub trait HasAttributes {
    fn get_attributes(&self) -> &Attributes;
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
