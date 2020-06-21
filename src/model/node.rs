use super::attributes::{Attributes, HasAttributes};
use std::hash::Hash;

#[derive(Clone, Hash, PartialEq, Eq)]
pub struct Node {
    id: String,
    attributes: Attributes,
}

impl Node {
    pub fn new(id: &str, attributes: Attributes) -> Node {
        Node {
            id: String::from(id),
            attributes,
        }
    }

    pub fn id(&self) -> &String {
        &self.id
    }
}

impl HasAttributes for Node {
    fn attributes_mut(&mut self) -> &mut Attributes {
        &mut self.attributes
    }

    fn attributes(&self) -> &Attributes {
        &self.attributes
    }
}
