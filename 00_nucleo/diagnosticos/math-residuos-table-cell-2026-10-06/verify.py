from pathlib import Path
import json,gzip,hashlib,collections,re,subprocess,datetime
D=Path(__file__).parent
load=lambda p:json.loads(gzip.decompress(p.read_bytes()))
b=load(D/'baseline-state.json.gz');f=load(D/'final-code.json.gz');seals=json.loads((D/'seals.json').read_text());allowed=set(seals)|{v['owner'] for v in seals.values()};changed=[p for p,h in b['sha256'].items() if f['sha256'].get(p)!=h];new=set(f['sha256'])-set(b['sha256']);outside=(set(changed)|new)-allowed;assert not outside,outside
for p,v in seals.items():
 assert hashlib.sha256(Path(p).read_bytes()).hexdigest()==v['source_sha256']
 assert hashlib.sha256(Path(v['owner']).read_bytes()).hexdigest()==v['l0_sha256']
lint=json.loads((D/'lint-final.sarif').read_text());levels=collections.Counter(x.get('level','warning') for run in lint['runs'] for x in run['results']);assert levels['error']==0
result={'head':f['head'],'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'working_tree':'uncommitted; contents/diffstat/status/hashes in final-code.json.gz; exact final core-test state in tested-final-code.json.gz','preservation':{'changed':changed,'new':sorted(new),'outside':sorted(outside)},'lint':dict(levels),'suites':{},'measurements':{}}
for log in ['core-final.log','infra-final.log','cli-query-final.log']:
 text=(D/log).read_text();summary=re.findall(r'test result: ok\.[^\n]*',text);assert summary,log;result['suites'][log]=summary
for log in ['debug-final.log','release-final.log']:assert 'Finished ' in (D/log).read_text()
for name in ['math-residuos-table-cell-2026-10-06/final','math-residuos-table-cell-2026-10-06/local-refuters-final','matrix-morfologia-2026-10-06/final-residuos-locais','matrix-morfologia-2026-10-06/final-residuos-locais-additional','equation-where-2026-10-06/final-residuos-locais','query-compilada-2026-10-06/final-residuos-locais','math-bare-2026-10-06/final-residuos-locais']:
 p=Path('00_nucleo/diagnosticos')/(name+'.json.gz');d=load(p);good=[];bad=[]
 for n,c in d['cases'].items():
  a,v=c['runs']['tekt'],c['runs']['vanilla'];ok=all(a.get(k)==v.get(k) for k in ['status','stdout','stderr','pdf_text','html']);(good if ok else bad).append(n)
 result['measurements'][name]={'utc':d['utc'],'finished_utc':d['finished_utc'],'head':d['head'],'bin_sha256':d['bin_sha256'],'coincident':good,'different':bad,'sha256':hashlib.sha256(p.read_bytes()).hexdigest()}
assert subprocess.run(['git','diff','--check']).returncode==0
result['evidence_sha256']={str(p):hashlib.sha256(p.read_bytes()).hexdigest() for p in D.iterdir() if p.suffix in ('.gz','.log','.sarif')}
(D/'final-results.json').write_text(json.dumps(result,ensure_ascii=False,indent=2)+'\n')
print(result['suites']);print('lint',levels,'outside scope',outside)
for n,m in result['measurements'].items():print(n,len(m['coincident']),len(m['different']),m['different'])
