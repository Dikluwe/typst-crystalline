use std::sync::Arc;
use typst_core::entities::{content::{Content,SourceContent,LocatedContent,RealizedContent,ShowRecipeContent},elements::equation::EquationElem,style::Styles,style_chain::StyleChain,value::Value,span::Span,file_id::FileId,locator::Locator};
fn wrap(kind:usize,body:Content)->Content{
 match kind {
  0=>Content::Source(Arc::new(SourceContent{body,span:Span::from_range(FileId::from_raw(std::num::NonZeroU16::new(7).unwrap()),0..1)})),
  1=>Content::Located(Arc::new(LocatedContent{body,location:Locator::new().next()})),
  2=>Content::Realized(Arc::new(RealizedContent{original:Content::text("causal original"),visual:body})),
  3=>Content::ShowRecipe(Arc::new(ShowRecipeContent{body,textual:false,context_id_floor:0,rules:Arc::from([]),styles:StyleChain::default_chain(),guards:vec![],file:FileId::from_raw(std::num::NonZeroU16::new(7).unwrap())})),
  _=>panic!("probe kind"),
 }
}

use typst_core::entities::{bib_entry::BibEntry,introspector::Introspector};
use typst_core::compiler::introspect::introspect_with_introspector;
fn bib()->Content {Content::bibliography(vec![BibEntry::new("k1","A1","T1",2024),BibEntry::new("k2","A2","T2",2023)],None)}
fn refs(b:Content)->Content {Content::Sequence(vec![Content::reference("k2"),Content::reference("k1"),b].into())}
fn main(){
 let expected=vec!["k2".to_string(),"k1".to_string()];assert_eq!(introspect_with_introspector(&refs(bib())).citation_order(),expected.as_slice());let mut differences=0;
 for k in 0..4 {for position in 0..3 {
  let input=match position{0=>wrap(k,refs(bib())),1=>refs(wrap(k,bib())),2=>Content::Sequence(vec![wrap(k,Content::reference("k2")),wrap(k,Content::reference("k1")),bib()].into()),_=>unreachable!()};
  let intr=introspect_with_introspector(&input);let got=intr.citation_order();if got!=expected.as_slice(){differences+=1;}println!("{k}/{position}: {got:?}");
 }}
 let hidden=Content::Realized(Arc::new(RealizedContent{original:refs(bib()),visual:Content::Empty}));assert!(introspect_with_introspector(&hidden).citation_order().is_empty());println!("differences={differences}");
}
