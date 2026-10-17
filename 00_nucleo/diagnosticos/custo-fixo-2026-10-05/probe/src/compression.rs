
use std::{hint::black_box,time::Instant};
use typst_core::entities::font_book::Coverage;
fn scalar(codepoints: Vec<u32>) -> Vec<u32> {
        let mut intervals: Vec<(u32,u32)> = Vec::new();
        let mut points=codepoints.into_iter();
        if let Some(mut start)=points.next(){
            let mut end=start;
            for point in points {
                if end.checked_add(1)==Some(point) {end=point;}
                else if (start..=end).contains(&point) {}
                else if point.checked_add(1)==Some(start) {start=point;}
                else {intervals.push((start,end));start=point;end=point;}
            }
            intervals.push((start,end));
        }
        intervals.sort_unstable_by_key(|&(start, _)| start);

        let mut runs: Vec<u32> = Vec::new();
        let mut next = 0u32;
        for (start, end) in intervals {
            let end = end + 1;
            if end <= next {
                continue;
            }
            if start <= next && !runs.is_empty() {
                *runs.last_mut().unwrap() += end - next;
            } else {
                runs.extend([start - next, end - start]);
            }
            next = end;
        }
        runs
    }


pub fn run(){
 let mut db=fontdb::Database::new();db.load_system_fonts();
 let vectors:Vec<_>=db.faces().filter_map(|face| db.with_face_data(face.id,|data,index|{
  let Ok(face)=ttf_parser::Face::parse(data,index) else{return None};let mut v=vec![];
  if let Some(cmap)=face.tables().cmap {for st in cmap.subtables{if st.is_unicode(){st.codepoints(&mut|c|v.push(c));}}}Some(v)
 }).flatten()).collect();
 let mut rows=vec![];
 for round in 0..9 {
  let mut old_ns=0;let mut new_ns=0;
  for v in &vectors{
   let input=v.clone();let input2=v.clone();
   let (old,new)=if round%2==0{
    let t=Instant::now();let old=black_box(Coverage::from_codepoints(black_box(input)));old_ns+=t.elapsed().as_nanos();
    let t=Instant::now();let new=black_box(scalar(black_box(input2)));new_ns+=t.elapsed().as_nanos();(old,new)
   }else{
    let t=Instant::now();let new=black_box(scalar(black_box(input2)));new_ns+=t.elapsed().as_nanos();
    let t=Instant::now();let old=black_box(Coverage::from_codepoints(black_box(input)));old_ns+=t.elapsed().as_nanos();(old,new)
   };
   assert_eq!(format!("{:?}",old),format!("Coverage({:?})",new));
  }
  rows.push(serde_json::json!({"round":round,"old_ms":old_ns as f64/1e6,"scalar_ms":new_ns as f64/1e6}));
 }
 println!("{}",serde_json::json!({"entries":vectors.len(),"codepoints":vectors.iter().map(|v|v.len()).sum::<usize>(),"cloning_and_result_drop_excluded":true,"canonical_equal_for_all_inputs":true,"rounds":rows}));
}
