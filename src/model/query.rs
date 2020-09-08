use multimap::MultiMap;

use crate::query::matcher::Matcher;

pub struct Query {
    attributes: MultiMap<String, Box<dyn Matcher>>,
}

impl Query {
    pub fn select(&self) -> &Query {
        self
    }

    pub fn with_att<F>(&mut self, key: String, matcher: Box<dyn Matcher>) -> &Query {
        self.attributes.insert(key, matcher);
        self
    }

    pub fn attributes(&self) -> &MultiMap<String, Box<dyn Matcher>> {
        &self.attributes
    }
}
