from pathlib import Path
import json,gzip
out=Path(__file__).resolve().parents[1];b=json.load(gzip.open(out/'before.json.gz'));assert (out/'l0-record.json').exists();p=Path('01_core/src/compiler/introspect/bibliography.rs');assert p.read_text()==b['files'][str(p)];src=(out/'citation-control.rs').read_text();shared=src[:src.index('fn main()')].replace('typst_core::','crate::');tests=r'''
#[test]
fn introspection_carriers_citations_keep_order() {
 let expected=vec!["k2".to_string(),"k1".to_string()];assert_eq!(introspect_with_introspector(&refs(bib())).citation_order(),expected.as_slice());let mut differences=Vec::new();
 for kind in 0..4 {for position in 0..3 {
  let input=match position{0=>wrap(kind,refs(bib())),1=>refs(wrap(kind,bib())),2=>Content::Sequence(vec![wrap(kind,Content::reference("k2")),wrap(kind,Content::reference("k1")),bib()].into()),_=>unreachable!()};let intr=introspect_with_introspector(&input);if intr.citation_order()!=expected.as_slice(){differences.push(format!("{kind}/{position}: {:?}",intr.citation_order()));}
 }}
 assert!(differences.is_empty(),"{}",differences.join("\n"));
}
#[test]
fn introspection_carriers_original_and_unknown_are_not_citations() {
 let hidden=Content::Realized(Arc::new(RealizedContent{original:refs(bib()),visual:Content::Empty}));assert!(introspect_with_introspector(&hidden).citation_order().is_empty());
 for kind in 0..4 {
  let unknown=wrap(kind,Content::reference("unknown"));let output=super::convert_bib_refs_to_cites(unknown.clone());assert_eq!(output,unknown);assert!(introspect_with_introspector(&output).citation_order().is_empty());
  let before=wrap(kind,refs(bib()));let after=super::convert_bib_refs_to_cites(before.clone());match (&before,&after){(Content::Source(a),Content::Source(z))=>assert_eq!(a.span,z.span),(Content::Located(a),Content::Located(z))=>assert_eq!(a.location,z.location),(Content::Realized(a),Content::Realized(z))=>assert_eq!(a.original,z.original),(Content::ShowRecipe(a),Content::ShowRecipe(z))=>{assert_eq!(a.textual,z.textual);assert_eq!(a.context_id_floor,z.context_id_floor);assert_eq!(a.file,z.file);assert!(Arc::ptr_eq(&a.rules,&z.rules));assert_eq!(a.guards,z.guards);},_=>panic!("carrier shape changed")}
 }
}
''';p.write_text(p.read_text()+'\n#[cfg(test)]\nmod carrier_regressions {\n'+shared+tests+'\n}\n')
p=Path('01_core/src/compiler/introspect/labelled.rs');assert p.read_text()==b['files'][str(p)];p.write_text(p.read_text()+r'''
#[cfg(test)]
mod carrier_regressions {
 use super::*;
 use std::sync::Arc;
 use crate::entities::{counter_update::CounterUpdate,elements::equation::EquationElem,content::SourceContent,file_id::FileId,span::Span,style::Styles};
 #[test]
 fn introspection_carriers_source_keeps_label_projection_and_limits() {
  let mut intr=TagIntrospector::empty();let location=Locator::new().next();
  intr.counters.apply_hierarchical_at(CounterKey::Selector(Selector::Kind(ElementKind::Heading)),1,location);
  intr.counters.apply_at(CounterKey::Selector(Selector::Kind(ElementKind::Equation)),CounterUpdate::step(),location);
  intr.counters.apply_at(CounterKey::Str("figure:image".into()),CounterUpdate::step(),location);
  let span=Span::from_range(FileId::from_raw(std::num::NonZeroU16::new(7).unwrap()),0..1);let wrap=|body|Content::Source(Arc::new(SourceContent{body,span}));let lang:Lang="pt".parse().unwrap();
  let targets=[Content::heading(1,Content::text("H")),Content::Equation(Arc::new(EquationElem::new(Content::text("x"),true))),Content::figure(Content::text("F"),Some(Content::text("caption")),None,None)];let mut differences=Vec::new();
  for target in targets {
   let expected=compute_labelled(&intr,location,&target,Some(&lang));assert!(expected.0.is_some(),"direct target should resolve: {target:?}");let observed=compute_labelled(&intr,location,&wrap(target.clone()),Some(&lang));if observed!=expected{differences.push(format!("{target:?}: {expected:?} != {observed:?}"));}
  }
  assert!(differences.is_empty(),"{}",differences.join("\n"));
  for target in [Content::text("x"),Content::styled(Content::heading(1,Content::text("H")),Styles::new())] {assert_eq!(compute_labelled(&intr,location,&target,None),(None,None));assert_eq!(compute_labelled(&intr,location,&wrap(target),None),(None,None));}
 }
}
''');print('Three tests added before implementation')
