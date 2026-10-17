from pathlib import Path
import datetime,gzip,json,hashlib,subprocess,sys
out=Path(__file__).resolve().parents[1];phase=sys.argv[1];b=json.load(gzip.open(out/'before.json.gz'));sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest();hashes={p:sha(p) for p in b['hashes']};cases={}
for name,body in {'content':'[A]','none':'none','string':'"A"','integer':'1','bool':'true','array':'(1,2)','table':'table([A])','wrapped-table':'block(table([A]))','styled-table':'text(fill:red,table([A]))','raw':'raw("A")','sequence-table':'[x #table([A])]','sequence-raw-table':'[#raw("x") #table([A])]'}.items():
 cases['figure-body-'+name]='figure('+body+')'
 for op in ['fields()','kind']:
  cases['figure-body-'+name+'-'+op]='figure('+body+').'+op
for name,kind in {'auto':'auto','none':'none','str':'"custom"','empty-str':'""','integer':'1','float':'2.0','bool':'true','array':'(1,2)','dict':'(a:1)','table':'table','raw':'raw','rect':'rect','function':'assert','closure':'x=>x','type':'int','content':'[A]'}.items():
 cases['figure-kind-'+name]='figure([A],kind:'+kind+')'
 cases['figure-kind-table-'+name]='figure(table([A]),kind:'+kind+').fields()'
for name,caption in {'none':'none','content':'[C]','string':'"C"','integer':'1','bool':'true','auto':'auto','caption':'figure.caption([C])'}.items():cases['figure-caption-'+name]='figure([A],caption:'+caption+')'
for name,value in {'length':'12pt','em':'1em','relative':'50%+3pt','ratio':'50%','integer':'12','float':'12.0','auto':'auto','none':'none','str':'"12pt"','bool':'true','fraction':'1fr','negative':'-3pt'}.items():
 for constructor,field in [('h','amount'),('v','amount'),('box','width'),('block','width')]:
  expr=f'{constructor}({value})' if constructor in ['h','v'] else f'{constructor}([{constructor}],width:{value})'
  cases[constructor+'-'+name]=expr+'.fields()'
cases.update({'figure-missing':'figure()','figure-missing-kind':'figure(kind:1)','figure-invalid-both':'figure(1,caption:1,kind:1)','figure-invalid-caption-kind':'figure([A],caption:1,kind:1)','figure-invalid-kind-caption':'figure([A],kind:1,caption:1)','figure-valid-kind-alias':'{let t=table;figure([A],kind:t)}','figure-invalid-with':'figure([A],kind:table.with(columns:1))'})
bins={'tekt':Path('target/debug/typst').resolve(),'vanilla':Path('lab/typst-original/target/release/typst').resolve()};assert sha(bins['vanilla'])=='7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea';r=dict(start=datetime.datetime.now(datetime.timezone.utc).isoformat(),head=b['head'],source_hashes=hashes,status=subprocess.check_output(['git','status','--porcelain=v1'],text=True),diffstat=subprocess.check_output(['git','diff','HEAD','--stat'],text=True),binaries={k:dict(path=str(p),sha256=sha(p)) for k,p in bins.items()},cases=[])
for name,expr in cases.items():
 for kind,bin in bins.items():
  cmd=[str(bin),'eval',expr,'--in',str(out/'empty.typ'),'--format','json'];z=subprocess.run(cmd,capture_output=True,text=True,timeout=30);assert z.returncode in [0,1];r['cases'].append(dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),name=name,expression=expr,binary=kind,argv=cmd,exit=z.returncode,stdout=z.stdout,stderr=z.stderr))
r.update(end=datetime.datetime.now(datetime.timezone.utc).isoformat(),sources_unchanged=all(sha(p)==h for p,h in hashes.items()));assert r['sources_unchanged']
with gzip.open(out/(phase+'-measurement.json.gz'),'wt') as f:json.dump(r,f)
print(len(cases),'pairs',len(r['cases']),'calls')
