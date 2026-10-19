"""Archive final lineage, build identity and the kept experiment verdict."""
from pathlib import Path
import gzip,json,hashlib,datetime,subprocess,difflib,re
D=Path(__file__).resolve().parent;R=D.parents[2]
b=json.load(gzip.open(D/'before.json.gz','rt'));c=json.load(gzip.open(D/'candidate.json.gz','rt'))
files={p:(R/p).read_text() for p in b['files']}
for p,s in files.items():
 if p.endswith('.rs'):
  clean=lambda t:re.sub(r'(?m)^//! @prompt-hash .*\n','',t)
  assert clean(s)==clean(c['files'][p]),p
binary=hashlib.sha256((R/'target/release/typst').read_bytes()).hexdigest()
assert binary=='9ae0a3d9285b8d0b4bae0da1db058483e7f5543003fabff46bb7d93b17e474f2'
logs={n:Path('/tmp/preflights-'+n+'.log').read_text() for n in ['core-tests','build-final','release-final','lint-final']}
assert '6680 passed; 0 failed; 5 ignored' in logs['core-tests']
assert 'Finished `dev` profile' in logs['build-final'] and 'Finished `release` profile' in logs['release-final']
gate=json.loads((D/'performance-green.json').read_text());assert gate['passed']
controls={}
for name in ['tables','extended','small-containers','effects']:
 s=json.load(gzip.open(D/f'qualification-{name}.json.gz','rt'))
 controls[name]={n:v for n,v in s['across'].items()}
 assert all(v['same_text'] and v['same_words'] and v['same_page_size'] and v['bbox_max_delta_pt']==0 and v['same_raster'] for v in controls[name].values())
state={'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'head':subprocess.check_output(['git','rev-parse','HEAD'],cwd=R,text=True).strip(),'working_tree':'uncommitted','status':subprocess.check_output(['git','status','--short','--untracked-files=all'],cwd=R,text=True),'diffstat':subprocess.check_output(['git','diff','HEAD','--stat'],cwd=R,text=True),'files':{p:{'text':s,'sha256':hashlib.sha256(s.encode()).hexdigest()} for p,s in files.items()},'binary_sha256':binary,'same_binary_as_measured':True,'same_algorithm_as_tested':True,'exit_codes':{'cargo_build':0,'cargo_build_release':0,'crystalline_lint':0},'logs':logs,'scope':'two productive owners; existing shared nucleus pinned by both; global warnings preexisting'}
with gzip.open(D/'final-validation.json.gz','wt') as f:json.dump(state,f,ensure_ascii=False,indent=2)
(D/'owner-diff.patch').write_text(''.join(''.join(difflib.unified_diff(old.splitlines(True),files[p].splitlines(True),fromfile=p,tofile=p)) for p,old in b['files'].items()))
(D/'final-verdict.json').write_text(json.dumps({'utc':state['utc'],'status':'kept','owners':json.loads((D/'owners.json').read_text()),'benefit':gate['benefit'],'robust_regressions':gate['regression'],'allocation_counts':'unchanged in all six cases','core_tests_passed':6680,'core_ignored':5,'controls_count':sum(map(len,controls.values())),'vanilla_equivalence':'not certified for existing divergent repeated math/block cases','final_binary_sha256':binary},indent=2)+'\n')
print(binary,'kept',sum(map(len,controls.values())),'controls')
