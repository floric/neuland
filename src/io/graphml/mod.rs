use crate::model::{Attributes, Graph};
use nanoid::nanoid;
use quick_xml::events::{BytesStart, Event};
use quick_xml::{Error, Reader};

pub fn import(path: &str) -> Result<Graph, Error> {
    let mut graph = Graph::default();

    let file_reader = Reader::from_file(path);
    if file_reader.is_err() {
        return Result::Err(file_reader.err().unwrap());
    }

    let mut reader = file_reader.unwrap();
    reader.trim_text(true);

    let mut buf = Vec::new();

    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::Empty(ref e)) => match e.name() {
                b"node" => {
                    let id = get_attribute(b"id", e).unwrap();
                    graph.create_node(&id, Attributes::default());
                }
                b"edge" => {
                    let id = get_attribute(b"id", e).unwrap();
                    let source_id = get_attribute(b"source", e).unwrap();
                    let target_id = get_attribute(b"target", e).unwrap();

                    graph
                        .create_edge(
                            &id,
                            "unknown",
                            Attributes::default(),
                            &source_id,
                            &target_id,
                        )
                        .unwrap();
                }
                _ => (),
            },
            Ok(Event::Start(ref e)) => {
                match e.name() {
                    b"node" => {
                        let id = get_attribute(b"id", e).unwrap();
                        graph.create_node(&id, Attributes::default());
                    }
                    b"edge" => {
                        let id = get_attribute(b"id", e).unwrap();
                        let source_id = get_attribute(b"source", e).unwrap();
                        let target_id = get_attribute(b"target", e).unwrap();

                        graph
                            .create_edge(
                                &id,
                                "unknown",
                                Attributes::default(),
                                &source_id,
                                &target_id,
                            )
                            .unwrap();
                    }
                    b"data" => {
                        // let source_id = get_attribute(b"key", e).unwrap();
                    }
                    _ => (),
                }
            }
            Ok(Event::Eof) => break,
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            _ => (),
        }

        buf.clear();
    }

    Result::Ok(graph)
}

fn process_data(e: &BytesStart, graph: &mut Graph) {
    match e.name() {
        b"data" => {
            // let source_id = get_attribute(b"key", e).unwrap();
        }
        _ => (),
    }
}
fn get_attribute(key: &[u8], e: &BytesStart) -> Option<String> {
    e.attributes()
        .filter(|x| x.is_ok())
        .map(|x| x.ok().unwrap())
        .filter(|x| x.key == key)
        .map(|x| String::from_utf8(x.value.into()).ok())
        .next()
        .unwrap_or(Option::Some(String::from(nanoid!())))
}
