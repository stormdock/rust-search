use serde_json::{Map, Value};

use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

use std::path::Path;

use tantivy::schema::{DocParsingError, Document};
use tantivy::Index;

use crossbeam::crossbeam_channel::{unbounded, Receiver};

fn process_lines(r: Receiver<String>) {
    let msg = r.recv().unwrap();
    println!("{}", msg);
    let _x = parse_document(&msg);
}

fn read_file_to_buffer(filename: String) {
    let f = File::open(filename).unwrap();
    let file = BufReader::new(&f);

    for (_num, line) in file.lines().enumerate() {
        // Create a channel of unbounded capacity.
        let (s, r) = unbounded();

        let l = line.unwrap();
        // Send a message into the channel.
        s.send(l).unwrap();
        process_lines(r);
    }
}

pub fn parse_document(doc_json: &str) -> Result<Document, DocParsingError> {
    let json_obj: Map<String, Value> = serde_json::from_str(doc_json).map_err(|_| {
        let doc_json_sample: String = if doc_json.len() < 20 {
            String::from(doc_json)
        } else {
            format!("{:?}...", &doc_json[0..20])
        };
        DocParsingError::NotJSON(doc_json_sample)
    })?;

    let doc = Document::default();

    let mut m = Map::new();
    for (json_key, json_value) in json_obj.iter() {
        let my_json_value = json_value.clone();
        m.insert(json_key.to_string(), my_json_value);
    }
    let x: Value = m.into();
    println!("{}", x);
    Ok(doc)
}

fn main() -> tantivy::Result<()> {
    let json_file = String::from("/tmp13/rust-search/data/tanhn01.txt");

    let directory = Path::new("/tmp/tantivy/idxhn");
    let dir_exists = directory.exists();
    if dir_exists {
        println!("{}", "Found the tantivy index directory")
    }

    let index = Index::open_in_dir(&directory)?;
    let schema = index.schema();

    for (field, field_entry) in schema.fields() {
        println!("{} {}", field.field_id(), field_entry.name());
    }

    read_file_to_buffer(json_file.to_string());
    Ok(())
}
