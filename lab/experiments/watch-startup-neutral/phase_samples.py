"""Validate real probe traces, then summarize inclusive operations by round."""
import argparse,csv,importlib.util,statistics as s
from pathlib import Path
from collections import defaultdict
p=argparse.ArgumentParser();p.add_argument('directory',type=Path);p.add_argument('samples',type=Path);p.add_argument('summary',type=Path);a=p.parse_args()
spec=importlib.util.spec_from_file_location('independent_evidence',Path(__file__).with_name('tests')/'check_evidence.py');checker=importlib.util.module_from_spec(spec);spec.loader.exec_module(checker)
rows=list(csv.DictReader((a.directory/'measurements.csv').open()));by_case=defaultdict(list)
for r in rows:by_case[r['case']].append(r)
output=[];labels=sorted(checker.TIMERS|checker.EVENTS)
for case,records in by_case.items():
 path=a.directory/case/'phases.tsv'
 if not records[0]['variant'].endswith('1'):
  assert not path.exists(),case
  continue
 groups=checker.parse_phases(path.read_text());expected=set()
 for r in records:
  cycle={'cli':0,'startup':0,'same_bytes':1,'main_edit':2}[r['mode']];key=int(r['process_id']),cycle;expected.add(key);assert key in groups,(case,key)
  for label in labels:
   calls,ns=groups[key].get(label,(0,0));output.append(dict(round=r['round'],repeat=r['repeat'],variant={'b':'bas','n':'neu','l':'laz'}[r['variant'][0]],probe_variant=r['variant'],fixture=r['fixture'],mode=r['mode'],process_id=r['process_id'],cycle=cycle,label=label,calls=calls,ns=ns))
 assert set(groups)==expected,(case,'surplus/missing cycles')
with a.samples.open('x',newline='') as f:
 w=csv.DictWriter(f,fieldnames=list(output[0]));w.writeheader();w.writerows(output)
g=defaultdict(list)
for r in output:g[r['variant'],r['fixture'],r['mode'],r['label'],r['round']].append((r['calls'],r['ns']))
blocks=defaultdict(list)
for k,values in g.items():blocks[k[:4]].append((s.mean(x for x,y in values),s.mean(y for x,y in values)))
summary=[dict(variant=k[0],fixture=k[1],mode=k[2],label=k[3],rounds=len(v),median_calls=s.median(x for x,y in v),median_round_mean_ms=s.median(y for x,y in v)/1e6) for k,v in sorted(blocks.items())]
with a.summary.open('x',newline='') as f:
 w=csv.DictWriter(f,fieldnames=list(summary[0]));w.writeheader();w.writerows(summary)
print('Validated',len(by_case),'processes;',len(output),'phase samples; missing labels represented as zero')
