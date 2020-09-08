use multimap::MultiMap;

pub trait Matcher {
    fn apply(&self, arg: &str) -> bool;
}
pub struct AttributeMatcher {
    matcher: Box<dyn Matcher>,
}

impl AttributeMatcher {
    pub fn new(matcher: Box<dyn Matcher>) -> AttributeMatcher {
        AttributeMatcher { matcher }
    }

    pub fn matcher(&self) -> &dyn Matcher {
        self.matcher.as_ref()
    }
}

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
