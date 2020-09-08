use multimap::MultiMap;

use crate::query::matcher::Matcher;

pub struct Query {
    attributes: MultiMap<String, Box<dyn Matcher>>,
}

impl Query {
    pub fn select() -> Query {
        Query {
            attributes: MultiMap::new(),
        }
    }

    pub fn with_att(&mut self, key: &str, matcher: Box<dyn Matcher>) -> &Query {
        self.attributes.insert(key.to_string(), matcher);
        self
    }

    pub fn attributes(&self) -> &MultiMap<String, Box<dyn Matcher>> {
        &self.attributes
    }
}
