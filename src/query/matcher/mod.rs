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

pub mod eq_matcher;
