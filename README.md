### Examples

basic_search is out of the box from tantivy

basic_search1 has the added capability of being
able to see the output of the index writer in
the directory /tmp/tantivy.  Where as basic_search
uses **tempfile::TempDir** and the indexed document
data is destroyed when the program exits.

##### New work

* Create a tantivy document
* convert it to json
* write it to redis using a redis pool
* read it from redis using a redis pool
* convert the json to a tantivy document
* compare the 2 documents
