use std::hash::{Hash, Hasher};

#[derive(PartialEq, Eq, Clone)]
pub struct Attribute {}

impl Hash for Attribute {
    fn hash<H: Hasher>(&self, _: &mut H) {}
}
