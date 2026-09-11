"""Run only the canonical owner test: real control, then two origin variants."""
import argparse, base64, datetime, hashlib, json, os, pathlib, resource, signal, subprocess
ROOT=pathlib.Path(__file__).resolve().parents[2]
D=ROOT/'00_nucleo/diagnosticos'
A=pathlib.Path('/tmp/p1339-mutant-array.jrXHiv')
sha=lambda p:hashlib.sha256(pathlib.Path(p).read_bytes()).hexdigest()
utc=lambda:datetime.datetime.now(datetime.timezone.utc).isoformat()
def new(path,value):
    with pathlib.Path(path).open('x') as f: json.dump(value,f,indent=2,ensure_ascii=False);f.write('\n')
def inv(root):
    return {str(p.relative_to(root)):sha(p) for p in sorted(root.rglob('*')) if p.is_file()}
def core_limit(): resource.setrlimit(resource.RLIMIT_CORE,(0,0))
def state():
    args=['git','diff','HEAD','--stat','--','.',':(exclude)00_nucleo/materialization',':(exclude)00_nucleo/context']
    return {'utc':utc(),'head':subprocess.check_output(['git','rev-parse','HEAD'],cwd=ROOT).decode().strip(),
      'working_tree_stat':subprocess.check_output(args,cwd=ROOT).decode(),
      'status':subprocess.check_output(['git','status','--porcelain','--untracked-files=all','--','.',
        ':(exclude)00_nucleo/materialization',':(exclude)00_nucleo/context'],cwd=ROOT).decode()}
def raw(blob):
    return {'utf8':blob.decode(errors='replace'),'base64':base64.b64encode(blob).decode(),
      'sha256':hashlib.sha256(blob).hexdigest(),'bytes':len(blob)}

parser=argparse.ArgumentParser();parser.add_argument('--prepare',action='store_true');parser.add_argument('--go')
args=parser.parse_args();mp=D/'p1339-mutant-array-private-focal-manifest-r2.json'
if args.prepare:
    build=json.loads((D/'p1339-mutant-array-private-build-r2.json').read_text())
    assert build['exit']==0 and build['source_before']==build['source_after']
    assert len(build['binaries'])==1,build['binaries']
    b=build['binaries'][0];assert sha(b['path'])==b['sha256']
    pins={name:sha(D/name) for name in ['p1339-contract-array-owner-harness-r2.rs',
      'p1339-contract-array-owner-freeze-r1.json','p1339-mutant-array-owner-binding-r2.rs',
      'p1339-mutant-array-private-build-manifest-r2.json','p1339-mutant-array-private-build-r2.json',
      'p1339-mutant-array-private-focal-r2.py','p1339-mutant-array-registry-r1.json',
      'p1339-mutant-array-focal-r1-output/aggregate.json']}
    new(mp,{'schema':'p1339-array-private-focal-manifest-v2','utc':utc(),'author':'/root/p1336_tests',
      'regime':'executado sem atestacao de isolamento','pins':pins,'binary':b,
      'source_root':str(A/'source-r2'),'source_sha256':build['source_after'],
      'predecessor_source_sha256':inv(A/'source'),'predecessor_binary_sha256':sha(A/'typst-array-multiplex-r1'),
      'argv':[b['path'],'compiler::eval::call_dispatch::p1339_array_owner_binding::p1339_array_owner_frozen',
        '--exact','--nocapture','--test-threads=1'],
      'process_plan':[{'mode':0,'role':'real narrow control'},{'mode':9,'role':'ARR-M09 known-origin overwritten by aggregate'},
        {'mode':13,'role':'ARR-M09 detached-origin fallback omitted'}],
      'environment_delta':{'NO_COLOR':'1','RUST_BACKTRACE':'0','PYTHONDONTWRITEBYTECODE':'1'},
      'timeout_seconds':30,'RLIMIT_CORE':[0,0],
      'raw_protocol':'one unmodified canonical harness invocation per process; full stdout/stderr/base64/sha/exit/signal/UTC/env and input/result Debug rows. No inferred PASS from empty rows.',
      'control_gate':'exit0 + exactly10 consecutive call indices1..10 + completion calls10; otherwise stop before modes9/13',
      'negative_observation':'record real normal test-harness exit101 and canonical assertion panic, with preceding rows. No rejection credit assigned by executor; verifier audits messages/spans and branch cause.',
      'budget':{'Array_cycle':2,'Array_cycle_limit':2,'processes':3,'compiler_attempts':1,'previous_cycle_CLI_processes':111,
        'historical_C_full_used':1,'historical_C_full_limit':2,'families':12,'negative_variants':13,'reset':False},
      'E08':'historical Unknown unchanged, zero credit. New legitimate private Args witnesses are independent.'})
    print(json.dumps({'manifest':str(mp),'sha256':sha(mp)}));raise SystemExit
