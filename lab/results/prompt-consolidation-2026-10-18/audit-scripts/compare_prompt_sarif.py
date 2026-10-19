from pathlib import Path
import json,collections,datetime
root=Path('/repos/Antigravity/typst-crystalline');out=root/'lab/results/prompt-consolidation-2026-10-18'
a=root/'lab/results/prompt-layout-migration-2026-10-18/final-lint.sarif';b=out/'final-lint.sarif'
def records(p):
 d=json.loads(p.read_text());return [r for run in d['runs'] for r in run.get('results',[])]
def key(r):
 loc=r.get('locations',[{}])[0].get('physicalLocation',{})
 return (r['ruleId'],r.get('level'),r.get('message',{}).get('text'),loc.get('artifactLocation',{}).get('uri'),json.dumps(loc.get('region',{}),sort_keys=True))
old=records(a);new=records(b);oc=collections.Counter(map(key,old));nc=collections.Counter(map(key,new))
result={'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'baseline':str(a.relative_to(root)),'candidate':str(b.relative_to(root)),'baseline_counts':dict(collections.Counter(r['ruleId']+':'+str(r.get('level')) for r in old)),'candidate_counts':dict(collections.Counter(r['ruleId']+':'+str(r.get('level')) for r in new)),'added':[list(k)+[v] for k,v in (nc-oc).items()],'removed':[list(k)+[v] for k,v in (oc-nc).items()]}
(out/'lint-comparison.json').write_text(json.dumps(result,indent=2)+'\n');print(json.dumps(result,indent=2));raise SystemExit(bool(result['added'] or result['removed']))
