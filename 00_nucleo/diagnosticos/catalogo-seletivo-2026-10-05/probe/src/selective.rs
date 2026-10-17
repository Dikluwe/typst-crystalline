//! Experimental safe table reads; returned bytes are never usable as a final font.
use std::io::{Read,Seek,SeekFrom};
fn u16at(b:&[u8],o:usize)->Option<u16>{Some(u16::from_be_bytes(b.get(o..o.checked_add(2)?)?.try_into().ok()?))}
fn u32at(b:&[u8],o:usize)->Option<u32>{Some(u32::from_be_bytes(b.get(o..o.checked_add(4)?)?.try_into().ok()?))}
fn get<R:Read+Seek>(r:&mut R,offset:u64,len:usize,size:u64,read:&mut u64)->Option<Vec<u8>>{
 if offset.checked_add(len as u64)?>size{return None}let mut b=vec![0;len];r.seek(SeekFrom::Start(offset)).ok()?;r.read_exact(&mut b).ok()?;*read+=len as u64;Some(b)
}
pub fn read<R:Read+Seek>(r:&mut R,size:u64,index:u32,cmap:bool)->Option<(Vec<u8>,u64)>{
 let mut count=0;let first=get(r,0,12,size,&mut count)?;
 let offset=if &first[..4]==b"ttcf"{let n=u32at(&first,8)?;if index>=n||12u64.checked_add(n as u64*4)?>size{return None}let b=get(r,12+index as u64*4,4,size,&mut count)?;let o=u32at(&b,0)? as u64;if o<12+n as u64*4{return None}o}else{if index!=0{return None}0};
 let header=if offset==0{first}else{get(r,offset,12,size,&mut count)?};
 if !matches!(&header[..4],b"\0\x01\0\0"|b"OTTO"|b"true"){return None}
 let n=u16at(&header,4)? as usize;let dir=get(r,offset+12,n.checked_mul(16)?,size,&mut count)?;
 let mut out=header;out.extend_from_slice(&dir);
 for i in 0..n{let at=12+i*16;let tag=&dir[i*16..i*16+4];let len=u32at(&dir,i*16+12)?;let off=u32at(&dir,i*16+8)?;
  let wanted=matches!(tag,b"head"|b"hhea"|b"maxp"|b"name"|b"OS/2"|b"post")||(cmap&&tag==b"cmap");
  if wanted {
   let mut take=len as usize;
   if tag==b"post" && (off as u64).checked_add(len as u64)?<=size {
    let prefix=get(r,off as u64,take.min(34),size,&mut count)?;
    take=if u32at(&prefix,0)==Some(0x00020000){u16at(&prefix,32).map(|n|34+n as usize*2).unwrap_or(take).min(take)}else{take.min(32)};
   }
   if let Some(b)=get(r,off as u64,take,size,&mut count){let new=u32::try_from(out.len()).ok()?;out[at+8..at+12].copy_from_slice(&new.to_be_bytes());out[at+12..at+16].copy_from_slice(&(b.len() as u32).to_be_bytes());out.extend_from_slice(&b);continue;}
   // Invalid optional table stays out of bounds, preserving parser skipping.
   out[at+8..at+12].copy_from_slice(&u32::MAX.to_be_bytes());
  }else{out[at+8..at+16].fill(0);}
 }
 Some((out,count))
}
