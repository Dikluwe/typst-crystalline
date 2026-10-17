#![allow(deprecated)]
use typst_infra::{world::SystemWorld,fonts::FontSlot,shaper::shape_document};
use typst_core::{contracts::world::World,entities::{layout_types::*,font_list::FontList}};
fn main(){
 let world=SystemWorld::new("/tmp/tekt-bidi-materializacao","emph-mixed.typ").unwrap().with_fonts(vec![
 FontSlot::new("/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf".into(),0),
 FontSlot::new("/usr/share/fonts/truetype/dejavu/DejaVuSans-Oblique.ttf".into(),0)]);
 let style=TextStyle{font:Some(FontList::single("DejaVu Sans".into())),italic:true,semantic_emph:true,size:Pt(11.),..Default::default()};
 let page=Page{width:145.,height:600.,logical_number:1,numbering:None,supplement:typst_core::entities::content::Content::Empty,bleed:Default::default(),fill:Default::default(),background:vec![],foreground:vec![],items:vec![FrameItem::Text{pos:Point{x:Pt(20.),y:Pt(30.)},text:"مرحبا".into(),style,bidi:None}]};
 let doc=shape_document(&world,PagedDocument::new(vec![page]));
 for item in &doc.pages[0].items{if let FrameItem::TextShaped{glyphs,style,text,..}=item{
 println!("emitted {:?} italic={} semantic_emph={} gids={:?}",text,style.italic,style.semantic_emph,glyphs.iter().map(|g|g.glyph_id).collect::<Vec<_>>());
 let variant=typst_infra::font_variant::text_style_to_font_variant(style);
 let slot=world.book().select_pattern(&style.font.as_ref().unwrap().as_slice()[0].name,&variant).unwrap();
 println!("export selects slot {} {:?}",slot,world.book().infos()[slot].variant);
 for i in 0..world.book().len(){let font=world.font(i).unwrap();let face=rustybuzz::Face::from_slice(font.as_slice(),0).unwrap();let mut buf=rustybuzz::UnicodeBuffer::new();buf.push_str(text);buf.set_direction(rustybuzz::Direction::RightToLeft);let result=rustybuzz::shape(&face,&[],buf);println!("face {} {:?} covers_arabic={} gids={:?}",i,world.book().infos()[i].variant,face.glyph_index('م').is_some(),result.glyph_infos().iter().map(|g|g.glyph_id).collect::<Vec<_>>());}
 }}
}
