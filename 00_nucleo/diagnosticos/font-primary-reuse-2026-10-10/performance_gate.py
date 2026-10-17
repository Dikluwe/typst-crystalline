"""Evaluate the thresholds declared before paired timing."""
from pathlib import Path
import gzip,json,statistics,datetime
D=Path(__file__).resolve().parent;d=json.load(gzip.open(D/'measurements.json.gz','rt'));a=json.load(gzip.open(D/'allocation-green.json.gz','rt'));cases={}
for n,c in d['summary'].items():
 rss_round=[]
 for r in range(3):
  ratios=[]
  for p in range(4):
   rows={s['engine']:s for s in d['samples'] if s['fixture']==n and s['round']==r and s['pair']==p}
   ratios.append(rows['after']['peak_rss_kib']/rows['before']['peak_rss_kib']-1)
  rss_round.append(statistics.median(ratios))
 cases[n]={'time_paired':c['after_before']['wall_ratio_median'],'time_rounds':c['after_before']['wall_round_median'],'rss_paired':c['after_before']['rss_ratio_median'],'rss_rounds':rss_round}
regressions=[n for n,c in cases.items() if (c['time_paired']>=0.02 and all(x>0 for x in c['time_rounds'])) or (c['rss_paired']>=0.03 and all(x>0 for x in c['rss_rounds']))]
c=cases['local-set-1600'];passed=a['passed'] and c['time_paired']<=-0.03 and all(x<0 for x in c['time_rounds']) and not regressions
state={'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'criterion':json.loads((D/'acceptance.json').read_text()),'cases':cases,'regressions':regressions,'allocations_passed':a['passed'],'passed':passed}
(D/'performance-gate.json').write_text(json.dumps(state,indent=2)+'\n');print(passed,regressions)
assert passed,state
