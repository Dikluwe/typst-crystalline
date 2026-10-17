from pathlib import Path
import json,gzip,shutil,datetime
out=Path('00_nucleo/diagnosticos/tekt-v16-introspect-carriers-2026-10-17');a=out/'located-fixture-attempt';a.mkdir(exist_ok=False)
for p in list(out.glob('validation-*.log'))+list(out.glob('*-resources.json'))+[out/'validation-result.json',out/'citation-control.rs',out/'before-citation-probe.json',out/'red.log',out/'red-result.json',out/'red-state.json.gz']:
 if p.exists():shutil.copy2(p,a/p.name)
for name in ['write_tests.py','expected.py','citation_probe.py']:shutil.copy2(out/'workers'/name,a/name)
(a/'notes.json').write_text(json.dumps(dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),issue='wrap used Locator::new().next() on every call. Two distinct Located refs shared the same identity; product correctly deduplicated them. Correct fixture threads one locator and reserves a disjoint prefix; no product identity change.',first_green='2 passed, 1 failed: only Located/reference position produced [k2]'),indent=2)+'\n')
# Update the L0 before fixture changes.
p=Path('00_nucleo/prompts/compiler/introspect/bibliography.md');s=p.read_text()+'''

### Refinamento I1 — identidades válidas na fixture (2026-10-17)

Medição: located-fixture-attempt conserva o source freeze e o primeiro GREEN:
11 posições coincidem e Located/refs produz só k2. A fixture reiniciava Locator
para cada wrapper, atribuindo a mesma identidade a referências distintas.
Isso refuta a fixture, não autoriza gerar identidades novas na conversão.
Corrigir somente os helpers de sonda/testes: um Locator por árvore, com prefixo
reservado para não coincidir com ocorrências frescas do walk. Source/recipe/
visual e assertivas ficam iguais. Repetir baseline com os algoritmos originais
extraídos de before.json.gz sobre tipos reais da rlib; adaptar só imports para
o executável de sonda. Observar Cite por walker estrito da fixture e, no GREEN,
confirmar também citation_order público. Distinguir essa reconstrução controlada
da medição pública inicial arquivada. Mesmo programa/hashes antes/depois;
enum/modelo, teste de label e código produtivo não mudam por este refinamento.
''';p.write_text(s)
# Same valid helper in public probe and repository tests.
def refine(s):
 s=s.replace('fn wrap(kind: usize, body: Content) -> Content {','fn wrap(kind: usize, body: Content, locator: &mut Locator) -> Content {').replace('fn wrap(kind:usize,body:Content)->Content{','fn wrap(kind:usize,body:Content,locator:&mut Locator)->Content{')
 s=s.replace('Locator::new().next()','locator.next()')
 s=s.replace('wrap(kind, refs(bib()))','wrap(kind, refs(bib()), &mut locator)').replace('wrap(kind, bib())','wrap(kind, bib(), &mut locator)').replace('wrap(kind, Content::reference("k2"))','wrap(kind, Content::reference("k2"), &mut locator)').replace('wrap(kind, Content::reference("k1"))','wrap(kind, Content::reference("k1"), &mut locator)').replace('wrap(kind, Content::reference("unknown"))','wrap(kind, Content::reference("unknown"), &mut locator)')
 s=s.replace('for kind in 0..4 {','for kind in 0..4 {\n            let mut locator = Locator::new();\n            for _ in 0..100 { locator.next(); }')
 return s
