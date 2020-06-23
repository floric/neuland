use crate::model::{Attributes, Graph};
use nanoid::nanoid;
use quick_xml::events::{BytesStart, BytesText, Event};
use quick_xml::{Error, Reader};
use std::str;

pub fn import<'a>(path: &str) -> Result<Graph, Error> {
    let mut graph = Graph::default();

    let file_reader = Reader::from_file(path);
    if file_reader.is_err() {
        return Err(file_reader.err().unwrap());
    }

    let mut reader = file_reader.unwrap();
    let mut buf = Vec::new();
    let mut created_node: Option<String> = None;
    let mut created_edge: Option<String> = None;
    let mut key: Option<String> = None;

    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::Empty(ref e)) => match e.name() {
                b"node" => created_node = create_node(e, &mut graph),
                b"edge" => created_edge = create_edge(e, &mut graph),
                _ => (),
            },
            Ok(Event::Start(ref e)) => match e.name() {
                b"node" => created_node = create_node(e, &mut graph),
                b"edge" => created_edge = create_edge(e, &mut graph),
                b"data" => key = get_attribute(b"key", e),
                _ => (),
            },
            Ok(Event::End(ref e)) => match e.name() {
                b"node" => created_node = None,
                b"edge" => created_edge = None,
                b"data" => key = None,
                _ => (),
            },
            Ok(Event::Text(ref e)) => create_data_attributes(
                &e,
                &mut key,
                &mut created_node,
                &mut created_edge,
                &mut graph,
            ),
            Ok(Event::Eof) => break,
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            _ => (),
        }

        buf.clear();
    }

    Result::Ok(graph)
}

fn create_data_attributes(
    e: &BytesText,
    key: &mut Option<String>,
    created_node: &Option<String>,
    created_edge: &Option<String>,
    graph: &mut Graph,
) {
    let value = String::from_utf8(e.to_vec());
    if value.is_ok() && key.is_some() {
        let val = value.unwrap();
        let k = key.clone().unwrap();
        if created_node.is_some() {
            let attrs = graph.attributes_of_node_mut(&created_node.clone().unwrap());
            if attrs.is_some() {
                attrs.unwrap().set(&k, &val);
            }
        } else if created_edge.is_some() {
            let attrs = graph.attributes_of_edge_mut(&created_edge.clone().unwrap());
            if attrs.is_some() {
                attrs.unwrap().set(&k, &val);
            }
        }
    }
}

fn create_node(e: &BytesStart, graph: &mut Graph) -> Option<String> {
    let id = get_attribute(b"id", e).unwrap();
    Option::Some(
        graph
            .create_node(&id, Attributes::default())
            .id()
            .to_string(),
    )
}

fn create_edge(e: &BytesStart, graph: &mut Graph) -> Option<String> {
    let id = get_attribute(b"id", e).unwrap();
    let source_id = get_attribute(b"source", e).unwrap();
    let target_id = get_attribute(b"target", e).unwrap();

    let edge_id = graph
        .create_edge(
            &id,
            "unknown",
            Attributes::default(),
            &source_id,
            &target_id,
        )
        .unwrap();

    graph.get_edge(&edge_id).map(|x| x.id().to_string())
}

fn get_attribute(key: &[u8], e: &BytesStart) -> Option<String> {
    e.attributes()
        .filter_map(|x| x.ok())
        .filter(|x| x.key == key)
        .map(|x| String::from_utf8(x.value.into()).ok())
        .next()
        .unwrap_or(Option::Some(String::from(nanoid!())))
}
