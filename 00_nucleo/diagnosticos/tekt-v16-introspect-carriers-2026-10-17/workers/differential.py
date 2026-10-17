from pathlib import Path
import json,gzip,collections,re,datetime
out=Path(__file__).resolve().parents[1];prior=out.parent/'tekt-v16-content-views-2026-10-17';b=json.load(gzip.open(out/'before.json.gz'));assert json.load(gzip.open(prior/'final-state.json.gz'))['source_hashes']==b['hashes'];old=json.load(open(prior/'validation-architecture.log'))['runs'][0]['results'];new=json.load(open(out/'validation-architecture.log'))['runs'][0]['results'];plan=json.load(open(out/'projection-plan.json'))
def key(r):return r['locations'][0]['physicalLocation']['artifactLocation']['uri'].removeprefix('./'),r['ruleId'],r['level'],r['message']['text']
a=collections.Counter(map(key,old));z=collections.Counter(map(key,new));removed=list((a-z).elements());added=list((z-a).elements());(out/'differential-review.json').write_text(json.dumps(dict(removed=removed,added=added),indent=2)+'\n');assert len(removed)==len(added)==2,(removed,added);assert {k[0] for k in removed}=={t['source'] for t in plan['targets']};assert all(k[1:3]==('V16','error') for k in removed);canonical=lambda p:re.sub(r',\)',')',re.sub(r'\s+','',p));notes=[]
for t in plan['targets']:
 matches=[]
 for k in added:
  if k[0]!=t['source'] or k[1:3]!=('V19','note'):continue
  m=re.search(r'Alternativas escritas nos grupos: (\d+); guard: ausente; padrão: `([\s\S]*)`\.',k[3])
  if m and int(m[1])==t['alternatives'] and canonical(m[2])==canonical(t['replacement']):matches.append(k)
 assert len(matches)==1,(t['function'],matches,added);k=matches[0];added.remove(k);notes.append(dict(source=t['source'],function=t['function'],alternatives=t['alternatives'],message=k[3],reason='L0-prescribed absence cases; Source resolved separately; symbol controls include all Content variants, guard refusal and all payload bool combinations; active bodies preserved by replay'))
assert not added;count=lambda rs:dict(collections.Counter(r['ruleId']+':'+r['level'] for r in rs));r=dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),before=count(old),after=count(new),removed=removed,new_v19=notes,bibliography_v16_retained=True,other_diagnostics_preserved_by_source_rule_severity_message=True,new_errors_or_warnings=False);(out/'architecture-delta.json').write_text(json.dumps(r,indent=2)+'\n');print(r['before'],'->',r['after'])
