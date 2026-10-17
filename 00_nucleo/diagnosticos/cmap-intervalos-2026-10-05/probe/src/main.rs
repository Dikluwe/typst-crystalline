//! Independent coverage experiment; productive source is untouched.
mod ranges;
use std::{hint::black_box,time::Instant};
use typst_core::entities::font_book::Coverage;
fn baseline(face:&ttf_parser::Face)->Coverage{
 let mut points=vec![];
 if let Some(cmap)=face.tables().cmap{for st in cmap.subtables{if st.is_unicode(){st.codepoints(&mut|c|points.push(c));}}}
 Coverage::from_codepoints(points)
}
fn main(){
 assert!(!cfg!(debug_assertions));
 let mut db=fontdb::Database::new();db.load_system_fonts();let mut old_ns=0;let mut new_ns=0;let mut interval_count=0;let mut native_count=0;let mut fallback_count=0;let mut legacy_count=0;let mut rows=vec![];
 for face in db.faces(){
  let path=match &face.source{fontdb::Source::File(p)|fontdb::Source::SharedFile(p,_)=>p.clone(),fontdb::Source::Binary(_)=>continue};
  let row=db.with_face_data(face.id,|data,index|{
   let parsed=ttf_parser::Face::parse(data,index).unwrap();let mut samples=vec![];
   for i in 0..7{
    let (a,b,old,new)=if i%2==0{
     let t=Instant::now();let old=black_box(baseline(black_box(&parsed)));let a=t.elapsed().as_nanos();
     let t=Instant::now();let next=ranges::collect(black_box(&parsed));let new=next.map(|(v,n,f)|{if i==0{interval_count+=v.len();native_count+=n;fallback_count+=f;}black_box(ranges::canonical(v))});let b=t.elapsed().as_nanos();(a,b,old,new)
    }else{
     let t=Instant::now();let new=ranges::collect(black_box(&parsed)).map(|(v,_,_)|black_box(ranges::canonical(v)));let b=t.elapsed().as_nanos();
     let t=Instant::now();let old=black_box(baseline(black_box(&parsed)));let a=t.elapsed().as_nanos();(a,b,old,new)
    };
    if let Some(new)=new{assert_eq!(format!("{:?}",old),format!("Coverage({:?})",new));}else{legacy_count+=1;}
    old_ns+=a;new_ns+=b;samples.push(serde_json::json!({"index":i,"baseline_ns":a,"range_ns":b}));
   }
   serde_json::json!({"path":path,"index":index,"samples":samples})
  }).unwrap();rows.push(row);
 }
 println!("{}",serde_json::json!({"rows":rows,"input_intervals":interval_count,"native_subtables":native_count,"fallback_points":fallback_count,"legacy_fallback_count":legacy_count,"all_canonical_equal":true,"baseline_total_ms":old_ns as f64/1e6,"ranges_total_ms":new_ns as f64/1e6,"face_parse_and_output_drop_excluded":true}));
}

#[cfg(test)]
mod tests;
