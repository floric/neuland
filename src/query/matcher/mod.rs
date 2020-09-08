pub trait Matcher {
    fn apply(&self, arg: &str) -> bool;
}

pub mod eq_matcher;
