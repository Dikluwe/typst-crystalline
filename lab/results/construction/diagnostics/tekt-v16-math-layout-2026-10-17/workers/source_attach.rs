use std::{num::NonZeroU16,sync::Arc};
use typst_core::{compiler::{math::layout::MathLayouter,layout::FixedMetrics},entities::{content::{Content,SourceContent},elements::{math_attach::{MathAttachElem,MathAttachSlot},math_limits_override::MathLimitsOverrideElem},file_id::FileId,span::Span,layout_types::{TextStyle,Pt}}};
fn source(body:Content)->Content{Content::Source(Arc::new(SourceContent{body,span:Span::from_range(FileId::from_raw(NonZeroU16::new(1).unwrap()),2..5)}))}
fn main(){
 let base=Content::MathLimitsOverride(Arc::new(MathLimitsOverrideElem{body:Content::MathText("∑".into()),limits:true,inline:true,inline_explicit:true}));
 let style=TextStyle::regular(Pt(12.0));let l=MathLayouter::new(&FixedMetrics,false,&style);
 for (name,base) in [("plain",base.clone()),("source",source(base))]{let body=Content::MathAttach(Arc::new(MathAttachElem{base,t:MathAttachSlot::Omitted,b:MathAttachSlot::Omitted,tl:MathAttachSlot::Omitted,bl:MathAttachSlot::Omitted,tr:MathAttachSlot::Present(Content::MathText("a".into())),br:MathAttachSlot::Omitted}));let (_,e)=l.layout_equation_measured(&body,&style);println!("{name} width={} ascent={} descent={}",e.width,e.ascent,e.descent);}
}
