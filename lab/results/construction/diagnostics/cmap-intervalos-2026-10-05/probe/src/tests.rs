use super::*;
use std::{collections::BTreeSet,ops::Range};
fn put16(v:&mut Vec<u8>,n:u16){v.extend(n.to_be_bytes());}
fn put32(v:&mut Vec<u8>,n:u32){v.extend(n.to_be_bytes());}
fn format4(segments:&[(u16,u16)])->Vec<u8>{
 let mut v=vec![];for n in [4,(16+8*segments.len()) as u16,0,(2*segments.len()) as u16,0,0,0]{put16(&mut v,n);}
 for &(_,end) in segments{put16(&mut v,end);}put16(&mut v,0);
 for &(start,_) in segments{put16(&mut v,start);}
 for _ in 0..2*segments.len(){put16(&mut v,0);}v
}
fn grouped(format:u16,groups:&[(u32,u32,u32)])->Vec<u8>{
 let mut v=vec![];put16(&mut v,format);put16(&mut v,0);put32(&mut v,(16+12*groups.len()) as u32);put32(&mut v,0);put32(&mut v,groups.len() as u32);
 for &(start,end,glyph) in groups{for n in [start,end,glyph]{put32(&mut v,n);}}v
}
fn fixture(tables:&[(u16,u16,Vec<u8>)])->Vec<u8>{
 let mut cmap=vec![];put16(&mut cmap,0);put16(&mut cmap,tables.len() as u16);let mut offset=4+8*tables.len();
 for (platform,encoding,data) in tables{put16(&mut cmap,*platform);put16(&mut cmap,*encoding);put32(&mut cmap,offset as u32);offset+=data.len();}
 for (_,_,data) in tables{cmap.extend(data);}
 let mut font=include_bytes!("/repos/Antigravity/typst-crystalline/03_infra/fixtures/fonts/NotoSans-Regular.ttf").to_vec();
 let count=u16::from_be_bytes(font[4..6].try_into().unwrap()) as usize;
 let at=(0..count).map(|i|12+16*i).find(|&at|&font[at..at+4]==b"cmap").unwrap();
 while font.len()%4!=0{font.push(0);}
 let offset=font.len() as u32;let length=cmap.len() as u32;font.extend(cmap);font[at+8..at+12].copy_from_slice(&offset.to_be_bytes());font[at+12..at+16].copy_from_slice(&length.to_be_bytes());font
}
fn check(data:&[u8],expected:impl IntoIterator<Item=u32>){
 let face=ttf_parser::Face::parse(data,0).unwrap();let old=baseline(&face);let (input,_,_)=ranges::collect(&face).unwrap();let new=ranges::canonical(input);
 assert_eq!(format!("{:?}",old),format!("Coverage({:?})",new));
 assert_eq!(old.iter().collect::<BTreeSet<_>>(),expected.into_iter().collect());
}
#[test]
fn interval_union_matches_independent_set_including_empty_and_upper_boundary(){
 let inputs:Vec<Vec<Range<u32>>>=vec![vec![],vec![0..1,0x10ffff..0x110000],vec![10..20,3..12,5..5,9..7,20..25],vec![300..400,100..300,0..100],vec![5..8,0..2,2..5,5..8]];
 for v in inputs{
  let points:Vec<_>=v.iter().cloned().flatten().collect();
  assert_eq!(format!("Coverage({:?})",ranges::canonical(v)),format!("{:?}",Coverage::from_codepoints(points)));
 }
}
#[test]
fn format4_retains_declared_ranges_zero_glyph_and_sentinel_stop(){
 let data=fixture(&[(0,3,format4(&[(2,5),(8,6),(0xffff,0xffff),(100,105)]))]);check(&data,2..=5);
}
#[test]
fn formats12_and13_preserve_holes_overlap_and_unicode_max(){
 let data=fixture(&[(0,4,grouped(12,&[(2,5,0),(8,6,0),(0x10ffff,0x10ffff,0)])),(3,10,grouped(13,&[(4,8,0),(20,21,u32::MAX)]))]);
 check(&data,(2..=8).chain(20..=21).chain([0x10ffff]));
}
#[test]
fn nonunicode_is_ignored_and_format6_retains_callback_fallback(){
 let mut six=vec![];for n in [6,18,0,40,4,0,1,0,2]{put16(&mut six,n);}
 let data=fixture(&[(3,0,format4(&[(1,4),(0xffff,0xffff)])),(0,3,six)]);check(&data,40..44);
 let face=ttf_parser::Face::parse(&data,0).unwrap();let (_,native,points)=ranges::collect(&face).unwrap();assert_eq!((native,points),(0,4));
}
#[test]
fn first_invalid_record_keeps_the_pinned_iterator_termination(){
 let data=fixture(&[(0,0,vec![0x03,0xe7]),(0,3,format4(&[(1,5),(0xffff,0xffff)]))]);check(&data,[]);
}
#[test]
fn unrepresentable_exclusive_endpoint_requires_legacy_face_fallback(){
 let data=fixture(&[(0,4,grouped(12,&[(u32::MAX,u32::MAX,0)]))]);let face=ttf_parser::Face::parse(&data,0).unwrap();assert!(ranges::collect(&face).is_none());
}
#[test]
fn truncated_format4_is_not_promoted_to_a_native_range(){
 let mut four=format4(&[(1,5),(0xffff,0xffff)]);four.truncate(17);let data=fixture(&[(0,3,four)]);check(&data,[]);
}
