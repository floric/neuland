use super::attribute::Attribute;
use nanoid::nanoid;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};

#[derive(Clone, Eq)]
pub struct Edge {
    id: String,
    relation: String,
    from: String,
    to: String,
    attributes: HashSet<Attribute>,
}

impl Edge {
    pub fn new(relation: String, from: String, to: String, attributes: HashSet<Attribute>) -> Edge {
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
}

impl Hash for Edge {
    fn hash<H: Hasher>(&self, _: &mut H) {}
}

impl PartialEq for Edge {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
