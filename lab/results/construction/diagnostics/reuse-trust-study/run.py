#!/usr/bin/env python3
"""Structural probe with exact provenance; no performance samples."""
import datetime,importlib.util,json,os,pathlib,subprocess,hashlib
P=pathlib.Path(__file__).resolve().parent;ROOT=P.parents[2]
spec=importlib.util.spec_from_file_location('read_only_fingerprints',P.with_name('font-microbenchmarks-vanilla')/'measure.py');m=importlib.util.module_from_spec(spec);spec.loader.exec_module(m)
def now():return datetime.datetime.now(datetime.timezone.utc).isoformat()
def save(name,data):(P/name).write_text(json.dumps(data,indent=2)+'\n')
env=os.environ.copy()
for key in list(env):
 if key.startswith(('CARGO_PROFILE_','TYPST_','CRYSTALLINE_')):env.pop(key)
env.update(CARGO_BUILD_JOBS='1',RUSTFLAGS='',CARGO_ENCODED_RUSTFLAGS='',PYTHONDONTWRITEBYTECODE='1')
assert not (P/'results.json').exists(),'Preserve evidence: use a fresh directory'
before=m.fingerprint();save('source-before.json',before)
for name,args in [('status.txt',['git','status','--porcelain=v1','--untracked-files=all']),('diff-stat.txt',['git','diff','HEAD','--stat']),('incoming.patch',['git','diff','HEAD','--binary'])]: (P/name).write_text(subprocess.check_output(args,cwd=ROOT,text=True))
base=['cargo','build','--manifest-path',str(P/'Cargo.toml'),'--release','--offline','--target-dir',str(ROOT/'target'),'--message-format=json']
started=now()
for name,args in [('resolve-build',base),('locked-build',base+['--locked'])]:
 with (P/(name+'.jsonl')).open('w') as out,(P/(name+'.stderr')).open('w') as err:
  subprocess.run(args,cwd=ROOT,env=env,stdout=out,stderr=err,check=True)
artifacts=[json.loads(line) for line in (P/'locked-build.jsonl').read_text().splitlines()]
a=next(r for r in reversed(artifacts) if r.get('reason')=='compiler-artifact' and r.get('executable') and r['target']['name']=='reuse-trust-study')
exe=pathlib.Path(a['executable']);exe_sha=m.sha(exe)
source_inputs={str(f.relative_to(ROOT)):m.sha(f) for f in [P/'run.py',P/'Cargo.toml',P/'Cargo.lock',P/'inputs.json',P/'src/main.rs']}
probe_start=now();result=subprocess.run([str(exe)],cwd=ROOT,env=env,capture_output=True,text=True,check=True);probe_end=now()
save('results.json',json.loads(result.stdout));(P/'probe.stderr').write_text(result.stderr)
after=m.fingerprint();save('source-after.json',after)
assert (before['sha256'],before['head'])==(after['sha256'],after['head'])
assert subprocess.check_output(['git','diff','HEAD','--binary'],cwd=ROOT)==(P/'incoming.patch').read_bytes()
assert m.sha(exe)==exe_sha
for name,digest in source_inputs.items():assert m.sha(ROOT/name)==digest
save('provenance.json',{'preparation_start_utc':started,'probe_start_utc':probe_start,'probe_end_utc':probe_end,'head':before['head'],'source_inputs':source_inputs,'consumer_pin_fingerprint_before':before['sha256'],'consumer_pin_fingerprint_after':after['sha256'],'vanilla_pin':'9f2b6e8715237cb086899a42873660fe744622e8','vanilla_tree':'6ab8bb9434b1bd4d44e1a10fbf1e407b3b86a1a4','build_commands':[base,base+['--locked']],'probe_command':[str(exe)],'executable_sha256':exe_sha,'profile':a['profile'],'toolchain':subprocess.check_output(['rustc','-vV'],text=True),'structural_not_timing_measurement':True,'production_and_prior_measurement_inputs_unchanged':True})
print('Structural controls passed; source fingerprints and tracked patch unchanged.')
