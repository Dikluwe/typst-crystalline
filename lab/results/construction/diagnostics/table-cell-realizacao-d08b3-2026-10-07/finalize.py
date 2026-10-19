from pathlib import Path
import gzip,json,hashlib,datetime,subprocess,collections,re
R=Path.cwd();D=R/'00_nucleo/diagnosticos/table-cell-realizacao-d08b3-2026-10-07'
def read(p):return json.loads(gzip.decompress(p.read_bytes())) if p.suffix=='.gz' else json.loads(p.read_text())
def compare(p):
 d=read(p);bad=[];stderr_bad=[]
 for n,c in d['cases'].items():
  a,b=c['runs']['tekt'],c['runs']['vanilla'];keys=['status','stdout','pdf_text'];diff=[k for k in keys if a.get(k)!=b.get(k)]
  if diff:bad.append({'case':n,'different':diff})
  if a.get('stderr')!=b.get('stderr'):stderr_bad.append(n)
 return {'collection':str(p.relative_to(R)),'utc':d['utc'],'finished_utc':d.get('finished_utc'),'binaries':d.get('bin_sha256'),'total':len(d['cases']),'matching_status_stdout_pdf_text':len(d['cases'])-len(bad),'differences':bad,'stderr_differences':stderr_bad}
b=read(D/'approved-baseline.json.gz');a=read(D/'tested-tenth.json.gz');owners=read(D/'owners.json');allowed={p for pair in owners for p in pair}|{'00_nucleo/prompts/_nuclei/table/cell-realization.toml','01_core/src/compiler/eval/tests.rs','00_nucleo/prompts/compiler/eval/tests.md'}
changed=sorted(p for p in set(a['sha256'])|set(b['sha256']) if a['sha256'].get(p)!=b['sha256'].get(p));outside=[p for p in changed if p not in allowed];assert not outside,outside
res={'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'head':a['head'],'tree':'working tree não commitado','snapshot':'tested-tenth.json.gz','manifest_sha256':'5fec42835e38e9a28ee2ea2714b30135f069f111c140bb5e918a0902a06cc82b','vanilla_pin':'9f2b6e8715237cb086899a42873660fe744622e8','preservation':{'approved_owners':43,'supplemental_test_owner':['01_core/src/compiler/eval/tests.rs','00_nucleo/prompts/compiler/eval/tests.md'],'changed':changed,'outside_scope':outside},'collections':{}}
for label in ['after-tenth','refuters-tenth']:res['collections'][label]=compare(D/(label+'.json.gz'))
for name in ['table-cell-presenca-2026-10-07','table-cell-selector-2026-10-06','equation-where-2026-10-06','math-bare-2026-10-06','matrix-morfologia-2026-10-06','query-compilada-2026-10-06']:
 p=R/'00_nucleo/diagnosticos'/name/'verified-cell-realization-tenth.json.gz';res['collections'][name]=compare(p)
p=R/'00_nucleo/diagnosticos/matrix-morfologia-2026-10-06/verified-cell-realization-tenth-additional.json.gz';res['collections']['matrix-additional']=compare(p)
g=read(D/'geometry-tenth.json');res['geometry']={}
for name,c in g['cases'].items():
 a1,b1=c['runs']['tekt']['words'],c['runs']['vanilla']['words'];same=[v['text'] for v in a1]==[v['text'] for v in b1]
 res['geometry'][name]={'same_words_and_count':same,'max_bbox_delta_pt':max(abs(float(x['box'][k])-float(y['box'][k])) for x,y in zip(a1,b1) for k in ['xMin','yMin','xMax','yMax']) if same else None,'collection':'geometry-tenth.json'}
levels=collections.Counter(r.get('level','warning') for run in read(D/'lint-tenth.sarif')['runs'] for r in run['results']);res['lint']={'errors':levels['error'],'warnings':levels['warning'],'notes':levels['note'],'collection':'lint-tenth.sarif'}
res['tests']={'suites':re.findall(r'test result:.*', (D/'suites-tenth.log').read_text()),'query':re.findall(r'test result:.*',(D/'query-tenth.log').read_text()),'build':(D/'build-tenth.log').read_text().splitlines()[-1],'release':(D/'release-tenth.log').read_text().splitlines()[-1]}
res['binary_sha256']=hashlib.sha256((R/'target/release/typst').read_bytes()).hexdigest()
(D/'final-results.json').write_text(json.dumps(res,ensure_ascii=False,indent=2)+'\n');print(json.dumps({k:v for k,v in res.items() if k not in ['preservation']},ensure_ascii=False,indent=2))
