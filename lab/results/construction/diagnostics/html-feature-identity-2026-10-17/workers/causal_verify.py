from pathlib import Path
import gzip,json,datetime
out=Path(__file__).resolve().parents[1];a=json.load(gzip.open(out/'before-set-probe.json.gz'));z=json.load(gzip.open(out/'after-set-probe.json.gz'));counts={};changes=[]
for old,new in zip(a['cases'],z['cases']):
 assert (old['expression'],old['enabled'])==(new['expression'],new['enabled']);t=new['results']['tekt'];v=new['results']['vanilla'];assert old['results']['vanilla']==v
 # diagnostic error string, field/argument anchors and traces compared intact;
 # the independently inventoried empty-document warning remains in raw output.
 ts=t['stderr'].removesuffix('warning: ficheiro vazio: sem conteúdo\n\n');assert (t['exit'],t['stdout'],ts)==(v['exit'],v['stdout'],v['stderr']),(new['expression'],new['enabled'],t,v)
 category='syntax-rejection-control' if 'error: expected identifier\n' in v['stderr'] else ('disabled-correct-rejection' if not new['enabled'] and 'because the `' in v['stderr'] else 'language-or-control')
 counts[category]=counts.get(category,0)+1
 if (old['results']['tekt']['exit'],old['results']['tekt']['stdout'],old['results']['tekt']['stderr'])!=(t['exit'],t['stdout'],t['stderr']):changes.append(dict(expression=new['expression'],enabled=new['enabled']))
(out/'causal-verification.json').write_text(json.dumps(dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),source_hashes=z['source_hashes'],binaries=z['binaries'],cases=len(z['cases']),categories=counts,changes=changes),indent=2)+'\n');print('Causal probes',len(z['cases']),counts,len(changes))
