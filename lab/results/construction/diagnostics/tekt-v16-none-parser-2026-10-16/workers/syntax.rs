use std::{env, fs};
use tree_sitter::Parser;

fn main() {
    let mut parser = Parser::new();
    parser.set_language(&tree_sitter_rust::LANGUAGE.into()).unwrap();
    for path in env::args().skip(1) {
        let source = fs::read_to_string(&path).unwrap();
        let tree = parser.parse(&source, None).unwrap();
        assert!(!tree.root_node().has_error(), "{path}");
        println!("{}", serde_json::json!({"file":path,"ast":tree.root_node().to_sexp()}));
    }
}
