use super::attribute::Attribute;
use nanoid::nanoid;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};

#[derive(Clone, Eq)]
pub struct Node {
    id: String,
    attributes: HashSet<Attribute>,
}

impl Node {
    pub fn new(attributes: HashSet<Attribute>) -> Node {
        Node {
            id: nanoid!(),
            attributes,
        }
    }

    pub fn id(&self) -> &String {
        &self.id
    }
}

impl Hash for Node {
    fn hash<H: Hasher>(&self, _: &mut H) {}
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
