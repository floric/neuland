use nanoid::nanoid;
use neuland::model::Graph;
use rand::{prelude::StdRng, Rng, SeedableRng};

pub fn generate_random_graph(node_count: usize, edge_count: usize) -> Graph {
    let mut graph = Graph::default();
    let mut node_ids: Vec<String> = vec![];
    let mut rng = StdRng::seed_from_u64(123456);

    for _ in 0..node_count {
        let n = graph.create_default_node().id().to_string();
        graph.attributes_of_node_mut(&n).unwrap().set("abc", "123");
        node_ids.push(n);
    }
    for _ in 0..edge_count {
        let destination_id = get_random_node_id(&mut rng, &node_ids);
        let source_id = get_random_node_id(&mut rng, &node_ids);
        graph
            .create_default_edge(&nanoid!(), &source_id, &destination_id)
            .ok();
    }

    graph
}

fn get_random_node_id(rng: &mut StdRng, node_ids: &[String]) -> String {
    node_ids
        .get(rng.gen_range(0, node_ids.len()))
        .unwrap()
        .to_string()
}
