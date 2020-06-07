pub fn build_relation_key(source_id: &str, destination_id: &str) -> String {
    format!("{}-{}", source_id, destination_id)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_create_key() {
        let key = super::build_relation_key("a", "b");
        assert_eq!("a-b", key);
    }

    #[test]
    fn test_direction_of_key() {
        assert_ne!(
            super::build_relation_key("a", "b"),
            super::build_relation_key("b", "a")
        );
    }
}
