use super::attributes::{Attributes, HasAttributes};
use nanoid::nanoid;
use std::hash::Hash;

#[derive(Clone, Hash, PartialEq, Eq)]
pub struct Node {
    id: String,
    attributes: Attributes,
}

impl Node {
    pub fn new(attributes: Attributes) -> Node {
        Node {
            id: nanoid!(),
            attributes,
        }
    }

    pub fn id(&self) -> &String {
        &self.id
    }

    pub fn attributes(&self) -> &Attributes {
        &self.attributes
    }
}

impl HasAttributes for Node {
    fn get_attributes(&self) -> &Attributes {
        &self.attributes
    }
}
