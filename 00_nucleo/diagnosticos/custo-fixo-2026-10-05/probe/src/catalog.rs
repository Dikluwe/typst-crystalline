//! Diagnostic composition of the current private fontdb/coverage path.
//! No production source modification or new production API.
use std::time::Instant;
use typst_core::entities::font_book::{Coverage,FontBook};
use typst_infra::fonts::{font_info_from_bytes,FontSlot};
fn ms(t:Instant)->f64{t.elapsed().as_secs_f64()*1000.0}
pub fn run(){
 let all=Instant::now();let t=Instant::now();let embedded=typst_infra::embedded_fonts::load_embedded_fonts();let embedded_ms=ms(t);
 let t=Instant::now();let mut db=fontdb::Database::new();db.load_system_fonts();let discovery_ms=ms(t);
 let t=Instant::now();let mut rows=vec![];let mut slots=vec![];let mut book=FontBook::new();
 for face in db.faces(){
  let (path,index)=match &face.source{fontdb::Source::File(p)|fontdb::Source::SharedFile(p,_) =>(p.clone(),face.index),fontdb::Source::Binary(_)=>continue};
  let outer=Instant::now();let mut metadata_ms=0.0;let mut parse_ms=0.0;let mut enumerate_ms=0.0;let mut compress_ms=0.0;let mut count=0;let mut subtables=0;let mut bytes=0;
  let info=db.with_face_data(face.id,|data,index|{
   bytes=data.len();let t=Instant::now();let mut info=font_info_from_bytes(data,index)?;metadata_ms=ms(t);
   let t=Instant::now();let parsed=ttf_parser::Face::parse(data,index).ok()?;parse_ms=ms(t);
   let t=Instant::now();let mut codepoints=vec![];
   if let Some(cmap)=parsed.tables().cmap{for subtable in cmap.subtables{if subtable.is_unicode(){subtables+=1;subtable.codepoints(&mut |c|codepoints.push(c));}}}
   count=codepoints.len();enumerate_ms=ms(t);let t=Instant::now();info.coverage=Coverage::from_codepoints(codepoints);compress_ms=ms(t);Some(info)
  }).flatten();
  let callback_ms=ms(outer);let accepted=info.is_some();
  rows.push(serde_json::json!({"path":path,"index":index,"bytes":bytes,"accepted":accepted,"callback_ms":callback_ms,"metadata_ms":metadata_ms,"second_parse_ms":parse_ms,"enumerate_ms":enumerate_ms,"compress_ms":compress_ms,"codepoints":count,"unicode_subtables":subtables}));
  if let Some(info)=info{slots.push(FontSlot::new(path,index));book.push(info);}
 }
 let processing_ms=ms(t);let t=Instant::now();drop(db);let database_drop_ms=ms(t);let total_ms=ms(all);
 // Untimed parity of the diagnostic composition with the actual public loader.
 let (real_slots,real_book)=typst_infra::fontdb::load_system_fonts();
 assert_eq!(slots.len(),real_slots.len());
 assert!(slots.iter().zip(&real_slots).all(|(a,b)|a.path==b.path&&a.index==b.index));
 assert!(book.infos().iter().zip(real_book.infos()).all(|(a,b)|a.family==b.family&&a.variant==b.variant&&a.flags==b.flags&&a.coverage==b.coverage));
 println!("{}",serde_json::json!({"total_ms":total_ms,"embedded_ms":embedded_ms,"embedded_entries":embedded.text_slots.len()+embedded.math_code_slots.len(),"discovery_ms":discovery_ms,"processing_ms":processing_ms,"database_drop_ms":database_drop_ms,"system_entries":slots.len(),"rows":rows,"metadata_and_coverage_match_real_loader":true,"diagnostic_composition_not_instrumented_production":true}));
}
