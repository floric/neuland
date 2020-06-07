use super::attribute::Attribute;
use nanoid::nanoid;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};

#[derive(PartialEq, Eq, Clone)]
pub struct Edge {
    id: String,
    from: String,
    to: String,
    attributes: HashSet<Attribute>,
}

impl Edge {
    pub fn new(from: String, to: String, attributes: HashSet<Attribute>) -> Edge {
        Edge {
            id: nanoid!(),
            from,
            to,
            attributes,
        }
    }

    pub fn id(&self) -> &String {
        &self.id
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
