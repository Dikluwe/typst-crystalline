use std::{time::Instant,path::PathBuf};
use typst_infra::fonts::font_info_from_bytes;
fn main(){
 let start=Instant::now();let mut db=fontdb::Database::new();db.load_system_fonts();let discovery_ms=start.elapsed().as_secs_f64()*1000.;
 let faces:Vec<_>=db.faces().filter_map(|f|match &f.source{fontdb::Source::File(p)|fontdb::Source::SharedFile(p,_)=>Some((f.id,p.clone(),f.index)),_=>None}).collect();
 let mut samples=Vec::new();
 for iteration in 0..6 {
  for mode in if iteration%2==0{[0,1]}else{[1,0]} {
   let total=Instant::now();let mut inner_ns=0u128;let mut opens=0;let mut bytes_read=0u64;let mut last:Option<(PathBuf,Vec<u8>)>=None;let mut infos=Vec::new();
   for (id,path,index) in &faces {
    let info=if mode==0 { opens+=1;db.with_face_data(*id,|data,i|{let t=Instant::now();let info=font_info_from_bytes(data,i);inner_ns+=t.elapsed().as_nanos();info}).flatten() }
    else {
     if last.as_ref().map(|x|&x.0)!=Some(path){let bytes=std::fs::read(path).expect("readable catalog file");opens+=1;bytes_read+=bytes.len() as u64;last=Some((path.clone(),bytes));}
     let t=Instant::now();let info=font_info_from_bytes(&last.as_ref().unwrap().1,*index);inner_ns+=t.elapsed().as_nanos();info
    };infos.push(info);
   }
   let total_ms=total.elapsed().as_secs_f64()*1000.;
   // Equality after timer; metadata only, coverage intentionally empty in both.
   for ((id,_,_),info) in faces.iter().zip(&infos){let expected=db.with_face_data(*id,font_info_from_bytes).flatten();assert_eq!(info,&expected);}
   samples.push(serde_json::json!({"iteration":iteration,"mode":if mode==0{"fontdb_remap"}else{"safe_read_last_file"},"total_ms":total_ms,"metadata_ms":inner_ns as f64/1e6,"source_access_count":opens,"bytes_read":bytes_read,"exact_metadata_equal":true}));
  }
 }
 println!("{}",serde_json::json!({"discovery_ms":discovery_ms,"faces":faces.len(),"paths":faces.iter().map(|x|x.1.clone()).collect::<std::collections::BTreeSet<_>>(),"samples":samples}));
}
