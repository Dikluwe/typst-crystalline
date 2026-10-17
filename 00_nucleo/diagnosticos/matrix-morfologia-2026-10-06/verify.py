from pathlib import Path
import json,gzip,hashlib,re,datetime,subprocess,collections
D=Path(__file__).parent;R=Path.cwd()
def load(p):return json.loads(gzip.decompress(p.read_bytes()))
def compare(p):
 d=load(p);same=[];different=[]
 for name,c in d['cases'].items():
  a,b=c['runs']['tekt'],c['runs']['vanilla'];ok=True
  for k in ('status','stdout','stderr','pdf_text','html'):
   if a.get(k)==b.get(k):continue
   if k=='stdout':
    try:
     if json.loads(a[k])==json.loads(b[k]):continue
    except (ValueError,KeyError):pass
   ok=False
  (same if ok else different).append(name)
 return {'path':str(p),'utc':d['utc'],'finished_utc':d.get('finished_utc'),'head':d['head'],'bin_sha256':d['bin_sha256'],'coincident':same,'different':different}
seals=json.loads((D/'seals.json').read_text());allowed=set(seals)|{s['owner'] for s in seals.values()}|{'00_nucleo/prompts/_nuclei/math/matrix-morphology.toml'}
baseline=load(D/'baseline-state.json.gz');final=load(D/'final-code.json.gz');changed=[p for p,h in baseline['sha256'].items() if p in final['sha256'] and h!=final['sha256'][p]];new=sorted(set(final['sha256'])-set(baseline['sha256']));outside=sorted((set(changed)|set(new))-allowed);assert not outside,outside
for p,s in seals.items():
 assert hashlib.sha256(Path(p).read_bytes()).hexdigest()==s['source_sha256']
 assert hashlib.sha256(Path(s['owner']).read_bytes()).hexdigest()==s['l0_sha256']
lint=json.loads((D/'lint-final.sarif').read_text());levels=collections.Counter(x.get('level','warning') for run in lint['runs'] for x in run['results']);assert levels['error']==0
results={'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'head':final['head'],'working_tree':'uncommitted; exact diffstat/status/source hashes in final-code.json.gz','preservation':{'changed':changed,'new':new,'outside_scope':outside},'lint':dict(levels),'suites':{},'measurements':{}}
for log in ['core-final.log','shell.log','infra.log','cli-query.log']:
 s=(D/log).read_text();m=re.findall(r'test result: ok\.[^\n]*',s);assert m,log;results['suites'][log]=m
for log in ['debug-final.log','release-final.log']:
 s=(D/log).read_text();assert 'Finished ' in s,log;results['suites'][log]='Finished'
for key,p in [('matrix',D/'final-after-stroke.json.gz'),('matrix-additional',D/'final-after-stroke-additional.json.gz'),('D05',Path('00_nucleo/diagnosticos/equation-where-2026-10-06/final-matrix.json.gz')),('query',Path('00_nucleo/diagnosticos/query-compilada-2026-10-06/final-matrix.json.gz')),('D06',Path('00_nucleo/diagnosticos/math-bare-2026-10-06/final-matrix.json.gz'))]:results['measurements'][key]=compare(p)
assert subprocess.run(['git','diff','--check']).returncode==0
results['evidence_sha256']={str(p):hashlib.sha256(p.read_bytes()).hexdigest() for p in D.iterdir() if p.suffix in ('.gz','.log','.sarif')}
(D/'final-results.json').write_text(json.dumps(results,ensure_ascii=False,indent=2)+'\n');print(json.dumps({k:(len(v['coincident']),len(v['different']),v['different']) for k,v in results['measurements'].items()},ensure_ascii=False));print('lint',dict(levels),'preservation',outside)