assert args.go,'independent runtime GO required'
m=json.loads(mp.read_text());go=json.loads(pathlib.Path(args.go).read_text())
assert go['verdict']=='GO_ARRAY_PRIVATE_FOCAL_3'
assert go['manifest_sha256']==sha(mp)
def guard():
    for name,h in m['pins'].items():assert sha(D/name)==h,name
    assert sha(m['binary']['path'])==m['binary']['sha256']
    assert inv(pathlib.Path(m['source_root']))==m['source_sha256']
    assert inv(A/'source')==m['predecessor_source_sha256']
    assert sha(A/'typst-array-multiplex-r1')==m['predecessor_binary_sha256']
guard();folder=D/'p1339-mutant-array-private-focal-r2-output';folder.mkdir()
before=state();results=[];paused=False
for item in m['process_plan']:
    guard();mode=item['mode'];env={**os.environ,**m['environment_delta'],'P1339_ARRAY_MUTANT':str(mode)}
    row={'mode':mode,'role':item['role'],'start':utc(),'argv':m['argv'],'cwd':str(ROOT),
      'environment':{k:v for k,v in env.items() if k in ['PATH','HOME','LANG','LANGUAGE','LC_ALL','TZ','NO_COLOR','RUST_BACKTRACE','PYTHONDONTWRITEBYTECODE','P1339_ARRAY_MUTANT'] or k.startswith(('LC_','TYPST_','FONTCONFIG_','XDG_'))},
      'stdin':raw(b''),'timeout_seconds':m['timeout_seconds'],'RLIMIT_CORE':m['RLIMIT_CORE'],'binary_before':sha(m['binary']['path'])}
    try:
        proc=subprocess.Popen(m['argv'],cwd=ROOT,env=env,stdin=subprocess.PIPE,stdout=subprocess.PIPE,stderr=subprocess.PIPE,start_new_session=True,preexec_fn=core_limit)
        try:out,err=proc.communicate(b'',timeout=m['timeout_seconds']);transport='Completed'
        except subprocess.TimeoutExpired:
            os.killpg(proc.pid,signal.SIGKILL);out,err=proc.communicate();transport='Timeout'
        code=proc.returncode
    except OSError as error:out=b'';err=str(error).encode();code=None;transport='SpawnError'
    rows=[];complete=[];parse_errors=[]
    for line in out.decode(errors='replace').splitlines():
        for prefix,target in [('P1339_ARRAY_OWNER_RAW ',rows),('P1339_ARRAY_OWNER_COMPLETED ',complete)]:
            # libtest may prepend its test-name text to the first nocapture line.
            if prefix in line:
                try:target.append(json.loads(line.split(prefix,1)[1]))
                except ValueError as error:parse_errors.append(str(error))
    row.update(end=utc(),exit=code,signal=(-code if code is not None and code<0 else None),
      transport=transport,stdout=raw(out),stderr=raw(err),owner_calls=rows,completion=complete,
      parse_errors=parse_errors,binary_after=sha(m['binary']['path']))
    row['control_transport_complete']=(transport=='Completed' and code==0 and not parse_errors
      and [x.get('call_index') for x in rows]==list(range(1,11)) and complete==[{'calls':10}]) if mode==0 else None
    row['classification']='ObservedRawOnlyPendingIndependentVerdict'
    if transport!='Completed' or code not in [0,101] or parse_errors:row['classification']='UnknownTransportOrSchema'
    guard();path=folder/f'mode-{mode:02d}.json';new(path,row)
    results.append({'mode':mode,'path':str(path),'sha256':sha(path),'exit':code,'transport':transport,
      'calls':len(rows),'completion':complete,'classification':row['classification']})
    if mode==0 and not row['control_transport_complete']:paused=True;break
guard();aggregate={'schema':'p1339-array-private-focal-raw-v2','manifest_sha256':sha(mp),
 'go':{'path':str(pathlib.Path(args.go).resolve()),'sha256':sha(args.go)},'state_before':before,'state_after':state(),
 'results':results,'processes':len(results),'paused_control_failure':paused,'pins_intact':True,
 'mutation_score':None,'status':'RAW_COLLECTED_PENDING_INDEPENDENT_VERDICT','E08':'Unknown unchanged, zero credit'}
path=folder/'aggregate.json';new(path,aggregate)
print(json.dumps({'aggregate':str(path),'sha256':sha(path),'processes':len(results),'paused':paused}))
raise SystemExit(1 if paused else 0)
