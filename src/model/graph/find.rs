use crate::model::attributes::HasAttributes;

pub fn find_by_attributes<'a, T, I, F>(items: I, key: &str, matcher: F) -> Vec<&'a T>
where
    T: HasAttributes,
    I: Iterator<Item = &'a T>,
    F: Fn(&&'a String) -> bool,
{
    items
        .filter(|node| node.get_attributes().get(key).filter(&matcher).is_some())
        .collect()
}
