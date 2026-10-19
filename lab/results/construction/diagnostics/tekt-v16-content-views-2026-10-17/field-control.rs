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
fn styles()->Styles {Styles::new().push_custom("equation.numbering",Value::Str("(1)".into())).push_custom("equation.supplement",Value::Str("Eq".into())).push_custom("equation.alt",Value::Str("alternative".into())).push_custom("equation.number-align",Value::None)}
fn main(){
 let eq=Content::Equation(Arc::new(EquationElem::new(Content::text("x"),true)));
 let expected=Content::styled(eq.clone(),styles());let fields=["numbering","supplement","alt","number-align","body","block","missing"];
 let mut differences=0;
 for k in 0..4 {for position in 0..3 {
  let observed=match position{0=>Content::styled(wrap(k,eq.clone()),styles()),1=>wrap(k,expected.clone()),2=>Content::styled(wrap(k,Content::styled(eq.clone(),Styles::new())),styles()),_=>unreachable!()};
  for field in fields {let a=expected.get_field(field);let z=observed.get_field(field);let same=a==z;if !same{differences+=1;}println!("{k}/{position}/{field}: {same} expected={a:?} actual={z:?}");}
 }}
 println!("differences={differences}");
}
