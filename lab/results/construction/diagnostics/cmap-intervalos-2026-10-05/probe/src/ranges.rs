//! Experimental checked range adapter. Not production code.
use std::ops::Range;
use ttf_parser::{cmap::Format,Face,Tag};
fn u16_at(data:&[u8],offset:usize)->Option<u16>{Some(u16::from_be_bytes(data.get(offset..offset.checked_add(2)?)?.try_into().ok()?))}
fn u32_at(data:&[u8],offset:usize)->Option<u32>{Some(u32::from_be_bytes(data.get(offset..offset.checked_add(4)?)?.try_into().ok()?))}
fn native(data:&[u8],format:u16,out:&mut Vec<Range<u32>>)->Option<()> {
 match format {
  4=>{
   let count=u16_at(data,6)? as usize/2;
   for i in 0..count {
    let start=u16_at(data,16+2*count+2*i)? as u32;let end=u16_at(data,14+2*i)? as u32;
    if start==0xffff&&end==0xffff{break;}
    if start<=end{out.push(start..end+1);}
   }
  }
  12|13=>{
   let count=u32_at(data,12)? as usize;
   for i in 0..count{
    let at=16usize.checked_add(i.checked_mul(12)?)?;let start=u32_at(data,at)?;let end=u32_at(data,at+4)?;
    if start<=end{out.push(start..end.checked_add(1)?);}
   }
  }
  _=>return None,
 }
 Some(())
}
pub fn collect(face:&Face)->Option<(Vec<Range<u32>>,usize,usize)> {
 let mut out=vec![];let mut native_count=0;let mut fallback_points=0;
 let Some(cmap)=face.tables().cmap else{return Some((out,0,0));};
 let raw=face.raw_face().table(Tag::from_bytes(b"cmap"));
 for i in 0..cmap.subtables.len(){
  // The pinned iterator stops at its first unparseable record; retain that boundary.
  let Some(st)=cmap.subtables.get(i) else{break;};
  if !st.is_unicode(){continue;}
  let format=match st.format{Format::SegmentMappingToDeltaValues(_)=>Some(4),Format::SegmentedCoverage(_)=>Some(12),Format::ManyToOneRangeMappings(_)=>Some(13),_=>None};
  let checkpoint=out.len();
  let ok=format.and_then(|format|raw.and_then(|data|{
   let offset=u32_at(data,4+8*i as usize+4)? as usize;let sub=data.get(offset..)?;
   if u16_at(sub,0)?!=format{return None;}
   native(sub,format,&mut out)
  })).is_some();
  if ok{native_count+=1;continue;}
  out.truncate(checkpoint);
  let mut active:Option<Range<u32>>=None;let mut overflow=false;
  st.codepoints(&mut |c: u32|{
   fallback_points+=1;let Some(end)=c.checked_add(1) else{overflow=true;return;};
   if let Some(r)=active.as_mut(){
    if c==r.end{r.end=end;return;}
    if r.contains(&c){return;}
    if end==r.start{r.start=c;return;}
   }
   if let Some(r)=active.replace(c..end){out.push(r);}
  });
  if overflow{return None;}
  if let Some(r)=active{out.push(r);}
 }
 Some((out,native_count,fallback_points))
}
pub fn canonical(ranges:Vec<Range<u32>>)->Vec<u32>{
 let mut compact:Vec<Range<u32>>=vec![];
 let mut input=ranges.into_iter().filter(|r|r.start<r.end);
 if let Some(first)=input.next(){
  let mut start=first.start;let mut end=first.end;
  for range in input{
   if range.start<=end&&start<=range.end{start=start.min(range.start);end=end.max(range.end);}
   else{compact.push(start..end);start=range.start;end=range.end;}
  }
  compact.push(start..end);
 }
 compact.sort_unstable_by_key(|r|r.start);
 let mut runs:Vec<u32>=vec![];let mut next=0;
 for r in compact{
  if r.end<=next{continue;}
  if r.start<=next&&!runs.is_empty(){*runs.last_mut().unwrap()+=r.end-next;}
  else{runs.extend([r.start-next,r.end-r.start]);}
  next=r.end;
 }
 runs
}
