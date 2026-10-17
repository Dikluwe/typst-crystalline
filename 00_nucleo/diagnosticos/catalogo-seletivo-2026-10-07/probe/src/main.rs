#[path = "/repos/Antigravity/typst-crystalline/03_infra/src/fontdb/selective.rs"]
mod selective;
mod instrumented;
mod fonts {
 pub use typst_infra::fonts::{font_info_from_bytes, FontSlot};
 #[path = "/repos/Antigravity/typst-crystalline/03_infra/src/fonts/coverage.rs"]
 mod coverage;
 pub(crate) fn extract_coverage(face:&ttf_parser::Face)->typst_core::entities::font_book::Coverage{coverage::extract(face)}
}
use std::{hint::black_box,time::Instant};
use typst_core::entities::font_book::{FontBook,FontInfo};
fn legacy(data:&[u8],index:u32)->Option<FontInfo>{let mut info=fonts::font_info_from_bytes(data,index)?;let face=ttf_parser::Face::parse(data,index).ok()?;info.coverage=fonts::extract_coverage(&face);Some(info)}
fn old_catalog()->(Vec<fonts::FontSlot>,FontBook){
 let mut db=fontdb::Database::new();db.load_system_fonts();let mut slots=Vec::new();let mut book=FontBook::new();
 for face in db.faces(){let path=match &face.source{fontdb::Source::File(p)|fontdb::Source::SharedFile(p,_)=>p.clone(),_=>continue};let info=db.with_face_data(face.id,legacy).flatten();if let Some(info)=info{slots.push(fonts::FontSlot::new(path,face.index));book.push(info);}}
 (slots,book)
}
fn catalog(){let old=old_catalog();let new=typst_infra::fontdb::load_system_fonts();assert!(old.1==new.1);assert_eq!(old.0.len(),new.0.len());for(a,b)in old.0.iter().zip(&new.0){assert_eq!((&a.path,a.index),(&b.path,b.index));}let count=new.0.len();drop(old);drop(new);let mut samples=Vec::new();
 for round in 0..3{for pair in 0..12{let mut previous=0;let mut current=0;for step in 0..2{let is_new=(round+pair+step)%2==0;let t=Instant::now();black_box(if is_new{typst_infra::fontdb::load_system_fonts()}else{old_catalog()});let elapsed=t.elapsed().as_nanos();if is_new{current=elapsed}else{previous=elapsed}}samples.push(serde_json::json!({"round":round+1,"pair":pair,"legacy_ns":previous,"current_ns":current}));}}
 println!("{}",serde_json::json!({"all_books_slots_equal":true,"system_faces":count,"samples":samples,"includes_discovery_file_read_metadata_coverage":true}));
}
fn main(){assert!(!cfg!(debug_assertions));let mode=std::env::args().nth(1).unwrap_or_else(||"validate".into());if mode=="catalog"{catalog();return;}
 let mut db=fontdb::Database::new();db.load_system_fonts();let mut rows=Vec::new();let mut faces=Vec::new();
 for face in db.faces(){let path=match &face.source{fontdb::Source::File(p)|fontdb::Source::SharedFile(p,_)=>p.clone(),_=>continue};faces.push((face.id,path.clone(),face.index));}
 let mut inputs:Vec<_>=faces.iter().map(|(_,p,i)|(p.clone(),*i,"system")).collect();
 for(dir,kind)in[(std::path::PathBuf::from("/home/dikluwe/.cargo/git/checkouts/typst-assets-525e6d15ef7950cb/c0ae970/files/fonts"),"embedded"),(std::path::PathBuf::from("03_infra/fixtures/fonts"),"fixture")]{for e in std::fs::read_dir(dir).unwrap(){let p=e.unwrap().path();if p.is_file(){let bytes=std::fs::read(&p).unwrap();for index in 0..ttf_parser::fonts_in_collection(&bytes).unwrap_or(1){if ttf_parser::Face::parse(&bytes,index).is_ok(){inputs.push((p.clone(),index,kind));}}}}}
 for(path,index,kind)in inputs{let bytes=std::fs::read(&path).unwrap();let old=legacy(&bytes,index);let fast=selective::acquire(&path,index);let stats=instrumented::diagnostic_read(&path,index);let current=fast.clone().or_else(||legacy(&bytes,index));assert_eq!(current,old,"{} face {}",path.display(),index);if fast.is_some(){assert_eq!(instrumented::acquire(&path,index),fast);}
 rows.push(serde_json::json!({"path":path,"index":index,"kind":kind,"metadata_equal":true,"fast_accepted":fast.is_some(),"selectable":old.is_some(),"projected_bytes_and_requested":stats}));}
 if mode=="validate"{println!("{}",serde_json::json!({"all_equal":true,"faces":rows}));return;}
 let mut samples=Vec::new();for round in 0..3{for pair in 0..12{let mut previous=0;let mut current=0;for step in 0..2{let is_new=(step+round+pair)%2==0;let t=Instant::now();for(id,path,index)in &faces{black_box(if is_new{selective::acquire(path,*index).or_else(||db.with_face_data(*id,legacy).flatten())}else{db.with_face_data(*id,legacy).flatten()});}let elapsed=t.elapsed().as_nanos();if is_new{current=elapsed}else{previous=elapsed}}samples.push(serde_json::json!({"round":round+1,"pair":pair,"legacy_ns":previous,"current_ns":current}));}}
 println!("{}",serde_json::json!({"all_equal":true,"faces":rows,"samples":samples,"includes_acquisition_metadata_coverage":true,"excludes_discovery":true}));
}
