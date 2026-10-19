from pathlib import Path
import json,gzip,hashlib,subprocess,re
out=Path(__file__).resolve().parents[1];b=json.load(gzip.open(out/'before.json.gz'));sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest();assert all(sha(p)==h for p,h in b['hashes'].items());recipe=json.load(open(out/'audit-recipe.json'));assert all(sha(v['path'])==v['sha256'] for v in recipe['externs'].values());subprocess.run(recipe['command'],check=True);latent=[];all_matches={}
for source in b['sources']:
 nodes=json.loads(subprocess.check_output(['/tmp/tekt-whitespace-latent',source,'matches'],text=True));all_matches[source]=nodes
 for n in nodes:
  arms=n['arms'];patterns=[a['pattern'] for a in arms]
  for a in arms:
   if a['pattern']=='_' and patterns and any(p.startswith(('Content::','Value::','crate::entities::func::FuncRepr::')) for p in patterns):
    emitted=[r['locations'][0]['physicalLocation'] for r in json.load(open(out/'input.sarif.json'))['runs'][0]['results']]
    if any(l['artifactLocation']['uri'].removeprefix('./')==source and l['region']['startLine']==a['line'] for l in emitted):continue
    latent.append(dict(source=source,line=a['line'],reason='Nominal enum fallback not emitted; audit body/role before deciding.',ids=['typed-latent']))
   if a['pattern']=='_' and any(p.startswith('Some(Value::Content(Content::ShowRecipe(') for p in patterns):latent.append(dict(source=source,line=a['line'],reason='Optional snapshot projection; audited separately without deep nominal nesting.',ids=['optional-recipe']))
(out/'latent.json').write_text(json.dumps(latent,indent=2)+'\n');(out/'all-matches-before.json').write_text(json.dumps(all_matches,indent=2)+'\n')
with (out/'contexts.json').open('w') as f:subprocess.run(['/tmp/tekt-table-cells-audit','.',str(out/'input.sarif.json'),str(out/'latent.json')],stdout=f,check=True)
print({'emitted':10,'latent':len(latent)})
