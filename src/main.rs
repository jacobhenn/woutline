#![feature(let_chains)]

use std::env;

use lopdf::Document;

fn main() {
    let doc_path = env::args().nth(1).unwrap();
    let doc = Document::load(doc_path).unwrap();
    for bookmark_id in doc.bookmarks {
        let bookmark: Bookmark = doc.bookmark_table[bookmark_id];
    }
}
