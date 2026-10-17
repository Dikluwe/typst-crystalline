use serde_json::{json, Value};
use std::{env,fs};
use tree_sitter::{Node,Parser};
fn text<'a>(n:Node<'_>,s:&'a str)->&'a str{n.utf8_text(s.as_bytes()).unwrap()}
fn visit(n:Node<'_>,s:&str,rows:&mut Vec<Value>){
 if n.kind()=="match_arm" {
  let mut c=n.walk();let p=n.children(&mut c).find(|p|p.kind()=="match_pattern").unwrap();
  if text(p,s)=="_" {
   let mut c=n.parent().unwrap().walk();let patterns:Vec<String>=n.parent().unwrap().children(&mut c).filter(|a|a.kind()=="match_arm").map(|a|{let mut c=a.walk();let p=a.children(&mut c).find(|p|p.kind()=="match_pattern").unwrap();text(p,s).to_string()}).filter(|p|p!="_").collect();
   for en in ["Content","Value","MathStyleKind","FrameItem"] {if !patterns.is_empty() && patterns.iter().all(|p|p.starts_with(&format!("{en}::"))){rows.push(json!({"source":"01_core/src/compiler/eval/tests.rs","line":n.start_position().row+1,"column":n.start_position().column+1,"enum":en,"patterns":patterns}));}}
  }
 }
 let mut c=n.walk();for child in n.children(&mut c){visit(child,s,rows)}
}
fn all_matches(n:Node<'_>,s:&str,rows:&mut Vec<Value>){
 if n.kind()=="match_expression" {
  let block=n.child_by_field_name("body").unwrap();let mut c=block.walk();
  let arms:Vec<Value>=block.children(&mut c).filter(|a|a.kind()=="match_arm").map(|a|{let mut c=a.walk();let p=a.children(&mut c).find(|p|p.kind()=="match_pattern").unwrap();json!({"line":a.start_position().row+1,"column":a.start_position().column+1,"pattern":text(p,s)})}).collect();
  rows.push(json!({"line":n.start_position().row+1,"end_line":n.end_position().row+1,"column":n.start_position().column+1,"scrutinee":text(n.child_by_field_name("value").unwrap(),s),"arms":arms}));
 }
 let mut c=n.walk();for child in n.children(&mut c){all_matches(child,s,rows)}
}
fn main(){let path=env::args().nth(1).unwrap();let s=fs::read_to_string(path).unwrap();let mut p=Parser::new();p.set_language(&tree_sitter_rust::LANGUAGE.into()).unwrap();let t=p.parse(&s,None).unwrap();assert!(!t.root_node().has_error());let mut rows=Vec::new();if env::args().nth(2).as_deref()==Some("matches"){all_matches(t.root_node(),&s,&mut rows)}else{visit(t.root_node(),&s,&mut rows)};println!("{}",serde_json::to_string(&rows).unwrap());}
