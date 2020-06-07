use super::attribute::Attribute;
use nanoid::nanoid;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};

#[derive(PartialEq, Eq, Clone)]
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
