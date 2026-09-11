"""Repeat/reverse the same canonical owner test and actual modes; no new fixtures."""
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
args=parser.parse_args();mp=D/'p1339-mutant-array-private-expanded-manifest-r1.json'
if args.prepare:
    predecessor=D/'p1339-mutant-array-private-focal-manifest-r2.json'
    m=json.loads(predecessor.read_text())
    aggregate=D/'p1339-mutant-array-private-focal-r2-output/aggregate.json'
    base=json.loads(aggregate.read_text())
    assert base['processes']==3 and not base['paused_control_failure'] and base['pins_intact']
    m['schema']='p1339-array-private-expanded-manifest-v1';m['utc']=utc()
    m['pins'].update({predecessor.name:sha(predecessor),
       str(aggregate.relative_to(D)):sha(aggregate),pathlib.Path(__file__).name:sha(__file__)})
    baseline={}
    for row in base['results']:
        p=pathlib.Path(row['path']);assert sha(p)==row['sha256']
        m['pins'][str(p.relative_to(D))]=row['sha256'];baseline[str(row['mode'])]=str(p)
    m['baseline_raw_by_mode']=baseline
    roles={r['mode']:r['role'] for r in m['process_plan']}
    m['process_plan']=[{'mode':mode,'role':roles[mode],'order':order}
       for order,modes in [('repeat',[0,9,13]),('reverse',[13,9,0])] for mode in modes]
    m['budget'].update(processes=6,previous_private_focal_processes=3,private_total=9,
      new_compiler_attempts=0,affected_CLI_expansion=1332,affected_expansion_plus_private=1338)
    m['stability']='Compare owner DTO rows/completion, exit/signal/transport and panic assertion suffix starting panicked at. Raw full stdout/stderr still retained. Exclude PID/test summary timing from equality.'
    m['negative_observation']='same already-observed semantic assertion under real modes; no new expectation and no mutation score assigned'
    m['control_gate']='repeat starts control; any observation differs from same-mode normal reference stops before next process'
    new(mp,m);print(json.dumps({'manifest':str(mp),'sha256':sha(mp)}));raise SystemExit
assert args.go,'independent runtime GO required'
m=json.loads(mp.read_text());go=json.loads(pathlib.Path(args.go).read_text())
assert go['verdict']=='GO_ARRAY_PRIVATE_EXPANDED_6'
assert go['manifest_sha256']==sha(mp)
def guard():
    for name,h in m['pins'].items():assert sha(D/name)==h,name
    assert sha(m['binary']['path'])==m['binary']['sha256']
    assert inv(pathlib.Path(m['source_root']))==m['source_sha256']
    assert inv(A/'source')==m['predecessor_source_sha256']
    assert sha(A/'typst-array-multiplex-r1')==m['predecessor_binary_sha256']
guard();folder=D/'p1339-mutant-array-private-expanded-r1-output';folder.mkdir()
before=state();results=[];paused=False
for item in m['process_plan']:
    guard();mode=item['mode'];order=item['order'];env={**os.environ,**m['environment_delta'],'P1339_ARRAY_MUTANT':str(mode)}
    row={'mode':mode,'order':order,'role':item['role'],'start':utc(),'argv':m['argv'],'cwd':str(ROOT),
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
    baseline=json.loads(pathlib.Path(m['baseline_raw_by_mode'][str(mode)]).read_text())
    def assertion_suffix(text):
        return text.split('panicked at ',1)[1] if 'panicked at ' in text else text
    compared=['exit','signal','transport','owner_calls','completion','parse_errors']
    row['stable_same_mode']=(all(row[k]==baseline[k] for k in compared)
       and assertion_suffix(row['stderr']['utf8'])==assertion_suffix(baseline['stderr']['utf8']))
    row['baseline_sha256']=sha(m['baseline_raw_by_mode'][str(mode)])
    guard();path=folder/f'{order}-mode-{mode:02d}.json';new(path,row)
    results.append({'mode':mode,'order':order,'stable':row['stable_same_mode'],'path':str(path),'sha256':sha(path),'exit':code,'transport':transport,
      'calls':len(rows),'completion':complete,'classification':row['classification']})
    if not row['stable_same_mode'] or (mode==0 and not row['control_transport_complete']):paused=True;break
guard();aggregate={'schema':'p1339-array-private-expanded-raw-v1','manifest_sha256':sha(mp),
 'go':{'path':str(pathlib.Path(args.go).resolve()),'sha256':sha(args.go)},'state_before':before,'state_after':state(),
 'results':results,'processes':len(results),'paused_control_failure':paused,'pins_intact':True,
 'mutation_score':None,'status':'RAW_COLLECTED_PENDING_INDEPENDENT_VERDICT','E08':'Unknown unchanged, zero credit'}
path=folder/'aggregate.json';new(path,aggregate)
print(json.dumps({'aggregate':str(path),'sha256':sha(path),'processes':len(results),'paused':paused}))
raise SystemExit(1 if paused else 0)
