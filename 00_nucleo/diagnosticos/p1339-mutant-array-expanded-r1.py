"""Expanded Array mutation observation; no oracle extrapolation or product reads."""
import argparse, base64, datetime, hashlib, importlib.util, json, os, pathlib, resource, signal, subprocess
ROOT=pathlib.Path(__file__).resolve().parents[2]
D=ROOT/'00_nucleo/diagnosticos'
MANIFEST=D/'p1339-mutant-array-expanded-manifest-r1.json'
REGISTRY=D/'p1339-mutant-array-registry-r1.json'
BUILD=D/'p1339-mutant-array-build-r1.json'
BINARY=pathlib.Path('/tmp/p1339-mutant-array.jrXHiv/typst-array-multiplex-r1')
UNKNOWN_ID='ARR-E08-detached-surplus'
PROFILES={'default':[], 'html':['--features','html'], 'a11y':['--features','a11y-extras'], 'html+a11y':['--features','html,a11y-extras']}
ORDERS=['normal','repeat','reverse']
FROZEN=D/'p1339-mutant-array-focal-r1.py'
assert hashlib.sha256(FROZEN.read_bytes()).hexdigest()=='7ed3dd412191d53fc47511cd642e016fe87c4e570d2131c32115ab51fbc4c8ec'
spec=importlib.util.spec_from_file_location('frozen_transport',FROZEN)
f=importlib.util.module_from_spec(spec);spec.loader.exec_module(f)
sha,utc,read,pin,write_new,no_core,channels,state=[getattr(f,k) for k in ['sha','utc','read','pin','write_new','no_core','channels','state']]
def integrity(m):
    current={p['path']:sha(p['path']) for p in m['protected_inputs']}
    for p in m['protected_inputs']:assert current[p['path']]==p['sha256'],p['path']
    source=pathlib.Path(read(BUILD)['source_root'])
    inv={str(p.relative_to(source)):sha(p) for p in source.rglob('*') if p.is_file()}
    assert inv==read(BUILD)['source_sha256']
    return {'protected':current,'source_sha256':inv,'binary':pin(BINARY)}
