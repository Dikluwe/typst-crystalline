from pathlib import Path
import json,gzip
out=Path(__file__).resolve().parents[1];b=json.load(gzip.open(out/'before.json.gz'));p=Path('01_core/src/entities/content/fields.rs');assert p.read_text()==b['files'][str(p)];assert (out/'l0-record.json').exists();fixture=(out/'field-control.rs').read_text();a=fixture.index('fn main()');shared=fixture[:a].replace('typst_core::','crate::');tests=r'''
#[test]
fn content_views_styled_equation_carriers_preserve_fields() {
 let eq=Content::Equation(Arc::new(EquationElem::new(Content::text("x"),true)));
 let expected=Content::styled(eq.clone(),styles());let fields=["numbering","supplement","alt","number-align","body","block","missing"];let mut differences=Vec::new();
 for kind in 0..4 {for position in 0..3 {
  let observed=match position{0=>Content::styled(wrap(kind,eq.clone()),styles()),1=>wrap(kind,expected.clone()),2=>Content::styled(wrap(kind,Content::styled(eq.clone(),Styles::new())),styles()),_=>unreachable!()};
  for field in fields {let a=expected.get_field(field);let z=observed.get_field(field);if a!=z{differences.push(format!("{kind}/{position}/{field}: {a:?} != {z:?}"));}}
 }}
 assert!(differences.is_empty(),"{}",differences.join("\n"));
}
#[test]
fn content_views_non_equations_and_style_precedence_stay_separate() {
 let eq=Content::Equation(Arc::new(EquationElem::new(Content::text("x"),true)));
 for kind in 0..4 {
  let non_equation=Content::styled(wrap(kind,Content::text("x")),styles());
  for key in ["alt","body","block","numbering"] {assert_eq!(non_equation.get_field(key),None,"{kind}/{key}");}
  let visual_text=Content::styled(Content::Realized(Arc::new(RealizedContent{original:eq.clone(),visual:Content::text("x")})),styles());assert_eq!(visual_text.get_field("alt"),None);
  let inner=Content::styled(wrap(kind,eq.clone()),Styles::new().push_custom("equation.alt",Value::Str("inner".into())));
  let outer=Content::styled(wrap(kind,inner.clone()),Styles::new().push_custom("equation.alt",Value::Str("outer".into())));assert_eq!(outer.get_field("alt"),Some(Value::Str("outer".into())),"{kind}/outer");
  let fallback=Content::styled(wrap(kind,inner),Styles::new());assert_eq!(fallback.get_field("alt"),Some(Value::Str("inner".into())),"{kind}/inner");
 }
}
''';p.write_text(p.read_text()+'\n#[cfg(test)]\nmod content_view_regressions {\n'+shared+tests+'\n}\n');print('Two functional regressions added before implementation')
