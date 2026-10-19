from pathlib import Path
import json,gzip
out=Path(__file__).resolve().parents[1];cases=json.load(gzip.open(out/'before-set-probe.json.gz'))['cases'];failures=[]
for c in cases:
 t=c['results']['tekt'];v=c['results']['vanilla']
 if t['exit']!=v['exit'] or t['stderr'].splitlines()[:1]!=v['stderr'].splitlines()[:1] and v['exit']:
  failures.append(dict(expression=c['expression'],enabled=c['enabled']))
(out/'causal-red-result.json').write_text(json.dumps(dict(assertion='Gate/target error precedes arguments; HTML alias set validates arguments',failures=failures),indent=2)+'\n');assert not failures,failures
