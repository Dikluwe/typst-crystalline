from pathlib import Path
import json,subprocess,gzip,collections
out=Path(__file__).resolve().parents[1];b=json.load(gzip.open(out/'before.json.gz'));recipe=json.load(open(out/'audit-recipe.json'));cmd=recipe['command'][:-3]+[str(out/'workers/latent.rs'),'-o','/tmp/tekt-math-layout-latent'];subprocess.run(cmd,check=True);rows=[]
for source in b['sources']:
 r=subprocess.run(['/tmp/tekt-math-layout-latent',source],capture_output=True,text=True);assert r.returncode==0,r.stderr
 for row in json.loads(r.stdout):row['source']=source;rows.append(row)
(out/'wildcard-inventory.json').write_text(json.dumps(rows,indent=2)+'\n');emitted={(r['source'],r['line']) for r in json.load(open(out/'contexts.json'))};latent=[r for r in rows if (r['source'],r['line']) not in emitted];(out/'latent.json').write_text(json.dumps(latent,indent=2)+'\n')
with (out/'all-contexts.json').open('w') as f:subprocess.run(['/tmp/tekt-math-layout-audit','.',str(out/'input.sarif.json'),str(out/'latent.json')],stdout=f,check=True)
print(json.dumps({'emitted':len(emitted),'latent':len(latent),'families':dict(collections.Counter(r['enum'] for r in rows)),'latent_locations':[(r['source'],r['line'],r['enum']) for r in latent]}),flush=True)
