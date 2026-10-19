from pathlib import Path
import json,statistics as st,datetime,hashlib
D=Path(__file__).parent;M=D/'memory'
def pct(xs,q):
 s=sorted(xs);x=(len(s)-1)*q;i=int(x);return s[i]+(s[min(i+1,len(s)-1)]-s[i])*(x-i)
def stats(xs):return {'n':len(xs),'median':st.median(xs),'mean':st.mean(xs),'p05':pct(xs,.05),'p95':pct(xs,.95)}
index=json.loads((M/'index.json').read_text());blocks=[json.loads((M/p).read_text()) for p in index['blocks']];assert len(blocks)==21 and all(b['valid'] for b in blocks)
out={}
for name in sorted({r['fixture'] for b in blocks for r in b['samples']}):
 rows=[r for b in blocks for r in b['samples'] if r['fixture']==name];assert len(rows)==54;engines={e:stats([r['metrics']['maxrss_kib']/1024 for r in rows if r['engine']==e]) for e in ['previous','current','vanilla']};by={(r['round'],r['pair'],r['engine']):r for r in rows}
 out[name]={'maxrss_mib':engines,'paired_delta_current_previous_mib':stats([(by[(rnd,pair,'current')]['metrics']['maxrss_kib']-by[(rnd,pair,'previous')]['metrics']['maxrss_kib'])/1024 for rnd in [1,2,3] for pair in range(6)])}
summary=json.loads((D/'summary.json').read_text());summary['memory']=out
sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest()
for directory in [D,M]:
 a=json.loads((directory/'state/start.json').read_text());b=json.loads((directory/'state/end.json').read_text());assert a['sha256']==b['sha256'];assert a['head']==b['head']
assert all(sha(p)==summary['identity']['bin_sha256'][k] for k,p in summary['identity']['binaries'].items())
prod=['01_core/','02_shell/','03_infra/','04_wiring/','00_nucleo/prompts/']
current=json.loads((D/'state/end.json').read_text())
R=D.resolve().parents[2]
for p,digest in current['files'].items():
 if p.startswith(tuple(prod)):assert sha(R/p)==digest,p
summary['verification']={'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'wall_blocks':21,'wall_measured_invocations':756,'wall_warmups':126,'memory_blocks':21,'memory_measured_invocations':378,'memory_warmups':126,'stable_source_and_binaries':True,'competing_workloads_observed':False,'memory_method':'GNU time target RSS, separate population; wait4 fork-launcher RSS discarded'}
(D/'summary.json').write_text(json.dumps(summary,indent=2,ensure_ascii=False)+'\n')
for f,r in out.items():print(f,{k:round(v['median'],2) for k,v in r['maxrss_mib'].items()},'paired change MiB',round(r['paired_delta_current_previous_mib']['median'],3))
