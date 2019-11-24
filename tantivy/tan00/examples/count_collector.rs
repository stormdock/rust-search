#[macro_use]
extern crate tantivy;
use tantivy::collector::Count;
use tantivy::query::QueryParser;
use tantivy::schema::{Schema, TEXT};
use tantivy::{Index, Result};

// without Result above it would be this:
// fn main() -> tantivy::Result<()> {
fn main() -> Result<()> {
    let mut schema_builder = Schema::builder();
    let title = schema_builder.add_text_field("title", TEXT);
    let schema = schema_builder.build();
    let index = Index::create_in_ram(schema);
    {
        let mut index_writer = index.writer(3_000_000)?;
        index_writer.add_document(doc!(
            title => "The Name of diary the Wind",
        ));
        index_writer.add_document(doc!(
            title => "The Diary of Muadib",
        ));
        index_writer.add_document(doc!(
            title => "A Dairy Cow",
        ));
        index_writer.add_document(doc!(
            title => "The Diary of a Young Girl",
        ));
        index_writer.commit().unwrap();
    }

    let reader = index.reader()?;
    let searcher = reader.searcher();

    {
        let query_parser = QueryParser::for_index(&index, vec![title]);
        let query = query_parser.parse_query("diary")?;
        let count = searcher.search(&query, &Count).unwrap();

        assert_eq!(count, 3);
    }

    Ok(())
}
