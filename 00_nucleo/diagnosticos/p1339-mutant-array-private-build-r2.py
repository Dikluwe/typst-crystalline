"""Build-only private owner apparatus. Never executes the resulting test binary."""
import base64, datetime, difflib, hashlib, json, os, pathlib, signal, subprocess
ROOT = pathlib.Path(__file__).resolve().parents[2]
D = ROOT / '00_nucleo/diagnosticos'
A = pathlib.Path('/tmp/p1339-mutant-array.jrXHiv')
S = A / 'source-r2'
T = A / 'target-r2'
sha = lambda p: hashlib.sha256(pathlib.Path(p).read_bytes()).hexdigest()
utc = lambda: datetime.datetime.now(datetime.timezone.utc).isoformat()
def new(path, value):
    with pathlib.Path(path).open('x') as f: json.dump(value,f,indent=2,ensure_ascii=False); f.write('\n')
def inventory(source):
    return {str(p.relative_to(source)):sha(p) for p in sorted(source.rglob('*')) if p.is_file()}

if __name__ == '__main__':
    import argparse
    parser=argparse.ArgumentParser(); parser.add_argument('--prepare',action='store_true'); parser.add_argument('--go')
    args=parser.parse_args()
    manifest_path=D/'p1339-mutant-array-private-build-manifest-r2.json'
    if args.prepare:
        pins={name:sha(D/name) for name in [
            'p1339-contract-array-owner-harness-r2.rs','p1339-contract-array-owner-freeze-r1.json',
            'p1339-contract-array-inputs-r2.json','p1339-contract-array-supplement-r2.json',
            'p1339-contract-array-oracles-focal-r1.json','p1339-mutant-array-owner-binding-r2.rs',
            'p1339-mutant-array-build-r1.json','p1339-mutant-array-registry-r1.json',
            'p1339-mutant-array-focal-r1-output/aggregate.json','p1339-mutant-array-private-build-r2.py']}
        patch=''
        for name in ['01_core/src/compiler/eval/call_dispatch.rs','01_core/src/compiler/eval/p1339_mutant_array.rs']:
            patch+=''.join(difflib.unified_diff((A/'source'/name).read_text().splitlines(True),
              (S/name).read_text().splitlines(True),fromfile='r1/'+name,tofile='r2/'+name))
        patch_path=D/'p1339-mutant-array-private-r2.patch'
        with patch_path.open('x') as f: f.write(patch)
        new(manifest_path,{
          'schema':'p1339-array-private-build-manifest-v2','author':'/root/p1336_tests',
          'regime':'executado sem atestacao de isolamento','utc':utc(),'pins':pins,
          'baseline_commit':'2f42d64253547734564513a1159ee6b584c1c4b4',
          'archive_sha256':sha(A/'baseline-build.tar'),
          'source_root':str(S),'source_sha256':inventory(S),'predecessor_source_sha256':inventory(A/'source'),
          'patch':{'path':str(patch_path),'sha256':sha(patch_path)},
          'cache':{'origin':str(A/'target'),'destination':str(T),'copy':['cp','-a','--reflink=auto',str(A/'target'),str(T)],
            'copy_exit':0,'copy_wall_seconds':0.06688811,'shared_writable_target':False,
            'proof':'all workspace .rs mtimes touched before build; actual compiler channels retained; cache is not provenance'},
          'budget':{'Array_cycle':2,'Array_cycle_limit':2,'historical_C_full_used':1,'historical_C_full_limit':2,
            'reset':False,'planned_build_attempts':1,'max_attempts_instrumental_only':2,'timeout_seconds':2700,'jobs':3,
            'planned_runtime_processes':3,'runtime_requires_separate_GO':True,'previous_cycle_CLI_processes':111},
          'mutation':{'family':'ARR-M09-origin','additional_variant_mode':13,'cause':'actual converter omits detached fallback and uses extra.span unchanged',
            'mode9':'always aggregate; preserved','control_mode':0,'families':12,'compiled_negative_variants_after_build':13,
            'not_a_thirteenth_family':True,'no_fixture_lookup':True},
          'oracle':'canonical include unchanged, two positive and eight error calls; no own predecessor private harness used',
          'planned_private_runtime':{'modes':[0,9,13],'control_must_complete_calls':10,
            'mutants_may_abort_at_first_real_oracle_assertion':True,'positive_control_failure':'stop before mutants',
            'E08':'historical Unknown remains; private carrier witness is distinct, not E08 reclassification'},
          'capabilities':{'read':'pinned baseline isolated sources, contract/oracles/L0, own apparatus; no candidate',
            'write':'own p1339-mutant-array-* diagnostics and isolated source-r2/target-r2 only',
            'inherited_context':'P1336 plus historical P1339 adversary/adapter; shared filesystem, no isolation attestation'},
          'argv':['cargo','test','--manifest-path',str(S/'Cargo.toml'),'-p','typst-core','--lib','--release',
            '--no-run','--message-format=json','--locked','--offline','--target-dir',str(T),'-j','3'],
          'env_delta':{'TYPST_COMMIT_SHA':'2f42d64253547734564513a1159ee6b584c1c4b4','NO_COLOR':'1'},
          'status':'PREPARED_NO_PRIVATE_COMPILE_OR_RUNTIME'})
        print(json.dumps({'manifest':str(manifest_path),'sha256':sha(manifest_path)})); raise SystemExit
    assert args.go,'explicit independent build GO required'
    manifest=json.loads(manifest_path.read_text()); go=json.loads(pathlib.Path(args.go).read_text())
    assert go['verdict']=='GO_ARRAY_PRIVATE_BUILD'
    assert go['manifest_sha256']==sha(manifest_path)
    for name,h in manifest['pins'].items(): assert sha(D/name)==h,name
    assert inventory(S)==manifest['source_sha256']
    assert inventory(A/'source')==manifest['predecessor_source_sha256']
    for p in S.rglob('*.rs'): os.utime(p,None)
    record={'schema':'p1339-array-private-build-v2','manifest_sha256':sha(manifest_path),
      'go':{'path':str(pathlib.Path(args.go).resolve()),'sha256':sha(args.go)},'start':utc(),
      'argv':manifest['argv'],'cwd':str(S),'env_delta':manifest['env_delta'],'timeout_seconds':2700,
      'source_before':inventory(S),'no_runtime_executed':True}
    proc=subprocess.Popen(record['argv'],cwd=S,env={**os.environ,**record['env_delta']},stdout=subprocess.PIPE,stderr=subprocess.PIPE,start_new_session=True)
    try: out,err=proc.communicate(timeout=2700);record['transport']='Completed'
    except subprocess.TimeoutExpired:
        os.killpg(proc.pid,signal.SIGKILL);out,err=proc.communicate();record['transport']='Timeout'
    record.update(end=utc(),exit=proc.returncode,source_after=inventory(S))
    for label,raw in [('stdout',out),('stderr',err)]:
        record[label]=raw.decode(errors='replace');record[label+'_base64']=base64.b64encode(raw).decode()
        record[label+'_sha256']=hashlib.sha256(raw).hexdigest();record[label+'_bytes']=len(raw)
    binaries=[]
    for line in out.splitlines():
        try: obj=json.loads(line)
        except (ValueError,UnicodeDecodeError): continue
        if obj.get('reason')=='compiler-artifact' and obj.get('executable') and obj.get('profile',{}).get('test'):
            p=pathlib.Path(obj['executable']); binaries.append({'path':str(p),'sha256':sha(p),'target':obj.get('target')})
    record['binaries']=binaries
    path=D/'p1339-mutant-array-private-build-r2.json';new(path,record)
    print(json.dumps({'receipt':str(path),'sha256':sha(path),'exit':proc.returncode,'binaries':binaries}))
