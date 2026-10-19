//! Untimed language-demand control through the existing shaper API.
use std::{fs,path::PathBuf};
use typst_core::{contracts::world::World,entities::{content::Content,font_list::FontList,layout_types::{FrameItem,Page,PagedDocument,Point,Pt,TextStyle}}};
use typst_infra::{fonts::FontSlot,world::SystemWorld,shaper::shape_document};
fn main(){
 let path=PathBuf::from("/usr/share/fonts/opentype/noto/NotoSansCJK-Regular.ttc");
 let w=SystemWorld::for_eval(std::env::temp_dir()).with_fonts(vec![FontSlot::new(path.clone(),0)]);
 let font=w.font(0).unwrap();let family=w.book().infos()[0].family.clone();let mut rows=vec![];
 for text in ["骨","直","令","漢","葛","辻"] {
  for lang in ["ja","zh","ko"] {
   let mut buffer=rustybuzz::UnicodeBuffer::new();buffer.push_str(text);buffer.set_direction(rustybuzz::Direction::LeftToRight);buffer.set_language(lang.parse().unwrap());buffer.guess_segment_properties();
   let face=rustybuzz::Face::from_slice(font.as_slice(),0).unwrap();let direct=rustybuzz::shape(&face,&[],buffer);let ids:Vec<_>=direct.glyph_infos().iter().map(|g|g.glyph_id).collect();
   let doc=PagedDocument::new(vec![Page{width:100.0,height:100.0,logical_number:1,numbering:None,supplement:Content::Empty,bleed:Default::default(),fill:Default::default(),background:vec![],foreground:vec![],items:vec![FrameItem::Text{pos:Point::ZERO,text:text.into(),style:TextStyle{size:Pt(12.0),font:Some(FontList::single(family.clone().into())),lang:Some(lang.parse().unwrap()),..Default::default()}}]}]);
   let shaped=shape_document(&w,doc);let glyphs=match &shaped.pages[0].items[0] {FrameItem::TextShaped{glyphs,..}=>format!("{glyphs:?}"),x=>panic!("{x:?}")};
   rows.push(serde_json::json!({"text":text,"language":lang,"direct_language_configured_ids":ids,"tekt_glyphs":glyphs}));
  }
 }
 println!("{}",serde_json::to_string_pretty(&serde_json::json!({"font_path":path,"font_file_bytes":fs::metadata(path).unwrap().len(),"family":family,"rows":rows,"not_a_vanilla_pipeline_oracle":true})).unwrap());
}