def run(case,mode,role,family,profile,order,flags):
    binary_before=pin(BINARY)
    delta={'NO_COLOR':'1','PYTHONDONTWRITEBYTECODE':'1','P1339_ARRAY_MUTANT':str(mode),'RUST_BACKTRACE':'0'}
    effective={**os.environ,**delta}
    relevant={k:v for k,v in effective.items() if k in ['PATH','LANG','LC_ALL','LC_CTYPE','TZ','HOME','NO_COLOR','PYTHONDONTWRITEBYTECODE','SOURCE_DATE_EPOCH','RUST_BACKTRACE','P1339_ARRAY_MUTANT'] or k.startswith(('TYPST_','FONTCONFIG_','XDG_'))}
    argv=[str(BINARY),'eval',*flags,case['source']]
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
    raw={'id':case['id'],'family':family,'role':role,'mode':mode,'profile':profile,'order':order,
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

def make_plan(cases,registry):
    plan=[]
    for order in ORDERS:
        for profile in PROFILES:
            ids=[c['id'] for c in cases]
            witnesses=[{'role':'mutant','mode':m['mode'],'id':i,'family':m['id']} for m in registry['modes'] for i in m['prospective_witness_ids']]
            if order=='reverse':ids.reverse();witnesses.reverse()
            block=[{'role':'control','mode':0,'id':i,'family':None} for i in ids]
            block+=witnesses+[{'role':'invalid_mode','mode':255,'id':'ARR-P01-empty','family':None}]
            assert len(block)==55
            plan.extend([{**r,'profile':profile,'order':order} for r in block])
    return plan
def main():
    parser=argparse.ArgumentParser();parser.add_argument('--prepare',action='store_true')
    parser.add_argument('--oracles');parser.add_argument('--go');args=parser.parse_args()
    base_cases=read(D/'p1339-contract-array-oracles-focal-r1.json')['cases']
    registry=read(REGISTRY)
    if args.prepare:
        assert args.oracles
        op=pathlib.Path(args.oracles).resolve()
        assert op.parent==D and op.name.startswith('p1339-contract-array-')
        oracle=read(op)
        # The author must provide measured literals for each profile/order, not a default clone.
        cells=oracle['cases'];lookup={(c['id'],c['profile'],c['order']):c for c in cells}
        assert len(cells)==len(lookup)==336
        for base in base_cases:
            for profile in PROFILES:
                for order in ORDERS:
                    c=lookup[base['id'],profile,order]
                    assert c['source']==base['source']
                    if base['id']==UNKNOWN_ID:assert c['expected'] is None
                    else:
                        e=c['expected'];assert e['exit'] in [0,1]
                        assert isinstance(e['stdout'],str) and isinstance(e['stderr'],str)
        old=read(D/'p1339-mutant-array-focal-manifest-r1.json')
        protected=old['protected_inputs']+[pin(op),pin(__file__),pin(FROZEN),
            pin(D/'p1339-mutant-array-private-focal-r2-output/aggregate.json')]
        unique={p['path']:p for p in protected}
        plan=make_plan(base_cases,registry)
        write_new(MANIFEST,{
            'schema':'p1339-array-expanded-mutant-manifest-v1','utc':utc(),'author':'/root/p1336_tests',
            'regime':'executado sem atestacao de isolamento','executor':pin(__file__),'oracles':pin(op),
            'protected_inputs':list(unique.values()),'profiles':PROFILES,'orders':ORDERS,'process_plan':plan,
            'processes':660,'source_formula':'same28 controls + same26 family witnesses + invalid255 for each12cells',
            'reverse':'reverse controls and full witness list, keep control-first gate and invalid-mode last; normal/repeat identical order',
            'control_gate':'each55-process block stops before26mutants unless all27non-E08 controls match that measured profile/order literal',
            'budget':{'previous_focal_CLI':111,'private_processes':3,'reference_expansion':672,'mutant_expansion':660,
                'affected_expansion_total':1332,'Array_focal_cycles_used':2,'Array_focal_cycles_limit':2,'historical_full_C_used':1,'historical_full_C_limit':2,'reset':False},
            'runtime':'sequential processes,30s each,RLIMIT_CORE0,full immutable channels and binary checks before/after each',
            'E08':'12controls+12mode9 rows retainUnknown; invalid255 separately12Unknown, never semantic rejection credit',
            'no_credit':'No execution or verdict from preparation; precondition is accepted private focal and new independent GO'})
        print(json.dumps({'manifest':pin(MANIFEST)}));return
    assert args.go
    m=read(MANIFEST);go=read(args.go)
    assert go['verdict']=='GO_ARRAY_MUTANT_EXPANDED_660'
    assert go['executor_sha256']==sha(__file__) and go['manifest_sha256']==sha(MANIFEST)
    assert m['executor']['sha256']==sha(__file__)
    before=integrity(m);cases=read(m['oracles']['path'])['cases']
    lookup={(c['id'],c['profile'],c['order']):c for c in cases}
    plan=make_plan(base_cases,registry);assert plan==m['process_plan'] and len(plan)==660
    output=D/'p1339-mutant-array-expanded-r1-output';output.mkdir()
    started=utc();git_before=state();rows=[];pins=[];paused=False
    for index,item in enumerate(plan):
        offset=index%55
        if offset==28:
            controls=[r for r in rows[-28:] if r['id']!=UNKNOWN_ID]
            if len(controls)!=27 or any(r['comparison']!='ExactLiteralMatch' for r in controls):
                paused=True;break
        case=lookup[item['id'],item['profile'],item['order']]
        row=run(case,item['mode'],item['role'],item['family'],item['profile'],item['order'],PROFILES[item['profile']])
        p=output/f'{index:03d}-{item["order"]}-{item["profile"]}-{item["mode"]}-{item["id"]}.json'
        write_new(p,row);pins.append(pin(p));rows.append(row)
    after_error=None
    try:after=integrity(m)
    except Exception as e:after={'error':repr(e)};after_error=repr(e)
    groups=[]
    for order in ORDERS:
        for profile in PROFILES:
            for mutant in registry['modes']:
                actual=[r for r in rows if (r['family'],r['profile'],r['order'])==(mutant['id'],profile,order)]
                groups.append({'family':mutant['id'],'mode':mutant['mode'],'profile':profile,'order':order,
                    'rows':len(actual),'differences':[r['id'] for r in actual if r['comparison']=='ObservedLiteralDifference'],
                    'matches':[r['id'] for r in actual if r['comparison']=='ExactLiteralMatch'],
                    'unknowns':[r['id'] for r in actual if r['comparison'].startswith('Unknown')],'independent_verdict':None})
    aggregate={'schema':'p1339-array-expanded-raw-v1','author':'/root/p1336_tests','regime':'executado sem atestacao de isolamento',
        'start':started,'end':utc(),'manifest':pin(MANIFEST),'executor':pin(__file__),'go':pin(args.go),
        'integrity_before':before,'integrity_after':after,'integrity_error':after_error,
        'git_before':git_before,'git_after':state(),'raw_rows':pins,'actual_processes':len(rows),'planned_processes':660,
        'paused_control_failure':paused,'control_matches':sum(r['role']=='control' and r['comparison']=='ExactLiteralMatch' for r in rows),
        'control_differences':[{'id':r['id'],'profile':r['profile'],'order':r['order']} for r in rows if r['role']=='control' and r['comparison']=='ObservedLiteralDifference'],
        'unknowns':[{'id':r['id'],'mode':r['mode'],'profile':r['profile'],'order':r['order'],'reason':r['comparison']} for r in rows if r['comparison'].startswith('Unknown')],
        'groups':groups,'mutation_score':None,'status':'RAW_COLLECTED_PENDING_INDEPENDENT_VERDICT'}
    p=output/'aggregate.json';write_new(p,aggregate)
    print(json.dumps({'aggregate':pin(p),'processes':len(rows),'paused':paused,'control_matches':aggregate['control_matches']}))
    raise SystemExit(1 if paused or after_error else 0)
if __name__=='__main__':main()
