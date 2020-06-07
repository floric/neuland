pub fn build_relation_key(source_id: &str, destination_id: &str) -> String {
    format!("{}-{}", source_id, destination_id)
}
