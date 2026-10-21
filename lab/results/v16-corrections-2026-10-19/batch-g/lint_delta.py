"""Retain full differential diagnostic objects, with multiplicity and locations."""
from pathlib import Path
import gzip,json,collections,datetime
P=Path(__file__).resolve().parent
load=lambda p:json.load(gzip.open(p,'rt'))['runs'][0]['results']
before=load(P/'lint-baseline/after.sarif.gz');after=load(P/'lint-final/after.sarif.gz')
scope=json.loads((P/'scope.json').read_text());allowed=set(scope['sources'])|set(scope['owners'])
encode=lambda o:json.dumps(o,sort_keys=True,separators=(',',':'))
def outside(o):return all(l['physicalLocation']['artifactLocation']['uri'].removeprefix('./') not in allowed for l in o['locations'])
def count(rows):return collections.Counter(encode(o) for o in rows)
b=count(before);a=count(after);bo=count(o for o in before if outside(o));ao=count(o for o in after if outside(o))
expand=lambda c:[{'multiplicity':n,'diagnostic':json.loads(o)} for o,n in sorted(c.items())]
r={'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'baseline':'lint-baseline/after.sarif.gz','candidate':'lint-final/after.sarif.gz','baseline_counts':dict(collections.Counter(o['ruleId']+':'+o['level'] for o in before)),'candidate_counts':dict(collections.Counter(o['ruleId']+':'+o['level'] for o in after)),'outside_scope_identical':bo==ao,'outside_scope_objects':sum(bo.values()),'added':expand(a-b),'removed':expand(b-a),'outside_added':expand(ao-bo),'outside_removed':expand(bo-ao),'policy':'Complete messages, patterns, multiplicities and physical locations retained. Informational reminders remain enabled; grouped alternatives are not claimed individually runtime-covered.'}
(P/'lint-delta.json').write_text(json.dumps(r,indent=2)+'\n');assert bo==ao;assert all(o['level']=='note' for o in after);assert not any(o['ruleId']=='V16' for o in after)
print(json.dumps({k:r[k] for k in ['baseline_counts','candidate_counts','outside_scope_identical','outside_scope_objects']},indent=2))