source=Path('01_core/src/compiler/introspect/bibliography.rs');s=source.read_text();assert 'mod carrier_regressions {' in s;head,test=s.split('mod carrier_regressions {',1);source.write_text(head+'mod carrier_regressions {'+refine(test))
# Build a corrected control program, using the final fixture helper verbatim.
s=source.read_text().split('mod carrier_regressions {',1)[1];shared=s[:s.index('    #[test]')].replace('crate::','typst_core::');b=json.load(gzip.open(out/'before.json.gz'));old=b['files']['01_core/src/compiler/introspect/bibliography.rs'];old=old[old.index('pub fn convert_bib_refs_to_cites'):].replace('crate::','typst_core::');legacy='mod baseline { use typst_core::entities::content::Content;\n'+old+'\n}\n';(out/'baseline-algorithms.rs').write_text(legacy)
main=r'''
fn keys(c:&Content,out:&mut Vec<String>){match c {Content::Cite(e)=>out.push(e.key.clone()),Content::Sequence(s)=>for c in s.iter(){keys(c,out)},Content::Source(e)=>keys(&e.body,out),Content::Located(e)=>keys(&e.body,out),Content::ShowRecipe(e)=>keys(&e.body,out),Content::Realized(e)=>keys(&e.visual,out),Content::Bibliography(_)|Content::Ref(_)|Content::Empty|Content::Text(_)=>{},_=>panic!("unsupported fixture node")}}
fn main(){
 let before=std::env::args().nth(1).unwrap()=="before";
 let convert=|c| if before{baseline::convert_bib_refs_to_cites(c)}else{typst_core::compiler::introspect::convert_bib_refs_to_cites(c)};
 let expected=vec!["k2".to_string(),"k1".to_string()];let mut bare=Vec::new();keys(&convert(refs(bib())),&mut bare);assert_eq!(bare,expected);let mut differences=0;
 for kind in 0..4 {for position in 0..3 {
  let mut locator=Locator::new();for _ in 0..100{locator.next();}
  let input=match position{0=>wrap(kind,refs(bib()),&mut locator),1=>refs(wrap(kind,bib(),&mut locator)),2=>Content::Sequence(vec![wrap(kind,Content::reference("k2"),&mut locator),wrap(kind,Content::reference("k1"),&mut locator),bib()].into()),_=>unreachable!()};let output=convert(input);let mut got=Vec::new();keys(&output,&mut got);if !before{assert_eq!(introspect_with_introspector(&output).citation_order(),got.as_slice());}if got!=expected{differences+=1;}println!("{kind}/{position}: {got:?}");
 }}
 let hidden=Content::Realized(Arc::new(RealizedContent{original:refs(bib()),visual:Content::Empty}));let mut hidden_keys=Vec::new();keys(&convert(hidden),&mut hidden_keys);assert!(hidden_keys.is_empty());println!("differences={differences}");
}
''';(out/'citation-control.rs').write_text(shared+'\n'+legacy+main)
p=out/'workers/citation_probe.py';s=p.read_text().replace('subprocess.run([str(binary)],','subprocess.run([str(binary),phase],');s=s.replace('compile_stderr=compile.stderr,','method="Original algorithms extracted verbatim from baseline (only crate-path imports adapted) for before; live library for after. Strict Cite observer on valid fixture; after also checks public citation_order. Initial public invalid-Located fixture archived.",baseline_algorithm_sha256=sha(out/\'baseline-algorithms.rs\'),compile_stderr=compile.stderr,');p.write_text(s)
# Extend the exact replay to include only this documented test-fixture refinement.
p=out/'workers/expected.py';s=p.read_text();needle='  result[source]=subprocess.run';assert needle in s;helper='''  if source.endswith('/bibliography.rs'):
   head,test=text.split('mod carrier_regressions {',1)
   test=test.replace('fn wrap(kind: usize, body: Content) -> Content {','fn wrap(kind: usize, body: Content, locator: &mut Locator) -> Content {').replace('Locator::new().next()','locator.next()')
   for old,new in [('wrap(kind, refs(bib()))','wrap(kind, refs(bib()), &mut locator)'),('wrap(kind, bib())','wrap(kind, bib(), &mut locator)'),('wrap(kind, Content::reference("k2"))','wrap(kind, Content::reference("k2"), &mut locator)'),('wrap(kind, Content::reference("k1"))','wrap(kind, Content::reference("k1"), &mut locator)'),('wrap(kind, Content::reference("unknown"))','wrap(kind, Content::reference("unknown"), &mut locator)')]:test=test.replace(old,new)
   test=test.replace('for kind in 0..4 {','for kind in 0..4 {\\n            let mut locator = Locator::new();\\n            for _ in 0..100 { locator.next(); }')
   text=head+'mod carrier_regressions {'+test
''';s=s.replace(needle,helper+needle);p.write_text(s)
print('Located fixture corrected; baseline algorithm reconstruction recorded')
