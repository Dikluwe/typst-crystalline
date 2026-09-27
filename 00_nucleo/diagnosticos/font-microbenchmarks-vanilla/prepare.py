#!/usr/bin/env python3
"""Serial preparation; no preparation command is a timing sample."""
import json,os,pathlib,subprocess
import measure as m
P=m.P;env=os.environ.copy()
for k in list(env):
 if k.startswith('CARGO_PROFILE_'):env.pop(k)
env.update(CARGO_BUILD_JOBS='1',CARGO_ENCODED_RUSTFLAGS='',RUSTFLAGS='',PYTHONDONTWRITEBYTECODE='1')
base=['cargo','build','--manifest-path',str(P/'Cargo.toml'),'--release','--offline','--locked','--target-dir',str(m.ROOT/'target'),'--message-format=json']
def build(cmd,name):
 before=m.snapshot(name+'-before');start=m.utc()
 with (P/'logs'/(name+'.log')).open('w') as out,(P/'logs'/(name+'.stderr')).open('w') as err:r=subprocess.run(cmd,cwd=m.ROOT,env=env,stdout=out,stderr=err)
 after=m.snapshot(name+'-after');assert r.returncode==0;assert before['sha256']==after['sha256'] and before['head']==after['head']
 m.save(P/'state'/(name+'-receipt.json'),{'utc_start':start,'utc_end':m.utc(),'command':cmd,'environment':{k:env[k] for k in ['CARGO_BUILD_JOBS','CARGO_ENCODED_RUSTFLAGS','RUSTFLAGS']},'before':before['sha256'],'after':after['sha256'],'head':before['head'],'not_a_timing_sample':True})
 return after
cmd=['cargo','rustc',*base[2:],'--','-C','debug-assertions=yes']
build(cmd,'debug-guard-build')
exe=m.ROOT/'target/release/font-vanilla-comparison';r=subprocess.run([str(exe),'--check'],cwd=m.ROOT,env=env,capture_output=True,text=True)
assert r.returncode!=0 and not r.stdout and 'debug builds must not report timings' in r.stderr
m.save(P/'debug-guard.json',{'command':cmd,'executable_sha256':m.sha(exe),'exit_code':r.returncode,'stdout':r.stdout,'stderr':r.stderr,'refused_before_fixture_io_and_criterion':True,'note':'Explicit rustc assertion override exercises actual cfg(debug_assertions); Cargo profile JSON alone does not describe this override. Supported builds clear flags and use the fixed build command.'})
build(base,'build')
check=subprocess.run([str(exe),'--check'],cwd=m.ROOT,env=env,capture_output=True,text=True);assert check.returncode==0,check.stderr
m.save(P/'preparation-fixture-controls.json',json.loads(check.stdout))
cmd=['cargo','bench','--manifest-path','benches/Cargo.toml','--no-run','--offline','--locked','--message-format=json'];build(cmd,'existing-benches-build')
profiles=[]
for line in (P/'logs/existing-benches-build.log').read_text().splitlines():
 try:r=json.loads(line)
 except ValueError:continue
 if r.get('reason')=='compiler-artifact' and r.get('executable') and 'bench' in r['target']['kind']:
  assert r['profile']['opt_level']=='3' and not r['profile']['debug_assertions']
  profiles.append({'target':r['target']['name'],'profile':r['profile'],'executable':r['executable'],'sha256':m.sha(r['executable'])})
assert len(profiles)==8;m.save(P/'benchmark-build-profiles.json',profiles)
print('Preparation passed: actual debug guard rejects; optimized preflight passes; all 8 existing benchmarks compile.',flush=True)
