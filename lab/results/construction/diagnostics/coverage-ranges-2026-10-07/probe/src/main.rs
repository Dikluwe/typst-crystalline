#[path = "/repos/Antigravity/typst-crystalline/03_infra/src/fonts/coverage.rs"]
mod coverage;
use std::{hint::black_box, time::Instant, path::PathBuf};
use typst_core::entities::font_book::Coverage;
fn legacy(face: &ttf_parser::Face) -> Coverage {
 let mut points=Vec::new();
 if let Some(cmap)=face.tables().cmap { for st in cmap.subtables { if st.is_unicode() { st.codepoints(&mut |c| points.push(c)); } } }
 Coverage::from_codepoints(points)
}
fn main() {
 assert!(!cfg!(debug_assertions));
 let mode=std::env::args().nth(1).unwrap_or_else(|| "validate".into());
 if mode=="catalog" { catalog(); return; }
 let mut db=fontdb::Database::new();db.load_system_fonts();
 let mut inputs=Vec::new();
 for face in db.faces() { if let fontdb::Source::File(p)|fontdb::Source::SharedFile(p,_)=&face.source { inputs.push((p.clone(),face.index,"system")); } }
 let asset=PathBuf::from("/home/dikluwe/.cargo/git/checkouts/typst-assets-525e6d15ef7950cb/c0ae970/files/fonts");
 for (dir,kind) in [(asset,"embedded"),(PathBuf::from("03_infra/fixtures/fonts"),"fixture")] {
  for entry in std::fs::read_dir(dir).unwrap() { let p=entry.unwrap().path(); if p.is_file() { let bytes=std::fs::read(&p).unwrap();let n=ttf_parser::fonts_in_collection(&bytes).unwrap_or(1);for i in 0..n{if ttf_parser::Face::parse(&bytes,i).is_ok(){inputs.push((p.clone(),i,kind));}} } }
 }
 let mut data=Vec::new();let mut rows=Vec::new();
 for (p,index,kind) in inputs { let bytes=std::fs::read(&p).unwrap();let face=ttf_parser::Face::parse(&bytes,index).unwrap();
  assert_eq!(coverage::extract(&face),legacy(&face),"{} face {}",p.display(),index);
  rows.push(serde_json::json!({"path":p,"index":index,"kind":kind,"members":legacy(&face).iter().count()}));data.push((bytes,index)); }
 if mode=="validate" {println!("{}",serde_json::json!({"all_equal":true,"faces":rows}));return;}
 let mut samples=Vec::new();
 for round in 0..3 { for pair in 0..12 { let mut old=0;let mut new=0;
  for step in 0..2 { let is_new=(step+pair+round)%2==0;let tick=Instant::now();
   for (bytes,index) in &data { let face=ttf_parser::Face::parse(bytes,*index).unwrap();black_box(if is_new{coverage::extract(black_box(&face))}else{legacy(black_box(&face))}); }
   let ns=tick.elapsed().as_nanos();if is_new{new=ns}else{old=ns}
  } samples.push(serde_json::json!({"round":round+1,"pair":pair,"legacy_ns":old,"current_ns":new}));
 }}
 println!("{}",serde_json::json!({"all_equal":true,"faces":rows,"samples":samples,"includes_face_parse":true,"excludes_file_read":true}));
}

fn old_catalog() -> (Vec<typst_infra::fonts::FontSlot>, typst_core::entities::font_book::FontBook) {
 let mut db=fontdb::Database::new();db.load_system_fonts();
 let mut slots=Vec::new();let mut book=typst_core::entities::font_book::FontBook::new();
 for face in db.faces() {
  let path=match &face.source {fontdb::Source::File(p)|fontdb::Source::SharedFile(p,_)=>p.clone(),fontdb::Source::Binary(_)=>continue};
  let info=db.with_face_data(face.id,|data,index| {
   let mut info=typst_infra::fonts::font_info_from_bytes(data,index)?;
   let parsed=ttf_parser::Face::parse(data,index).ok()?;info.coverage=legacy(&parsed);Some(info)
  }).flatten();
  if let Some(info)=info { slots.push(typst_infra::fonts::FontSlot::new(path,face.index));book.push(info); }
 }
 (slots,book)
}
fn catalog() {
 let old=old_catalog();let new=typst_infra::fontdb::load_system_fonts();
 assert!(old.1==new.1);assert_eq!(old.0.len(),new.0.len());
 for (a,b) in old.0.iter().zip(&new.0) { assert_eq!((&a.path,a.index),(&b.path,b.index)); }
 let count=new.0.len();drop(old);drop(new);let mut samples=Vec::new();
 for round in 0..3 { for pair in 0..12 { let mut old=0;let mut new=0;
  for step in 0..2 { let is_new=(step+round+pair)%2==0;let tick=Instant::now();
   black_box(if is_new{typst_infra::fontdb::load_system_fonts()}else{old_catalog()});
   let ns=tick.elapsed().as_nanos();if is_new{new=ns}else{old=ns}
  } samples.push(serde_json::json!({"round":round+1,"pair":pair,"legacy_ns":old,"current_ns":new}));
 }}
 println!("{}",serde_json::json!({"all_books_slots_equal":true,"system_faces":count,"samples":samples,"includes_discovery_file_read_metadata_coverage":true}));
}
