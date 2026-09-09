"""Preserve source/alias policies after independent focal objection; no product writes."""
import hashlib,importlib.util,json,subprocess,sys
from pathlib import Path
sys.dont_write_bytecode=True
D=Path(__file__).resolve().parent
spec=importlib.util.spec_from_file_location('r',D/'p1335-record.py');r=importlib.util.module_from_spec(spec);spec.loader.exec_module(r)
old=json.loads((D/'p1335-sentinels-cases.json').read_text());f=json.loads((D/'p1335-sentinels-freeze.json').read_text())
assert all(r.sha(p)==h for p,h in f['inputs'].items())
history={c['id']:c for c in json.loads((D/'p1322-sentinels-cases.json').read_text())['cases']}
history.update({c['id']:c for c in json.loads((D/'p1335-classification-required-sentinels.json').read_text())['cases']})
for name in ['p1334-ab-cli-baseline.json','p1334-ab-cross-baseline.json']:
    for row in json.loads((D/name).read_text())['cases']:history.setdefault('p1334.'+row['case'],dict(id='p1334.'+row['case'],expression=row['expression'],origin=name,policy='raw bilateral channels, no historical future expectation'))
for c in old['cases']:
    c['alias_policies']={a:history[a] for a in c['aliases']}
    if c['id'].startswith('closure.'):
        c['document']=Path(c['fixture']).read_text();c['expression']=c['document'];c['source_sha256']=hashlib.sha256(c['document'].encode()).hexdigest()
for a in old['aliases']:a['original_definition']=history[a['id']]
old['revision']=dict(at=r.now(),predecessor_sha256=r.sha(D/'p1335-sentinels-cases.json'),reason='R1 source identity mismatch for two compiled closures; dedup alias original policies now retained in full and separately evaluated. No bilateral expectations changed.',cost='One 20-cell focal execution retained; no full matrix repeated. Distinct from pre-freeze fixture-schema setup incident.')
s=(D/'p1335-sentinels-r1.py').read_text().replace("'p1335-sentinels-freeze.json'","'p1335-sentinels-freeze-r2.json'").replace("'p1335-sentinels-cases.json'","'p1335-sentinels-cases-r2.json'")
s=s.replace("row['runtime_class']=m.classify(row['vanilla'],row['crystalline'])", "row['runtime_class']=m.classify(row['vanilla'],row['crystalline'])\n        row['alias_historical_preservation']={}\n        for alias,definition in c.get('alias_policies',{}).items():\n            if definition.get('legacy_projection'):\n                actual=legacy.envelope(row['crystalline']['exit_code'],row['crystalline']['stdout'],row['crystalline']['stderr'],definition)\n                row['alias_historical_preservation'][alias]=legacy.classify(definition['observations'][p]['future_expected'],actual)")
s=s.replace("'sentinels-'+phase", "'sentinels-'+phase+'-r2'")
s=s.replace("if sys.argv[1]=='freeze':freeze()", "if sys.argv[1]=='freeze':raise RuntimeError('R2 already frozen by separate revision script')")
path=D/'p1335-sentinels-r2.py';assert not path.exists();subprocess.run(['apply_patch'],input='*** Begin Patch\n*** Add File: '+str(path)+'\n'+''.join('+'+l+'\n' for l in s.splitlines())+'*** End Patch\n',text=True,check=True)
r.save('sentinels-cases-r2',old)
f['inputs'].update({str(D/'p1335-sentinels-r2.py'):r.sha(D/'p1335-sentinels-r2.py'),str(D/'p1335-sentinels-cases-r2.json'):r.sha(D/'p1335-sentinels-cases-r2.json'),str(D/'p1335-sentinels-revise.py'):r.sha(D/'p1335-sentinels-revise.py')})
f.update(at=r.now(),revision=old['revision'],predecessor_sha256=r.sha(D/'p1335-sentinels-freeze.json'))
r.save('sentinels-freeze-r2',f)
