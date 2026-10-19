"""Evaluate the predeclared improvement criterion, including confirmation."""
from pathlib import Path
import gzip,json,statistics,sys,datetime
D=Path(__file__).resolve().parent
label=sys.argv[1];assert label in ('red','green')
ds=[json.load(gzip.open(D/p,'rt')) for p in ['measurements.json.gz','confirmation.json.gz']]
cases={}
for name in ds[0]['summary']:
 ratios=[];rounds=[]
 for d in ds:
  if name not in d['summary']:continue
  for r in range(3):
   rr=[]
   for p in range(4):
    rows={s['engine']:s for s in d['samples'] if s['fixture']==name and s['round']==r and s['pair']==p}
    value=0.0 if label=='red' else rows['after']['wall_ms']/rows['before']['wall_ms']-1
    ratios.append(value);rr.append(value)
   rounds.append(statistics.median(rr))
 cases[name]={'paired_median':statistics.median(ratios),'round_medians':rounds,'pairs':len(ratios)}
benefit=[n for n,c in cases.items() if c['paired_median']<=-0.01 and all(r<0 for r in c['round_medians'])]
regression=[n for n,c in cases.items() if c['paired_median']>=0.01 and all(r>0 for r in c['round_medians'])]
alloc=json.load(gzip.open(D/'allocation-green.json.gz','rt'))
passed=bool(benefit) and not regression and alloc['passed']
result={'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'label':label,'criterion':json.loads((D/'acceptance.json').read_text()),'baseline_interpretation':'RED evaluates baseline against itself; no improvement expected','receipts':['measurements.json.gz','confirmation.json.gz','allocation-green.json.gz'],'cases':cases,'benefit':benefit,'regression':regression,'passed':passed}
(D/f'performance-{label}.json').write_text(json.dumps(result,indent=2)+'\n')
print(json.dumps(result,indent=2))
sys.exit(0 if passed else 1)
