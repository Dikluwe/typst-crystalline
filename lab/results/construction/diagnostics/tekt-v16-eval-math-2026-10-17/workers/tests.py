from pathlib import Path
import json,gzip,hashlib,datetime
out=Path(__file__).resolve().parents[1];b=json.load(gzip.open(out/'before.json.gz'));scope=json.load(open(out/'scope.json'));assert json.load(open(out/'l0-before-code.json'))['productive_sources_unchanged'];record={'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'files':{}}
p=Path(scope['sources'][0]);s=p.read_text();assert s==b['files'][str(p)];insertions=[]
needle='''            "$ x $ deve preservar grapheme como MathIdent: {grapheme:?}"
        );''';assert s.count(needle)==1;pos=s.index(needle)+len(needle);text='''
        assert!(leaf(&number, "x").is_none(), "guard rejeita número diferente");
        assert!(leaf(&grapheme, "2").is_none(), "guard rejeita grapheme diferente");''';insertions.append((pos,text))
needle='''    /// **P1132** — `binom` é morfologicamente uma fracção sem barra,''';assert s.count(needle)==1;pos=s.index(needle);text='''    #[test]
    fn v16_eval_math_unlined_fraction_skips_lined_before_binom() {
        let lined = extract_math_content(&MockWorld::new("$ frac(a, b) $"));
        assert!(find_unlined_frac_in(&lined).is_none());
        let mixed = extract_math_content(&MockWorld::new("$ frac(a, b) + binom(n, k) $"));
        let (num, den) = find_unlined_frac_in(&mixed).expect("binom posterior sem barra");
        assert_eq!(num.plain_text(), "n");
        assert_eq!(den.plain_text(), "k");
    }

''';insertions.append((pos,text))
for pos,text in sorted(insertions,reverse=True):s=s[:pos]+text+s[pos:]
p.write_text(s);record['files'][str(p)]={'expected':s,'insertions':[{'char':pos,'text':text,'byte':len(b['files'][str(p)][:pos].encode())} for pos,text in insertions]}
for r in scope['targets']:
 for key in ['arm_start_byte','arm_end_byte','pattern_start_byte','pattern_end_byte']:
  r['baseline_'+key]=r[key]+sum(len(t.encode()) for pos,t in insertions if len(b['files'][str(p)][:pos].encode())<=r[key])
 scope['test_insertions']=record['files'][str(p)]['insertions']
p=Path(scope['sources'][1]);assert p.read_text()==b['files'][str(p)]
module='''

#[cfg(test)]
mod v16_eval_math_table_root_tests {
    use super::*;
    use crate::entities::{content::{LocatedContent, RealizedContent, ShowRecipeContent, SourceContent}, file_id::FileId, span::Span};

    fn source(body: Content) -> Content {
        Content::Source(Arc::new(SourceContent {
            body,
            span: Span::from_range(FileId::from_raw(std::num::NonZeroU16::new(1).unwrap()), 2..5),
        }))
    }
    fn realized(original: Content, visual: Content) -> Content {
        Content::Realized(Arc::new(RealizedContent { original, visual }))
    }
    fn table() -> Content {
        Content::table(vec![], vec![], vec![Content::heading(1, Content::text("hidden"))])
    }
    fn observed(content: &Content) -> (Vec<String>, usize) {
        let intr = crate::compiler::introspect::introspect(content);
        let headings = intr.query(&Selector::Kind(ElementKind::Heading)).into_iter()
            .map(|loc| intr.elements[&loc].content().plain_text()).collect();
        (headings, intr.query(&Selector::Kind(ElementKind::Table)).len())
    }
    #[test]
    fn v16_eval_math_raw_table_origin_remains_shallow() {
        assert_eq!(observed(&realized(table(), Content::text("replacement"))), (vec![], 1));
    }
    #[test]
    fn v16_eval_math_source_table_carriers_hide_original_heading() {
        let loc = Location::from_raw(13);
        let located = |body| Content::Located(Arc::new(LocatedContent { location: loc, body }));
        let recipe = Content::ShowRecipe(Arc::new(ShowRecipeContent {
            body: source(table()), textual: false, context_id_floor: 0,
            rules: Arc::from([]), styles: StyleChain::default_chain(), guards: vec![],
            file: FileId::from_raw(std::num::NonZeroU16::new(1).unwrap()),
        }));
        let cases = [("one", source(table())), ("nested", source(source(table()))),
            ("located-source", located(source(table()))), ("source-located", source(located(table()))),
            ("recipe-source", recipe)];
        let actual: Vec<_> = cases.into_iter().map(|(name, original)|
            (name, observed(&realized(original, Content::text("replacement"))))).collect();
        let expected: Vec<_> = ["one", "nested", "located-source", "source-located", "recipe-source"]
            .into_iter().map(|name| (name, (Vec::<String>::new(), 1))).collect();
        assert_eq!(actual, expected);
    }
    #[test]
    fn v16_eval_math_source_table_keeps_visual_heading_and_location() {
        let loc = Location::from_raw(13);
        let original = Content::Located(Arc::new(LocatedContent { location: loc, body: source(table()) }));
        let doc = realized(original, Content::heading(1, Content::text("shown")));
        assert_eq!(observed(&doc), (vec!["shown".to_string()], 1));
        let intr = crate::compiler::introspect::introspect(&doc);
        assert_eq!(intr.query(&Selector::Kind(ElementKind::Table)), vec![loc]);
    }
    #[test]
    fn v16_eval_math_non_table_source_retains_origin_and_visual() {
        let doc = realized(source(Content::heading(1, Content::text("original"))), Content::heading(1, Content::text("shown")));
        assert_eq!(observed(&doc), (vec!["original".to_string(), "shown".to_string()], 0));
    }
}
'''
p.write_text(p.read_text()+module);record['files'][str(p)]={'expected':p.read_text(),'module':module};(out/'tests-before-code.json').write_text(json.dumps(record,ensure_ascii=False,indent=2)+'\n');(out/'scope.json').write_text(json.dumps(scope,ensure_ascii=False,indent=2)+'\n');print('Added guard controls and four Source(Table) regressions before implementation')
