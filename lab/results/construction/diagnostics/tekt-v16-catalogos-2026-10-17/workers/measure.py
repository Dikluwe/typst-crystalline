from pathlib import Path
import datetime,gzip,json,hashlib,re,subprocess,sys
out=Path(__file__).resolve().parents[1];phase=sys.argv[1];b=json.load(gzip.open(out/'before.json.gz'));sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest();hashes={p:sha(p) for p in b['hashes']}
if phase=='before':assert hashes==b['hashes']
# The original catalogue is an inventory, not evidence that omitted fields should be absent upstream.
original=b['files'];types={'Direction':'type(ltr)','Alignment':'type(left)','Angle':'type(1deg)','Function':'type(assert)','Duration':'type(duration(seconds:1))','Length':'type(1pt)','Selector':'type(heading.where(level:1))','State':'type(state("catalogue",0))','Array':'type((1,2))','Dictionary':'type((a:1))','Str':'type("A")','Bytes':'type(bytes((1,2)))','Arguments':'type(((..a)=>a)(1,x:2))'};cases={};inventory=[]
for source,text in original.items():
 if not source.endswith('.rs'):continue
 for kind,field in re.findall(r'\(Type::(\w+), "([^"]+)"\)',text):
  key=kind+'.'+field
  if key not in [x['name'] for x in inventory]:inventory.append(dict(name=key,type=kind,field=field,source=source))
  if kind in types:cases['field-'+key]='repr(('+types[kind]+').'+field+')'
for kind,expr in types.items():cases['unknown-'+kind]='('+expr+').nonexistent'
for name,expr in {'none':'none','int':'1','ratio':'20%','content':'[A]','color':'red','type':'int'}.items():cases['unsupported-'+name]='('+expr+').nonexistent()'
cases.update({'direction-constant':'type(ltr).ltr == ltr','alignment-constant':'type(left).left == left','length-unbound':'type(1pt).pt(3pt)','angle-unbound':'type(1deg).deg(90deg)','function-with':'{let t=type(assert); let f=t.with(assert,true);f();true}','array-static-map':'type((1,2)).map((1,2),x=>x*2)','array-method-map':'(1,2).map(x=>x*2)','array-static-invalid':'type((1,2)).at((1,2),"bad")','array-method-invalid':'(1,2).at("bad")','array-unknown-method':'(1,2).nonexistent()','dict-keys':'(a:1,b:2).keys()','dict-static-keys':'type((a:1)).keys((a:1,b:2))','str-normalize':'"a\u0301".normalize()','str-static-normalize':'type("A").normalize("a\u0301")','str-missing':'"A".at(9)','bytes-at':'bytes((1,2)).at(1)','bytes-static-at':'type(bytes((1,2))).at(bytes((1,2)),1)','bytes-unknown':'bytes((1,2)).first()','args-pos':'((..a)=>a.pos())(1,x:2,3)','args-static-len':'{let a=((..a)=>a)(1,x:2);type(a).len(a)}','args-filter':'((..a)=>a.filter((k,v)=>v>1).named())(1,x:2,y:3)','collection-alias':'{let t=type((1,2));let f=t.len;f((1,2))}','named-self':'type((1,2)).len(self:(1,2))','wrong-self':'type((1,2)).len("A")','missing-self':'type((1,2)).len()'})
bins={'tekt':Path('target/debug/typst').resolve(),'vanilla':Path('lab/typst-original/target/release/typst').resolve()};assert sha(bins['vanilla'])=='7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea';r=dict(start=datetime.datetime.now(datetime.timezone.utc).isoformat(),head=b['head'],source_hashes=hashes,status=subprocess.check_output(['git','status','--porcelain=v1'],text=True),diffstat=subprocess.check_output(['git','diff','HEAD','--stat'],text=True),inventory=inventory,excluded_runtime_fields=[x for x in inventory if x['type']=='Location'],binaries={k:dict(path=str(p),sha256=sha(p)) for k,p in bins.items()},cases=[])
for name,expr in cases.items():
 for kind,bin in bins.items():
  cmd=[str(bin),'eval',expr,'--in',str(out/'empty.typ'),'--format','json'];z=subprocess.run(cmd,capture_output=True,text=True,timeout=30);assert z.returncode in [0,1];r['cases'].append(dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),name=name,expression=expr,binary=kind,argv=cmd,exit=z.returncode,stdout=z.stdout,stderr=z.stderr));print(name,kind,z.returncode,z.stdout.strip(),flush=True)
r.update(end=datetime.datetime.now(datetime.timezone.utc).isoformat(),sources_unchanged=all(sha(p)==h for p,h in hashes.items()));assert r['sources_unchanged']
with gzip.open(out/(phase+'-measurement.json.gz'),'wt') as f:json.dump(r,f)
