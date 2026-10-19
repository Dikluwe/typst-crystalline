from pathlib import Path
import subprocess,os,json,gzip,datetime,hashlib,base64
root=Path.cwd();out=root/'00_nucleo/diagnosticos/transformacoes-2026-10-14/shape-flow/snapshots'
sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest();now=lambda:datetime.datetime.now(datetime.timezone.utc).isoformat()
prior=json.load(gzip.open(out/'before.json.gz'));files={str(p):sha(p) for top in ['01_core','02_shell','03_infra','04_wiring','00_nucleo/prompts'] for p in Path(top).rglob('*') if p.is_file() and p.suffix in ['.rs','.md','.toml']}
results=[];env=dict(os.environ,CARGO_BUILD_JOBS='2',UPDATE_P307B_SNAPSHOTS='1')
for number in [1,2]:
 cmd=['cargo','test','--offline','--locked','--workspace','--lib','--','p307b_04_shapes','p307b_07_multi_feature','--test-threads=1']
 start=now()
 with (out/f'generation-{number}.log').open('w') as f:r=subprocess.run(cmd,stdout=f,stderr=subprocess.STDOUT,env=env)
 hashes={str(p):sha(p) for p in Path('03_infra/fixtures/p307b/reference').glob('*.pdf')}
 results.append(dict(start=start,end=now(),command=cmd,environment={k:env[k] for k in ['CARGO_BUILD_JOBS','UPDATE_P307B_SNAPSHOTS']},exit=r.returncode,hashes=hashes));(out/'generations.json').write_text(json.dumps(results,indent=2));assert r.returncode==0
 if number==1:
  for stem in ['04-shapes','07-multi-feature']:(out/(stem+'-after.pdf')).write_bytes(Path('03_infra/fixtures/p307b/reference',stem+'.pdf').read_bytes())
assert results[0]['hashes']==results[1]['hashes']
allowed={'03_infra/fixtures/p307b/reference/04-shapes.pdf','03_infra/fixtures/p307b/reference/07-multi-feature.pdf'}
assert all(sha(p)==x['sha256'] for p,x in prior['files'].items() if p not in allowed)
assert all(sha(p)==h for p,h in files.items())
env.pop('UPDATE_P307B_SNAPSHOTS');cmd=['cargo','test','--offline','--locked','--workspace','--lib','p307b_','--','--test-threads=1']
with (out/'normal.log').open('w') as f:r=subprocess.run(cmd,stdout=f,stderr=subprocess.STDOUT,env=env)
assert r.returncode==0;print(json.dumps(dict(utc=now(),sources_unchanged=True,other_references_unchanged=True,deterministic=True,normal_exit=r.returncode)),flush=True)
