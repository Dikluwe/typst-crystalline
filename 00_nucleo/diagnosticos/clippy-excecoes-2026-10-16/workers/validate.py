from pathlib import Path
import json,gzip,hashlib,subprocess,datetime,collections,os,sys
out=Path('00_nucleo/diagnosticos/clippy-excecoes-2026-10-16');now=lambda:datetime.datetime.now(datetime.timezone.utc).isoformat();before=json.load(gzip.open(out/'before.json.gz'));sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest();hashes={p:sha(p) for p in before['all_source_hashes']}
assert {p for p,h in before['all_source_hashes'].items() if h!=hashes[p]}==set(before['files'])
record=dict(start=now(),head=before['head'],status=subprocess.check_output(['git','status','--porcelain=v1'],text=True),diffstat=subprocess.check_output(['git','diff','HEAD','--stat'],text=True),source_hashes=hashes,commands=[],prior_full_suite='00_nucleo/diagnosticos/clippy-locais-2026-10-16/workspace/workspace-result.json',verification_scope='Corpos, assinaturas, cfg e testes byte-equivalentes após remover somente os atributos novos e hash; compilação all-targets e testes Counter/cobertura atuais, sem repetir suíte funcional completa do mesmo código executável.')
linter='/tmp/tekt-linter-ci-target/debug/crystalline-lint'
commands=[('guard',['python3',str(out/'workers/guard.py')]),('build',['cargo','build','--offline','--locked']),('clippy',['cargo','clippy','--offline','--locked','--workspace','--all-targets','--message-format=json','--','--cap-lints','warn']),('strict',['python3',str(out/'workers/strict.py')]),('fmt',['cargo','fmt','--check','--all']),('lineage',[linter,'.','--checks','v1,v5,v15,v26','--fail-on','warning']),('reverse',['python3','tools/ci/check_reverse_lineage.py']),('architecture',[linter,'.','--format','sarif']),('counter',['cargo','test','--offline','--locked','-p','typst-core','--lib','entities::counter::tests','--','--test-threads=1']),('coverage',['cargo','test','--offline','--locked','-p','typst-infra','--lib','fonts::coverage::tests','--','--test-threads=1']),('rustdoc',['cargo','doc','--offline','--locked','--workspace','--no-deps']),('diff',['git','diff','--check'])]
if '--resume' in sys.argv:
 previous=json.loads((out/'validation-details.json').read_text());assert previous['source_hashes']==hashes
 first=json.loads((out/'first-validation/validation-details.json').read_text());assert len(first['commands'])==4 and first['commands'][-1]['label']=='strict' and first['commands'][-1]['exit']==1
 record=first.copy();record['commands']=first['commands'][:3];record['resume_utc']=now();record['resumed_from']='strict';record['resume_source_hashes_match']=True
 commands=commands[3:]
for label,command in commands:
 start=now();print(json.dumps(dict(start=start,label=label,command=command)),flush=True);env=dict(os.environ)
 if label=='rustdoc':env['RUSTDOCFLAGS']='-D warnings'
 with (out/('validation-'+label+'.log')).open('w') as f:r=subprocess.run(command,stdout=f,stderr=subprocess.STDOUT,env=env)
 record['commands'].append(dict(label=label,command=command,start=start,end=now(),exit=r.returncode));record.update(end=now(),sources_unchanged_during_validation=all(sha(p)==h for p,h in hashes.items()));(out/'validation-details.json').write_text(json.dumps(record,indent=2)+'\n');print(json.dumps(record['commands'][-1]),flush=True);assert record['sources_unchanged_during_validation']
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
  old=json.loads(Path('00_nucleo/diagnosticos/clippy-locais-2026-10-16/clippy-inventory.json').read_text());a=collections.Counter(x['code'] for x in old);b=collections.Counter(x['code'] for x in inventory);record['clippy_delta']={k:b[k]-a[k] for k in a.keys()|b.keys() if b[k]!=a[k]};record['clippy_total']=len(inventory);(out/'validation-details.json').write_text(json.dumps(record,indent=2)+'\n')
  expected_delta={k:-n for k,n in collections.Counter(c['rule'] for c in before['cases']).items()};assert record['clippy_delta']==expected_delta and len(inventory)==28,record['clippy_delta']
 if label=='architecture':
  results=json.loads((out/'validation-architecture.log').read_text())['runs'][0]['results'];counts=collections.Counter(x['ruleId'] for x in results if x['level']=='warning');assert not any(x['level']=='error' for x in results);previous={'V16':327,'V17':28,'V12':19,'V18':6};assert all(n<=previous.get(k,0) for k,n in counts.items()),counts;record['architecture_levels']=dict(collections.Counter(x['level'] for x in results));record['architecture_warnings']=dict(counts);(out/'validation-details.json').write_text(json.dumps(record,indent=2)+'\n')
print(json.dumps(dict(end=record['end'],clippy_total=record['clippy_total'],sources_unchanged=record['sources_unchanged_during_validation'])),flush=True)
