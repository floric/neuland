use super::attributes::Attributes;
use nanoid::nanoid;
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
    pub fn new(relation: String, from: String, to: String, attributes: Attributes) -> Edge {
        Edge {
            id: nanoid!(),
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

    pub fn attributes(&self) -> &Attributes {
        &self.attributes
    }
}
