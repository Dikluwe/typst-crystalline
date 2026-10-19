use std::{num::NonZeroU16,sync::Arc};
use typst_core::{compiler::introspect::introspect,entities::{content::{Content,RealizedContent,SourceContent},element_kind::ElementKind,file_id::FileId,introspector::Introspector,selector::Selector,span::Span}};
fn main(){
 let table=Content::table(vec![],vec![],vec![Content::heading(1,Content::text("hidden"))]);
 for (label,original) in [("plain",table.clone()),("source",Content::Source(Arc::new(SourceContent{body:table,span:Span::from_range(FileId::from_raw(NonZeroU16::new(1).unwrap()),2..5)}))) ] {
  let realized=Content::Realized(Arc::new(RealizedContent{original,visual:Content::text("replacement")}));
  let intr=introspect(&realized);let result=intr.query(&Selector::Kind(ElementKind::Heading));println!("{label} {}",result.len());
 }
}
