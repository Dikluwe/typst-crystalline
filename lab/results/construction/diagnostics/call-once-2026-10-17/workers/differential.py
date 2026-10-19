from pathlib import Path
import json,gzip,collections,re,datetime,copy
out=Path(__file__).resolve().parents[1];prior=out.parent/'html-feature-identity-2026-10-17';b=json.load(gzip.open(out/'before.json.gz'));assert json.load(gzip.open(prior/'final-state.json.gz'))['source_hashes']==b['hashes'];old=json.load(open(prior/'validation-architecture.log'))['runs'][0]['results'];new=json.load(open(out/'validation-architecture.log'))['runs'][0]['results']
def uri(r):return r['locations'][0]['physicalLocation']['artifactLocation']['uri'].removeprefix('./')
def key(r):return uri(r),r['ruleId'],r['level'],r['message']['text']
a=collections.Counter(map(key,old));z=collections.Counter(map(key,new));removed=list((a-z).elements());added=list((z-a).elements());(out/'differential-review.json').write_text(json.dumps(dict(removed=removed,added=added),indent=2)+'\n');updates=[]
# Existing diagnostics retain severity, message and source; only documented
# changes to a V19 pattern may update that note's text.
pattern_re=r'Alternativas escritas nos grupos: (\d+); guard: ([^;]+); padrão: `([\s\S]*)`\.'
for oldkey in removed:
 p,rule,level,text=oldkey;assert rule=='V19' and level=='note',oldkey
 candidates=[k for k in added if k[:3]==oldkey[:3]];a0=re.search(pattern_re,text);assert a0
 found=[]
 for k in candidates:
  z0=re.search(pattern_re,k[3])
  if not z0 or a0[2]!=z0[2] or text[:a0.start()]!=k[3][:z0.start()]:continue
  norm=lambda s:re.sub(r'\s+','',s)
  if p.endswith('/field_access/fields.rs'):
   if int(z0[1])!=int(a0[1])+1:continue
   stripped=re.sub(r'\|\s*Value::Type\(\s*_\s*\)\s*','',z0[3]);ok=norm(stripped)==norm(a0[3]);reason='Add Type to field-only span policy; old cases retained, Type::Array anchor tested against pin'
  elif p.endswith('/bindings/method_dispatch.rs'):
   if int(z0[1])!=int(a0[1]):continue
   clean=re.sub(r'^\s*target\s*@\s*\((.*)\)\s*$',r'\1',z0[3],flags=re.S);ok=norm(clean)==norm(a0[3]);reason='Bind and transfer the same five fallback variants, preserving evaluated Args'
  else:continue
  if ok:found.append((k,reason))
 assert len(found)==1,(oldkey,found,candidates);k,reason=found[0];added.remove(k);updates.append(dict(source=p,reason=reason,old_message=text,new_message=k[3]))
notes=[]
for k in added:
 p,rule,level,msg=k;assert rule=='V19' and level=='note' and p.endswith('/call_dispatch/invoke.rs'),k
 m=re.search(pattern_re,msg);assert m and int(m[1])==37;names=re.findall(r'Value::(\w+)',m[3]);assert len(names)==len(set(names))==37 and 'Func' not in names and 'Type' not in names
 notes.append(dict(source=p,message=msg,reason='All non-callable Value variants reject before Args; both callable paths remain separate',tests=['call_once_callable_classification_covers_all_value_variants_before_arguments','call_once_unhandled_keeps_every_value_and_does_not_evaluate_arguments']))
count=lambda rows:dict(collections.Counter(r['ruleId']+':'+r['level'] for r in rows));result=dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),baseline_source_hashes_verified=True,before=count(old),after=count(new),all_existing_errors_and_warnings_preserved=True,other_notes_preserved_by_source_severity_message=True,new_errors_or_warnings=False,updated_v19=updates,added_v19=notes);(out/'architecture-delta.json').write_text(json.dumps(result,indent=2)+'\n');print(result['after'],'updated V19',len(updates),'added V19',len(notes))
