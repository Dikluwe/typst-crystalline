from pathlib import Path
import json,collections,gzip,datetime,hashlib
p=Path(__file__).resolve().parent;q=p.parent/'bibliography-margins-2026-10-18'
old=json.loads((q/'architecture.log').read_text());new=json.loads((p/'architecture.log').read_text())
rows=lambda d:[r for run in d['runs'] for r in run.get('results',[])]
key=lambda r:(r['ruleId'],r.get('level'),r['locations'][0]['physicalLocation']['artifactLocation']['uri'],r['message']['text'])
a=collections.Counter(map(key,rows(new)));b=collections.Counter(map(key,rows(old)))
convert=lambda c:[dict(rule=k[0],level=k[1],file=k[2],message=k[3],count=v) for k,v in sorted(c.items())]
d=dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),baseline=str(q/'architecture.log'),baseline_sha256=hashlib.sha256((q/'architecture.log').read_bytes()).hexdigest(),method='Compare rule/level/consumer/message with multiplicity, ignoring line movement. Explicit binary pin1279707; no suppression or CI pin change.',before=dict(collections.Counter(r['ruleId']+':'+r.get('level','') for r in rows(old))),after=dict(collections.Counter(r['ruleId']+':'+r.get('level','') for r in rows(new))),added=convert(a-b),removed=convert(b-a),notes_decision='V19: exhaustive nominal fragment classes intentionally preserve variants; executing one alternative is not complete coverage. The structural/par ignored-argument group shrank from3 to2 by implementing hanging. V20: nested Option<Value::Length/Dir> patterns directly decode typed style transport, prescribed in owning L0s. Notes remain visible; no lint allow added.')
(p/'lint-diff.json.gz').write_bytes(gzip.compress(json.dumps(d,indent=2,ensure_ascii=False).encode()))
assert not [x for x in d['added'] if x['level'] in ['error','warning']]
print(json.dumps({k:d[k] for k in ['before','after','notes_decision']},indent=2))
