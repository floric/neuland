use super::attributes::{Attributes, HasAttributes};
use std::hash::Hash;

#[derive(Clone, Hash, PartialEq, Eq)]
pub struct Edge {
    id: String,
    relation: String,
    from: String,
    to: String,
    attributes: Attributes,
}

impl Edge {
    pub fn new(
        id: String,
        relation: String,
        from: String,
        to: String,
        attributes: Attributes,
    ) -> Edge {
        Edge {
            id,
            relation,
            from,
            to,
            attributes,
        }
    }

    pub fn id(&self) -> &String {
        &self.id
    }

    pub fn relation(&self) -> &String {
        &self.relation
    }

    pub fn from_id(&self) -> &String {
        &self.from
    }

    pub fn to_id(&self) -> &String {
        &self.to
    }
}

impl HasAttributes for Edge {
    fn attributes_mut(&mut self) -> &mut Attributes {
        &mut self.attributes
    }

    fn attributes(&self) -> &Attributes {
        &self.attributes
    }
}
