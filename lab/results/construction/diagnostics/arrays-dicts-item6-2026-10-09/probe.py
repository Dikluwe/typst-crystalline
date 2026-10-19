"""Compile allocator probes against the actual release rlibs and archive counts."""
import datetime, hashlib, json, subprocess
from pathlib import Path
D=Path(__file__).resolve().parent;R=D.parents[2]
def sha(p):return hashlib.sha256(Path(p).read_bytes()).hexdigest()
def now():return datetime.datetime.now(datetime.timezone.utc).isoformat()
def one(glob):
 p=list((R/glob[0]).glob(glob[1]));assert len(p)==1,p;return p[0]
core=R/'target/release/libtypst_core.rlib'
ecow=one(('target/release/deps','libecow-*.rlib'))
indexmap=one(('target/release/deps','libindexmap-*.rlib'))
hasher=one(('target/release/deps','librustc_hash-*.rlib'))
vanilla=R/'lab/typst-original/target/release/deps/libtypst_library-7b44c344382fe0f9.rlib'
inputs=[core,ecow,indexmap,hasher,vanilla,D/'tekt-probe.rs',D/'vanilla-probe.rs']
commands={
 'tekt':['rustc','--edition=2021','-O',str(D/'tekt-probe.rs'),'--extern',f'typst_core={core}','--extern',f'ecow={ecow}','--extern',f'indexmap={indexmap}','--extern',f'rustc_hash={hasher}','-L',f'dependency={R/"target/release/deps"}','-o','/tmp/item6-tekt-probe'],
 'vanilla':['rustc','--edition=2021','-O',str(D/'vanilla-probe.rs'),'--extern',f'typst_library={vanilla}','-L',f'dependency={R/"lab/typst-original/target/release/deps"}','-o','/tmp/item6-vanilla-probe']}
stat=subprocess.check_output(['git','diff','HEAD','--stat'],cwd=R)
status=subprocess.check_output(['git','status','--short','--untracked-files=all'],cwd=R)
(D/'probe-git-diff-head-stat.txt').write_bytes(stat)
(D/'probe-git-status-short.txt').write_bytes(status)
state={'utc_start':now(),'head':subprocess.check_output(['git','rev-parse','HEAD'],cwd=R,text=True).strip(),
 'working_tree':'uncommitted','git_diff_stat_sha256':hashlib.sha256(stat).hexdigest(),
 'git_status_sha256':hashlib.sha256(status).hexdigest(),
 'rustc_version':subprocess.check_output(['rustc','--version'],text=True).strip(),
 'inputs':{str(p.relative_to(R)):sha(p) for p in inputs},'commands':commands,'runs':{}}
for name,cmd in commands.items():
 p=subprocess.run(cmd,cwd=R,capture_output=True,text=True);assert p.returncode==0,p.stderr
 binary=Path(cmd[-1]);rows=[]
 for i in range(3):
  output=subprocess.check_output([str(binary)],cwd=R,text=True)
  rows.append({'utc':now(),'stdout':output})
 assert rows[0]['stdout']==rows[1]['stdout']==rows[2]['stdout']
 state['runs'][name]={'binary_sha256':sha(binary),'repetitions':rows}
state['utc_end']=now()
assert state['inputs']=={str(p.relative_to(R)):sha(p) for p in inputs}
(D/'probe-results.json').write_text(json.dumps(state,ensure_ascii=False,indent=2)+'\n')
print('saved',state['utc_end'])
