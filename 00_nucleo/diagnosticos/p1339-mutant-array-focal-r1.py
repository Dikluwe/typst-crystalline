"""Frozen default Array mutation focal: real CLI only, complete raw transport.

Requires independent GO pinned to this executor and its prospective manifest.
No private bridge, expanded matrix, candidate or product mutation is performed.
"""
import argparse, base64, datetime, hashlib, json, os, pathlib, resource, signal, subprocess

ROOT=pathlib.Path(__file__).resolve().parents[2]
D=ROOT/'00_nucleo/diagnosticos'
MANIFEST=D/'p1339-mutant-array-focal-manifest-r1.json'
ORACLES=D/'p1339-contract-array-oracles-focal-r1.json'
REGISTRY=D/'p1339-mutant-array-registry-r1.json'
BUILD=D/'p1339-mutant-array-build-r1.json'
BINARY=pathlib.Path('/tmp/p1339-mutant-array.jrXHiv/typst-array-multiplex-r1')
UNKNOWN_ID='ARR-E08-detached-surplus'

def sha(p):return hashlib.sha256(pathlib.Path(p).read_bytes()).hexdigest()
def utc():return datetime.datetime.now(datetime.timezone.utc).isoformat()
def read(p):return json.loads(pathlib.Path(p).read_text())
def pin(p):return {'path':str(pathlib.Path(p).resolve()),'sha256':sha(p)}
def write_new(p,x):
    with pathlib.Path(p).open('x') as f:json.dump(x,f,ensure_ascii=False,indent=2);f.write('\n')
def no_core():resource.setrlimit(resource.RLIMIT_CORE,(0,0))
def channels(raw):
    return {name:{'utf8':data.decode('utf-8','replace'),'base64':base64.b64encode(data).decode(),
                  'sha256':hashlib.sha256(data).hexdigest(),'bytes':len(data)} for name,data in raw.items()}
def state():
    commands=[['git','rev-parse','HEAD'],['git','status','--porcelain=v1','--untracked-files=all','--','.',':!00_nucleo/materialization',':!00_nucleo/context'],
              ['git','diff','HEAD','--stat','--','.',':!00_nucleo/materialization',':!00_nucleo/context']]
    result=[]
    for argv in commands:
        p=subprocess.run(argv,cwd=ROOT,capture_output=True,timeout=30)
        result.append({'argv':argv,'exit':p.returncode,'channels':channels({'stdout':p.stdout,'stderr':p.stderr})})
    return result
def integrity(manifest):
    actual={item['path']:sha(item['path']) for item in manifest['protected_inputs']}
    for item in manifest['protected_inputs']:assert actual[item['path']]==item['sha256'],('protected drift',item['path'])
    build=read(BUILD);source=pathlib.Path(build['source_root'])
    files={str(p.relative_to(source)):sha(p) for p in source.rglob('*') if p.is_file()}
    assert files==build['source_sha256'],'isolated compiled source/config drift'
    return {'protected':actual,'compiled_source_inventory':files,'binary':pin(BINARY),'executor':pin(__file__),'manifest':pin(MANIFEST)}
def run(case,mode,role,family):
    binary_before=pin(BINARY)
    delta={'NO_COLOR':'1','PYTHONDONTWRITEBYTECODE':'1','P1339_ARRAY_MUTANT':str(mode),'RUST_BACKTRACE':'0'}
    effective={**os.environ,**delta}
    relevant={k:v for k,v in effective.items() if k in ['PATH','LANG','LC_ALL','LC_CTYPE','TZ','HOME','NO_COLOR','PYTHONDONTWRITEBYTECODE','SOURCE_DATE_EPOCH','RUST_BACKTRACE','P1339_ARRAY_MUTANT'] or k.startswith(('TYPST_','FONTCONFIG_','XDG_'))}
    argv=[str(BINARY),'eval',case['source']]
    started=utc();error=None
    try:
        proc=subprocess.Popen(argv,cwd=ROOT,env=effective,stdin=subprocess.PIPE,stdout=subprocess.PIPE,stderr=subprocess.PIPE,preexec_fn=no_core,start_new_session=True)
        try:
            out,err=proc.communicate(input=b'',timeout=30);transport='Completed'
        except subprocess.TimeoutExpired:
            os.killpg(proc.pid,signal.SIGKILL);out,err=proc.communicate();transport='Timeout'
        code=proc.returncode
        if transport=='Completed' and code<0:transport='Signal'
        elif transport=='Completed' and code not in (0,1):transport='AbnormalExit'
    except OSError as exc:
        out=err=b'';code=None;transport='SpawnError';error=repr(exc)
    ended=utc();binary_after=pin(BINARY)
    raw={'id':case['id'],'family':family,'role':role,'mode':mode,'profile':'default','order':'normal',
         'source':case['source'],'source_sha256':hashlib.sha256(case['source'].encode()).hexdigest(),
         'argv':argv,'cwd':str(ROOT),'stdin_base64':'','utc_start':started,'utc_end':ended,
         'environment_delta':delta,'effective_relevant_environment':relevant,'core_limit':0,'timeout_seconds':30,
         'binary_before':binary_before,'binary_after':binary_after,'transport':transport,'exit':code,
         'signal':-code if code is not None and code<0 else None,'spawn_error':error,
         'channels':channels({'stdout':out,'stderr':err})}
    expected=case['expected']
    if binary_before!=binary_after:classification='UnknownIdentityDrift'
    elif role=='invalid_mode':classification='UnknownInvalidModeRaw'
    elif transport!='Completed':classification='UnknownTransport'
    elif expected is None:classification='UnknownNoFrozenExpected'
    else:
        equal=code==expected['exit'] and out==expected['stdout'].encode('utf-8') and err==expected['stderr'].encode('utf-8')
        classification='ExactLiteralMatch' if equal else 'ObservedLiteralDifference'
    raw['comparison']=classification
    return raw

