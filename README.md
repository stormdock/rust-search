### Examples

basic_search is out of the box from tantivy

basic_search1 has the added capability of being
able to see the output of the index writer in
the directory /tmp/tantivy.  Where as basic_search
uses **tempfile::TempDir** and the indexed document
data is destroyed when the program exits.
