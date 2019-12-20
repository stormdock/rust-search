use tantivy::collector::TopDocs;
use tantivy::query::QueryParser;
// use tantivy::schema::*;
use std::fs::{create_dir, remove_dir_all};
use std::path::Path;
use tantivy::schema::{Document, Schema, STORED, TEXT};
use tantivy::{doc, Index, ReloadPolicy};

fn main() -> tantivy::Result<()> {
    let directory = Path::new("/tmp/tantivy/idxhn");
    let dir_exists = directory.exists();
    if dir_exists {
        println!("{}", "Found the tantivy index directory")
    }

    let index = Index::open_in_dir(&directory)?;
    let schema = index.schema();

    Ok(())
}
