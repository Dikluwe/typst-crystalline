from pathlib import Path
import json,subprocess,datetime
out=Path(__file__).resolve().parents[1];r=json.load(open(out/'recipe-refinement.json'));p=Path(r['source']);s=p.read_text();assert s==r['source_before_tests'];assert Path(r['owner']).read_text()==r['expected_l0'];changes=[]
def replace(old,new):
 global s
 assert s.count(old)==1,(old[:50],s.count(old));s=s.replace(old,new,1);changes.append({'old':old,'new':new})
old='''        let recipe = Content::ShowRecipe(Arc::new(ShowRecipeContent {
            body: source(table()),
            textual: false,
            context_id_floor: 0,
            rules: Arc::from([]),
            styles: StyleChain::default_chain(),
            guards: vec![],
            file: FileId::from_raw(std::num::NonZeroU16::new(1).unwrap()),
        }));''';replace(old,'        let wrapped_recipe = recipe(source(table()));')
replace('    fn observed(content: &Content) -> (Vec<String>, usize) {','''    fn recipe(body: Content) -> Content {
        Content::ShowRecipe(Arc::new(ShowRecipeContent {
            body, textual: false, context_id_floor: 0, rules: Arc::from([]),
            styles: StyleChain::default_chain(), guards: vec![],
            file: FileId::from_raw(std::num::NonZeroU16::new(1).unwrap()),
        }))
    }
    fn observed(content: &Content) -> (Vec<String>, usize) {''')
replace('            ("recipe-source", recipe),','            ("recipe-source", wrapped_recipe),\n            ("recipe-plain", recipe(table())),');replace('"source-located", "recipe-source"]','"source-located", "recipe-source", "recipe-plain"]')
newtest='''
    #[test]
    fn v16_eval_math_show_recipe_preserves_location_and_shallow_context() {
        let loc = Location::from_raw(13);
        let original = Content::Located(Arc::new(LocatedContent {
            location: loc, body: recipe(source(table())),
        }));
        let doc = realized(original, Content::heading(1, Content::text("shown")));
        let intr = crate::compiler::introspect::introspect(&doc);
        assert_eq!((observed(&doc), intr.query(&Selector::Kind(ElementKind::Table))),
            ((vec!["shown".to_string()], 1), vec![loc]));
    }
''';pos=s.rfind('\n}');s=s[:pos]+newtest+s[pos:];p.write_text(s);subprocess.run(['rustfmt','--edition','2021','--config-path',str(Path('rustfmt.toml').resolve()),'--config','skip_children=true',str(p)],check=True);record={'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'source':str(p),'before':r['source_before_tests'],'changes':changes,'new_test':newtest,'expected_with_tests':p.read_text()};(out/'recipe-tests.json').write_text(json.dumps(record,ensure_ascii=False,indent=2)+'\n');print('Added plain recipe and explicit-location controls before repairing delegation')
