from pathlib import Path
import json,gzip,re,subprocess
out=Path(__file__).resolve().parents[1]
def expected():
 b=json.load(gzip.open(out/'before.json.gz'));red=json.load(gzip.open(out/'red-state.json.gz'));scope=json.load(open(out/'scope.json'));plan=json.load(open(out/'projection-plan.json'));result={}
 for source in scope['sources']:
  text=red['files'][source];assert text.startswith(b['files'][source]);data=text.encode()
  for t in [t for t in plan['targets'] if t['source']==source]:
   a,z=t['pattern_start_byte'],t['pattern_end_byte'];assert data[a:z]==b'_';data=data[:a]+t['replacement'].encode()+data[z:]
  text=data.decode()
  if source.endswith('/bibliography.rs'):
   assert text.count('        match c {')==1;text=text.replace('        match c {','''        match c {
            Content::Source(e) => walk(&e.body, keys),
            Content::ShowRecipe(e) => walk(&e.body, keys),
            Content::Located(e) => walk(&e.body, keys),
            Content::Realized(e) => walk(&e.visual, keys),''',1)
   arms=''
   for variant,field in [('Source','body'),('ShowRecipe','body'),('Located','body'),('Realized','visual')]:arms+='''        Content::'''+variant+'''(e) => {
            let mut e = Arc::unwrap_or_clone(e);
            e.'''+field+''' = convert_refs(e.'''+field+''', keys);
            Content::'''+variant+'''(Arc::new(e))
        }
'''
   assert text.count('    match content {')==1;text=text.replace('    match content {','    match content {\n'+arms,1)
   old='_ => {} // neutro: N16[β] — variantes sem content estático navegável (terminais) não contribuem keys';assert text.count(old)==1;text=text.replace(old,'_ => {} // I2-bibliography-static-coverage: unhandled static containers remain visible debt.',1)
   # Test-only fixture imports, unused by the citation control, are removed.
   for path in ['elements::equation::EquationElem','style::Styles','value::Value']:
    text,n=re.subn(r'^\s*'+re.escape(path)+r',\n','',text,flags=re.M);assert n==1,(path,n)
  elif source.endswith('/labelled.rs'):
   needle='    match target {';assert text.count(needle)==1;text=text.replace(needle,needle+'''
        Content::Source(e) => compute_labelled(intr, location, &e.body, lang),''',1)
  if source.endswith('/bibliography.rs'):
   head,test=text.split('mod carrier_regressions {',1)
   test=test.replace('fn wrap(kind: usize, body: Content) -> Content {','fn wrap(kind: usize, body: Content, locator: &mut Locator) -> Content {').replace('Locator::new().next()','locator.next()')
   for old,new in [('wrap(kind, refs(bib()))','wrap(kind, refs(bib()), &mut locator)'),('wrap(kind, bib())','wrap(kind, bib(), &mut locator)'),('wrap(kind, Content::reference("k2"))','wrap(kind, Content::reference("k2"), &mut locator)'),('wrap(kind, Content::reference("k1"))','wrap(kind, Content::reference("k1"), &mut locator)'),('wrap(kind, Content::reference("unknown"))','wrap(kind, Content::reference("unknown"), &mut locator)')]:test=test.replace(old,new)
   test=test.replace('for kind in 0..4 {','for kind in 0..4 {\n            let mut locator = Locator::new();\n            for _ in 0..100 { locator.next(); }')
   text=head+'mod carrier_regressions {'+test
  result[source]=subprocess.run(['rustfmt','--edition','2021','--emit','stdout'],input=text,capture_output=True,text=True,check=True).stdout
 return result
