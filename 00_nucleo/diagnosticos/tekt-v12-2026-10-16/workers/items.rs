use std::{env,fs};
use tree_sitter::Parser;
use serde_json::json;
fn main(){
 let mut parser=Parser::new();parser.set_language(&tree_sitter_rust::LANGUAGE.into()).unwrap();let mut rows=vec![];
 for path in env::args().skip(1){let source=fs::read_to_string(&path).unwrap();let tree=parser.parse(&source,None).unwrap();assert!(!tree.root_node().has_error());let mut walk=tree.root_node().walk();let mut attrs=vec![];
 for node in tree.root_node().named_children(&mut walk){if node.kind()=="attribute_item"{attrs.push(node.start_byte());continue;}
 let start=attrs.first().copied().unwrap_or(node.start_byte());attrs.clear();let name=node.child_by_field_name("name").or_else(||node.child_by_field_name("type")).map(|n|n.utf8_text(source.as_bytes()).unwrap()).unwrap_or("");
 rows.push(json!({"source":path,"kind":node.kind(),"name":name,"start":start,"node_start":node.start_byte(),"end":node.end_byte(),"line":node.start_position().row+1,"text":&source[start..node.end_byte()]}));
 }}println!("{}",serde_json::to_string(&rows).unwrap());
}
