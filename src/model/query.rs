use multimap::MultiMap;

use crate::query::matcher::Matcher;

pub struct Query {
    attributes: MultiMap<String, Box<dyn Matcher>>,
    paths: Vec<Vec<String>>,
}

impl Query {
    pub fn new() -> Query {
        Query {
            attributes: MultiMap::new(),
            paths: vec![],
        }
    }

    pub fn select(&mut self) -> &mut Query {
        self
    }

    pub fn with_att(&mut self, key: &str, matcher: Box<dyn Matcher>) -> &mut Query {
        self.attributes.insert(key.to_string(), matcher);
        self
    }

    pub fn with_path(&mut self, path: &[&str]) -> &mut Query {
        self.paths
            .push(path.iter().map(|x| x.to_string()).collect());
        self
    }

    pub fn attributes(&self) -> &MultiMap<String, Box<dyn Matcher>> {
        &self.attributes
    }

    pub fn paths(&self) -> &Vec<Vec<String>> {
        &self.paths
    }
}
