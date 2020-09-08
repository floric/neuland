use super::Matcher;

pub struct EqMatcher {
    value: String,
}

impl EqMatcher {
    pub fn new(value: &str) -> EqMatcher {
        EqMatcher {
            value: String::from(value),
        }
    }
}

impl Matcher for EqMatcher {
    fn apply(&self, arg: &str) -> bool {
        arg == self.value
    }
}
