from pathlib import Path
import json,gzip,hashlib,subprocess,datetime,collections,os
out=Path('00_nucleo/diagnosticos/clippy-expressions-2026-10-16');now=lambda:datetime.datetime.now(datetime.timezone.utc).isoformat()
before=json.load(gzip.open(out/'before.json.gz'))
sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest()
hashes={p:sha(p) for p in before['all_source_hashes']}
assert {p for p,h in before['all_source_hashes'].items() if h!=hashes[p]}==set(before['files'])
record=dict(start=now(),head=before['head'],status=subprocess.check_output(['git','status','--porcelain=v1'],text=True),diffstat=subprocess.check_output(['git','diff','HEAD','--stat'],text=True),source_hashes=hashes,commands=[])
linter='/tmp/tekt-linter-ci-target/debug/crystalline-lint'
commands=[('build',['cargo','build','--offline','--locked']),('clippy',['cargo','clippy','--offline','--locked','--workspace','--all-targets','--message-format=json','--','--cap-lints','warn']),
    ('fmt',['cargo','fmt','--check','--all']),('lineage',[linter,'.','--checks','v1,v5,v15,v26','--fail-on','warning']),('reverse',['python3','tools/ci/check_reverse_lineage.py']),
    ('architecture',[linter,'.','--format','sarif']),('workspace',['python3','00_nucleo/diagnosticos/clippy-expressions-2026-10-16/workers/suite.py']),('rustdoc',['cargo','doc','--offline','--locked','--workspace','--no-deps']),('diff',['git','diff','--check'])]
for label,command in commands:
    start=now();print(json.dumps(dict(start=start,label=label,command=command)),flush=True)
    env=dict(os.environ)
    if label=='rustdoc':env['RUSTDOCFLAGS']='-D warnings'
    with (out/('validation-'+label+'.log')).open('w') as f:r=subprocess.run(command,stdout=f,stderr=subprocess.STDOUT,env=env)
    record['commands'].append(dict(label=label,command=command,start=start,end=now(),exit=r.returncode))
    record.update(end=now(),sources_unchanged_during_validation=all(sha(p)==h for p,h in hashes.items()))
    (out/'validation-details.json').write_text(json.dumps(record,indent=2)+'\n');print(json.dumps(record['commands'][-1]),flush=True);assert record['sources_unchanged_during_validation']
    if r.returncode:raise SystemExit(r.returncode)
    if label=='clippy':
        unique={}
        for line in (out/'validation-clippy.log').read_text().splitlines():
            try:x=json.loads(line)
            except ValueError:continue
            if x.get('reason')!='compiler-message':continue
            m=x['message'];code=m.get('code');spans=[s for s in m.get('spans',[]) if s.get('is_primary')]
            if not code or not spans or not all(s['file_name'].startswith(('01_core/','02_shell/','03_infra/','04_wiring/','benches/')) for s in spans):continue
            unique[(code['code'],m['message'],tuple((s['file_name'],s['byte_start'],s['byte_end']) for s in spans))]=dict(code=code['code'],message=m['message'],spans=spans)
        inventory=list(unique.values());(out/'clippy-inventory.json').write_text(json.dumps(inventory,ensure_ascii=False,indent=2)+'\n')
        old=json.loads(Path('00_nucleo/diagnosticos/field-reassign-2026-10-15/clippy-inventory.json').read_text());a=collections.Counter(x['code'] for x in old);b=collections.Counter(x['code'] for x in inventory)
        record['clippy_delta']={k:b[k]-a[k] for k in a.keys()|b.keys() if b[k]!=a[k]};record['clippy_total']=len(inventory)
        (out/'validation-details.json').write_text(json.dumps(record,indent=2)+'\n')
        expected_delta={k:-n for k,n in collections.Counter(c['rule'] for c in before['cases']).items()};assert record['clippy_delta']==expected_delta,record['clippy_delta']
    if label=='architecture':
        results=json.loads((out/'validation-architecture.log').read_text())['runs'][0]['results'];counts=collections.Counter(x['ruleId'] for x in results if x['level']=='warning')
        assert not any(x['level']=='error' for x in results)
        previous={'V16':327,'V17':28,'V12':19,'V18':6};assert all(n<=previous.get(k,0) for k,n in counts.items()),counts
        record['architecture_levels']=dict(collections.Counter(x['level'] for x in results));record['architecture_warnings']=dict(counts)
        (out/'validation-details.json').write_text(json.dumps(record,indent=2)+'\n')
print(json.dumps(dict(end=record['end'],clippy_total=record['clippy_total'],clippy_delta=record['clippy_delta'],sources_unchanged=record['sources_unchanged_during_validation'])),flush=True)
