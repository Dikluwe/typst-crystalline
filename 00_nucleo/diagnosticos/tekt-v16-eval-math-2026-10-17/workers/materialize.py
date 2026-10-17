from pathlib import Path
import json,gzip,subprocess,re,datetime,hashlib
out=Path(__file__).resolve().parents[1];b=json.load(gzip.open(out/'before.json.gz'));scope=json.load(open(out/'scope.json'));baseline=json.load(open(out/'baseline-result.json'));t=json.load(open(out/'tests-before-code.json'));sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest();assert baseline['functional_red'] and baseline['structural_red'] and baseline['sources_unchanged'];assert all(sha(p)==h for p,h in baseline['source_hashes'].items());assert json.load(open(out/'probe-baseline-result.json'))['selected']==0
files={p:Path(p).read_text() for p in scope['sources']};source=scope['sources'][0];s=files[source].encode();assert files[source]==t['files'][source]['expected']
for r in sorted(scope['targets'],key=lambda x:x['baseline_pattern_start_byte'],reverse=True):
 a,z=r['baseline_pattern_start_byte'],r['baseline_pattern_end_byte'];assert s[a:z]==b'_',(r['id'],s[a:z]);s=s[:a]+r['replacement'].encode()+s[z:]
files[source]=s.decode();source=scope['sources'][1];s=files[source];assert s==t['files'][source]['expected'];old='fn table_root(c: &Content) -> bool {\n                match c {';new='fn table_root(c: &Content) -> bool {\n                match c.source_body() {';assert s.count(old)==1;files[source]=s.replace(old,new,1)
record={'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'files':{},'table_root_before':old,'table_root_after':new}
for p,s in files.items():
 s=re.sub(r'^//! @updated [0-9-]+$','//! @updated 2026-10-17',s,count=1,flags=re.M);Path(p).write_text(s);record['files'][p]={'unformatted':s}
subprocess.run(['rustfmt','--edition','2021','--config-path',str(Path('rustfmt.toml').resolve()),'--config','skip_children=true',*scope['sources']],check=True)
for p in files:record['files'][p]['expected_after']=Path(p).read_text()
(out/'materialization.json').write_text(json.dumps(record,ensure_ascii=False,indent=2)+'\n');print('70 nominal patterns and one source_body scrutinee applied',flush=True)
