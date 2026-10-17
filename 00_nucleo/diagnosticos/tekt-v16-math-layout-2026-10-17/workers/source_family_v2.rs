use std::{num::NonZeroU16,sync::Arc};
use typst_core::{compiler::{math::layout::MathLayouter,layout::{FixedMetrics,FontMetrics}},entities::{content::{Content,SourceContent},elements::math_attach::MathAttachSlot,glyph_variants::{GlyphVariants,GlyphVariant,MathGlyphKern,MathKernTable,MathKernRecord},file_id::FileId,span::Span,layout_types::{TextStyle,Pt,Length,FrameItem},math_class::MathClass,rel::Rel}};
#[derive(Clone,Copy)]struct Metrics;
impl FontMetrics for Metrics{
 fn advance(&self,t:&str,z:Pt,s:&TextStyle)->Pt{FixedMetrics.advance(t,z,s)}
 fn vertical_metrics(&self,z:Pt,s:&TextStyle)->(Pt,Pt){FixedMetrics.vertical_metrics(z,s)}
 fn cap_height(&self,z:Pt,s:&TextStyle)->Pt{FixedMetrics.cap_height(z,s)}
 fn text_edges(&self,z:Pt,s:&TextStyle)->(Pt,Pt){FixedMetrics.text_edges(z,s)}
 fn char_italics_correction(&self,_:char,z:Pt,_:&TextStyle)->Pt{z*0.08}
 fn top_accent_attach(&self,c:char,z:Pt,_:&TextStyle)->Option<Pt>{Some(z*if c=='x'||c=='𝑥'{0.23}else{0.61})}
 fn math_kern(&self,_:char,_:&TextStyle)->MathGlyphKern{let t=MathKernTable{records:vec![MathKernRecord{correction_height:None,kern_value:75.0}]};MathGlyphKern{top_left:t.clone(),top_right:t.clone(),bottom_left:t.clone(),bottom_right:t}}
 fn horizontal_glyph_variants(&self,c:char,_:&TextStyle)->GlyphVariants{if c!='→'{return GlyphVariants::default()}GlyphVariants{variants:[600.0,1200.0,2400.0].into_iter().enumerate().map(|(i,w)|GlyphVariant{glyph_id:200+i as u16,advance:w,hor_advance:w,top_accent_attach:None}).collect()}}
}
fn src(body:Content,on:bool)->Content{if !on{return body}Content::Source(Arc::new(SourceContent{body,span:Span::from_range(FileId::from_raw(NonZeroU16::new(1).unwrap()),2..5)}))}
fn text(t:&str)->Content{Content::MathText(t.into())}
fn seq(v:Vec<Content>)->Content{Content::MathSequence(v.into())}
fn attach(base:Content,script:Content)->Content{Content::math_attach_scripts(base,MathAttachSlot::Omitted,MathAttachSlot::Omitted,MathAttachSlot::Omitted,MathAttachSlot::Present(script))}
fn visit(items:&[FrameItem],out:&mut Vec<String>){for item in items{match item{
 FrameItem::TextReplacement{text,items}=>{out.push(format!("actual:{text}"));visit(items,out)},
 FrameItem::Semantic{items,..}=>visit(items,out),
 FrameItem::Text{pos,text,style,..}|FrameItem::TextShaped{pos,text,style,..}=>out.push(format!("text:{text}@{:.6},{:.6}/{:.6}",pos.x.val(),pos.y.val(),style.size.val())),
 FrameItem::Glyph{pos,base_char,size,..}=>out.push(format!("glyph:{base_char}@{:.6},{:.6}/{:.6}",pos.x.val(),pos.y.val(),size.val())),
 other=>panic!("Unknown observation: {other:?}"),
}}}
fn main(){let style=TextStyle::regular(Pt(12.0));let l=MathLayouter::new(&Metrics,false,&style);
 for name in ["limits-base","ordinary-base","script","op-text","accent-base","accent-i","accent-nested","underover-base","underover-piece","matrix-cell","matrix-align-point","text-spacing","pipe-spacing","weak-space","weak-between","punctuation-tail","punctuation-source","punctuation-next","stretch-body","sequence-body","tighten-text","tighten-next","class-body"]{for on in [false,true]{let c=match name{
 "limits-base"=>attach(src(Content::math_limits_override(text("∑"),true,true,true),on),text("a")),
 "ordinary-base"=>attach(src(text("x"),on),text("a")),
 "script"=>attach(text("x"),src(text("a"),on)),
 "op-text"=>attach(Content::math_op(src(Content::text("lim"),on),false),text("a")),
 "accent-base"=>Content::math_accent(src(text("x"),on),'\u{0307}'),
 "accent-i"=>Content::math_accent(src(text("i"),on),'\u{0307}'),
 "accent-nested"=>Content::math_accent(src(Content::math_accent(text("x"),'\u{0307}'),on),'\u{0307}'),
 "underover-base"=>Content::math_underover(src(text("x"),on),None,Some(text("⏞"))),
 "underover-piece"=>Content::math_underover(text("x"),None,Some(src(text("⏞"),on))),
 "matrix-cell"=>Content::math_matrix(vec![vec![src(seq(vec![text("1"),Content::math_align_point(),text("2")]),on)],vec![seq(vec![text("333"),Content::math_align_point(),text("4")])]],('\0','\0')),
 "matrix-align-point"=>Content::math_matrix(vec![vec![seq(vec![text("1"),src(Content::math_align_point(),on),text("2")])],vec![seq(vec![text("333"),Content::math_align_point(),text("4")])]],('\0','\0')),
 "text-spacing"=>seq(vec![src(Content::text("hello"),on),text("x")]),
 "pipe-spacing"=>seq(vec![src(text("|"),on),text("x")]),
 "weak-space"=>seq(vec![src(Content::h_space(Length::em(0.2),true),on),text("x")]),
 "weak-between"=>seq(vec![text("a"),src(Content::h_space(Length::em(0.2),true),on),text("x")]),
 "punctuation-tail"=>seq(vec![text("; "),src(Content::h_space(Length::em(0.2),true),on)]),
 "punctuation-source"=>seq(vec![src(text("; "),on),Content::text("hello")]),
 "punctuation-next"=>seq(vec![text("; "),src(Content::text("hello"),on)]),
 "sequence-body"=>seq(vec![text("a"),src(seq(vec![text("="),text("b")]),on),text("c")]),
 "tighten-text"=>seq(vec![src(Content::text("hello"),on),Content::math_delimited('(',text("x"),')')]),
 "tighten-next"=>seq(vec![Content::text("hello"),src(Content::math_delimited('(',text("x"),')'),on)]),
 "class-body"=>Content::math_class_override(MathClass::Normal,src(text("f"),on)),
 "stretch-body"=>Content::math_stretch(src(text("→"),on),Rel{rel:1.5,abs:Length::ZERO},true),
 _=>unreachable!(),};let(items,e)=l.layout_equation_measured(&c,&style);let mut atoms=vec![];visit(&items,&mut atoms);println!("{name} source={on} width={:.6} ascent={:.6} descent={:.6} {atoms:?}",e.width,e.ascent,e.descent);}}
}
