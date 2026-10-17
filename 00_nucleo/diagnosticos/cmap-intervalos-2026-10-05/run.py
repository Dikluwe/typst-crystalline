"""Three independent release processes; no production changes."""
import datetime,gzip,hashlib,json,os,statistics,subprocess
from pathlib import Path
H=Path(__file__).resolve().parent;R=H.parents[2];exe=R/'target/release/cmap-ranges-probe';sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest();git=lambda *a:subprocess.check_output(['git',*a],cwd=R,text=True)
cpu=min(os.sched_getaffinity(0));files=[R/n for n in git('diff','--name-only').splitlines() if n.endswith(('.rs','.md','.toml'))]+[R/n for n in git('ls-files','--others','--exclude-standard').splitlines() if n.startswith(('01_core/','00_nucleo/prompts/'))]
r={'started':datetime.datetime.now(datetime.timezone.utc).isoformat(),'head':git('rev-parse','HEAD').strip(),'status':git('status','--short'),'diff_stat':git('diff','HEAD','--stat'),'source_hashes':{str(p.relative_to(R)):sha(p) for p in files},'binary_sha256':sha(exe),'cpu':cpu,'helpers':{str(p.relative_to(H)):p.read_text() for p in (H/'probe').rglob('*') if p.is_file()},'runs':[]}
for i in range(3):
 cmd=[str(exe)];p=subprocess.run(cmd,text=True,capture_output=True,check=True,timeout=60,env=dict(os.environ,RAYON_NUM_THREADS='1',SOURCE_DATE_EPOCH='0'),preexec_fn=lambda:os.sched_setaffinity(0,{cpu}));d=json.loads(p.stdout);r['runs'].append({'index':i,'command':cmd,'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'stderr':p.stderr,'measurement':d});print(i,len(d['rows']),d['baseline_total_ms']/7,d['ranges_total_ms']/7,'intervals',d['input_intervals'],'native',d['native_subtables'],'fallback',d['fallback_points'],'legacy',d['legacy_fallback_count'],flush=True)
r['fonts']={p:sha(Path(p)) for p in sorted({x['path'] for x in r['runs'][0]['measurement']['rows']})}
assert all(sha(R/n)==s for n,s in r['source_hashes'].items())
r['finished']=datetime.datetime.now(datetime.timezone.utc).isoformat()
with gzip.open(H/'measurement.json.gz','wt') as f:json.dump(r,f,ensure_ascii=False,indent=2)
