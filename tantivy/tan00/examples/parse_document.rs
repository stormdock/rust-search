use serde_json::{self, Map as JsonObject, Value as JsonValue};
use std::path::Path;

use tantivy::schema::{DocParsingError, Document};
use tantivy::Index;

pub fn parse_document(doc_json: &str) -> Result<Document, DocParsingError> {
    let json_obj: JsonObject<String, JsonValue> = serde_json::from_str(doc_json).map_err(|_| {
        let doc_json_sample: String = if doc_json.len() < 20 {
            String::from(doc_json)
        } else {
            format!("{:?}...", &doc_json[0..20])
        };
        DocParsingError::NotJSON(doc_json_sample)
    })?;

    let doc = Document::default();
    for (field_name, json_value) in json_obj.iter() {
        println!("{} {}", field_name, json_value);
    }
    Ok(doc)
}

fn main() -> tantivy::Result<()> {
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

    let doc_json = r#"{
            "title": "my title",
            "id": 123456,
            "score": 4
    }"#;

    // this should work too !
    // let doc = schema.parse_document(doc_json).unwrap();
    let _doc = parse_document(doc_json).unwrap();

    Ok(())
}
