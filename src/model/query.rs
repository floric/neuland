use multimap::MultiMap;

use crate::query::matcher::AttributeMatcher;

pub struct Query {
    attributes: MultiMap<String, AttributeMatcher>,
}

impl Query {
    pub fn select(&self) -> &Query {
        self
    }

    pub fn with_att<F>(&mut self, key: String, matcher: AttributeMatcher) -> &Query {
        self.attributes.insert(key, matcher);
        self
    }

    pub fn attributes(&self) -> &MultiMap<String, AttributeMatcher> {
        &self.attributes
    }
}
