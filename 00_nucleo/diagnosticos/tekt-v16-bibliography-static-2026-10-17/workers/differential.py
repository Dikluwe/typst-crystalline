from pathlib import Path
import json,collections,datetime,re,subprocess
out=Path(__file__).resolve().parents[1];before=json.load(open('00_nucleo/diagnosticos/tekt-v16-introspect-carriers-2026-10-17/validation-architecture.log'));after=json.load(open(out/'validation-architecture.log'))
def keys(d):
 return collections.Counter((r['locations'][0]['physicalLocation']['artifactLocation']['uri'].removeprefix('./'),r['ruleId'],r['level'],r['message']['text']) for r in d['runs'][0]['results'])
a,z=keys(before),keys(after);removed=list((a-z).elements());added=list((z-a).elements());counts=lambda d:dict(sorted(collections.Counter(r['ruleId']+':'+r['level'] for r in d['runs'][0]['results']).items()));record=dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),before=counts(before),after=counts(after),removed=removed,added=added);(out/'architecture-delta.json').write_text(json.dumps(record,indent=2)+'\n')
assert len(removed)==2 and all(r[0]=='01_core/src/compiler/introspect/bibliography.rs' for r in removed)
assert sorted(r[1:3] for r in removed)==[('V16','error'),('V19','note')]
assert all('Content::ContextBlock(_) | Content::Dynamic(_)' in r[3] for r in removed if r[1]=='V19')
assert len(added)==4 and all(r[:3]==('01_core/src/entities/content/traversal.rs','V19','note') for r in added)
canon=lambda p:re.sub(r'\s+','',p);asts=json.loads(subprocess.check_output(['/tmp/tekt-math-layout-latent','01_core/src/entities/content/traversal.rs','matches'],text=True));expected={canon(a['pattern']) for a in asts[0]['arms'] if '|' in a['pattern']};actual={canon(r[3].split('padrão: `',1)[1].rsplit('`',1)[0]) for r in added};assert actual==expected and len(expected)==4
assert record['after']=={'V16:error':106,'V19:note':1099,'V20:note':873};print(record['before'],'->',record['after'])
