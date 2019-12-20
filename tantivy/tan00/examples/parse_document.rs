// use tantivy::schema::field_type::ValueParsingError;

use std::collections::BTreeMap;
use std::collections::HashMap;
use std::sync::Arc;

use serde::de::{SeqAccess, Visitor};
use serde::ser::SerializeSeq;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json::{self, Map as JsonObject, Value as JsonValue};
use std::fmt;

use std::path::Path;
use tantivy::Index;

//use tantivy::schema::field_value::FieldValue;

use tantivy::schema::{DocParsingError, Document};
// use tantivy::schema::DocParsingError;

//
/// Build a document object from a json-object.
pub fn parse_document(doc_json: &str) -> Result<Document, DocParsingError> {
    let json_obj: JsonObject<String, JsonValue> = serde_json::from_str(doc_json).map_err(|_| {
        let doc_json_sample: String = if doc_json.len() < 20 {
            String::from(doc_json)
        } else {
            format!("{:?}...", &doc_json[0..20])
        };
        DocParsingError::NotJSON(doc_json_sample)
    })?;

    let mut doc = Document::default();
    for (field_name, json_value) in json_obj.iter() {
        /*
                let field = self
                    .get_field(field_name)
                    .ok_or_else(|| DocParsingError::NoSuchFieldInSchema(field_name.clone()))?;
                let field_entry = self.get_field_entry(field);
                let field_type = field_entry.field_type();

                match *json_value {
                    JsonValue::Array(ref json_items) => {
                        for json_item in json_items {
                            let value = field_type
                                .value_from_json(json_item)
                                .map_err(|e| DocParsingError::ValueError(field_name.clone(), e))?;
                            doc.add(FieldValue::new(field, value));
                        }
                    }
                    _ => {
                        let value = field_type
                            .value_from_json(json_value)
                            .map_err(|e| DocParsingError::ValueError(field_name.clone(), e))?;
                        doc.add(FieldValue::new(field, value));
                    }
                }
        */
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
    let doc = schema.parse_document(doc_json).unwrap();

    Ok(())
}
