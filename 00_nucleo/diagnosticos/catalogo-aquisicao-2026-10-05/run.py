import datetime,gzip,json,hashlib,subprocess,os,statistics
from pathlib import Path
H=Path(__file__).resolve().parent;R=H.parents[2];sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest();git=lambda *a:subprocess.check_output(['git',*a],cwd=R,text=True)
files=git('diff','--name-only').splitlines()+git('ls-files','--others','--exclude-standard').splitlines();files=[n for n in files if n.endswith(('.rs','.md','.toml')) and not n.startswith('00_nucleo/diagnosticos/')]
d={'started':datetime.datetime.now(datetime.timezone.utc).isoformat(),'head':git('rev-parse','HEAD').strip(),'status':git('status','--short'),'diff_stat':git('diff','HEAD','--stat'),'source_hashes':{n:sha(R/n) for n in files},'sources':{n:(R/n).read_text() for n in files},'helpers':{str(p.relative_to(H)):p.read_text() for p in (H/'probe').rglob('*') if p.is_file()},'binary_sha256':sha(R/'target/release/catalog-acquisition-probe'),'cli_sha256':sha(R/'target/release/typst'),'runs':[]}
cpu=min(os.sched_getaffinity(0));d['cpu']=cpu
for i in range(3):
 p=subprocess.run([str(R/'target/release/catalog-acquisition-probe')],capture_output=True,text=True,check=True,preexec_fn=lambda:os.sched_setaffinity(0,{cpu}));m=json.loads(p.stdout);d['runs'].append({'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'measurement':m,'stderr':p.stderr});print(i,m['discovery_ms'],[(mode,statistics.median(x['total_ms'] for x in m['samples'] if x['mode']==mode),statistics.median(x['metadata_ms'] for x in m['samples'] if x['mode']==mode)) for mode in ['fontdb_remap','safe_read_last_file']],flush=True)
d['fonts']={p:sha(Path(p)) for p in d['runs'][0]['measurement']['paths']};d['finished']=datetime.datetime.now(datetime.timezone.utc).isoformat();assert all(sha(R/n)==s for n,s in d['source_hashes'].items())
with gzip.open(H/'measurement.json.gz','wt') as f:json.dump(d,f,ensure_ascii=False,indent=2)