def main():
    parser=argparse.ArgumentParser();parser.add_argument('--go',required=True);args=parser.parse_args()
    manifest=read(MANIFEST);go=read(args.go)
    assert go['verdict']=='GO_ARRAY_MUTANT_FOCAL_55'
    assert go['executor_sha256']==sha(__file__) and go['manifest_sha256']==sha(MANIFEST)
    assert manifest['executor']['sha256']==sha(__file__)
    before=integrity(manifest)
    cases=read(ORACLES)['cases'];lookup={c['id']:c for c in cases}
    assert len(cases)==len(lookup)==28
    assert [c['id'] for c in cases if c['expected'] is None]==[UNKNOWN_ID]
    registry=read(REGISTRY);assert len(registry['modes'])==12
    plan=manifest['process_plan']
    expected_plan=[{'role':'control','mode':0,'id':c['id'],'family':None} for c in cases]
    expected_plan += [{'role':'mutant','mode':m['mode'],'id':i,'family':m['id']} for m in registry['modes'] for i in m['prospective_witness_ids']]
    expected_plan += [{'role':'invalid_mode','mode':255,'id':'ARR-P01-empty','family':None}]
    assert plan==expected_plan and len(plan)==55
    output=D/'p1339-mutant-array-focal-r1-output';output.mkdir(exist_ok=False)
    started=utc();git_before=state();rows=[];row_pins=[];pause=False
    for index,item in enumerate(plan):
        if index==28:
            expected_controls=[r for r in rows if r['id']!=UNKNOWN_ID]
            if len(expected_controls)!=27 or any(r['comparison']!='ExactLiteralMatch' for r in expected_controls):
                pause=True;break
        row=run(lookup[item['id']],item['mode'],item['role'],item['family'])
        path=output/f'{index:02d}-{item["role"]}-{item["mode"]}-{item["id"]}.json'
        write_new(path,row);row_pins.append(pin(path));rows.append(row)
    after_error=None
    try:after=integrity(manifest)
    except Exception as exc:after={'error':repr(exc)};after_error=repr(exc)
    groups=[]
    for mode in registry['modes']:
        actual=[r for r in rows if r['family']==mode['id']]
        groups.append({'id':mode['id'],'mode':mode['mode'],'rows':len(actual),
                       'planned_rows':len(mode['prospective_witness_ids']),
                       'observed_difference_ids':[r['id'] for r in actual if r['comparison']=='ObservedLiteralDifference'],
                       'exact_match_ids':[r['id'] for r in actual if r['comparison']=='ExactLiteralMatch'],
                       'unknown_ids':[r['id'] for r in actual if r['comparison'].startswith('Unknown')],
                       'independent_rejection_verdict':None})
    record={'schema':'p1339-array-mutant-focal-evidence-v1','author':'/root/p1336_tests',
            'regime':'executado sem atestacao de isolamento','authority_manifest_sha256':manifest['authority_manifest_sha256'],
            'utc_start':started,'utc_end':utc(),'go':pin(args.go),'manifest':pin(MANIFEST),'executor':pin(__file__),
            'integrity_before':before,'integrity_after':after,'integrity_error':after_error,
            'git_before':git_before,'git_after':state(),'raw_rows':row_pins,'actual_processes':len(rows),'planned_processes':55,
            'paused_before_mutants_due_control':pause,'control_literal_matches':sum(r['role']=='control' and r['comparison']=='ExactLiteralMatch' for r in rows),
            'control_literal_differences':[r['id'] for r in rows if r['role']=='control' and r['comparison']=='ObservedLiteralDifference'],
            'unknown_rows':[{'role':r['role'],'mode':r['mode'],'id':r['id'],'reason':r['comparison']} for r in rows if r['comparison'].startswith('Unknown')],
            'families':groups,'status':'PAUSED_CONTROL_MISMATCH' if pause else 'FOCAL_RAW_COLLECTED_PENDING_INDEPENDENT_VERDICT',
            'no_seal':'E08 remains Unknown and earns zero credit; private bridge still due. No mutation score or candidate/runtime-F PASS asserted.'}
    aggregate=output/'aggregate.json';write_new(aggregate,record)
    print(json.dumps({'aggregate':pin(aggregate),'processes':len(rows),'control_matches':record['control_literal_matches'],'paused':pause,'status':record['status']}))
    raise SystemExit(1 if pause or after_error else 0)

if __name__=='__main__':main()
