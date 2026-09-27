#!/usr/bin/env python3
"""Analyze admitted Criterion samples without mixing operation boundaries."""
import json,math,pathlib,statistics
from collections import defaultdict
P=pathlib.Path(__file__).resolve().parent
def distribution(values):
 v=sorted(values);n=len(v)
 def q(frac):
  at=(n-1)*frac;lo=math.floor(at);hi=math.ceil(at);return v[lo]+(v[hi]-v[lo])*(at-lo)
 return {'n':n,'mean_ns':statistics.mean(v),'median_ns':statistics.median(v),'sd_ns':statistics.stdev(v) if n>1 else 0,'min_ns':v[0],'p05_ns':q(.05),'p95_ns':q(.95),'max_ns':v[-1]}
def main():
 by=defaultdict(list);raw=[]
 for directory in sorted((P/'rounds').glob('r*')):
  receipt=json.loads((directory/'receipt.json').read_text());assert receipt['valid']
  for f in (directory/'criterion').rglob('new/sample.json'):
   sample=json.loads(f.read_text());key=json.loads((f.parent/'benchmark.json').read_text())['full_id']
   values=[t/i for t,i in zip(sample['times'],sample['iters'])];assert len(values)==30
   row={'round':directory.name,'case':key,'distribution':distribution(values),'raw_sample':str(f.relative_to(P)),'criterion_estimates':json.loads((f.parent/'estimates.json').read_text())};by[key].append(row);raw.append(row)
 summary={}
 for key,rows in sorted(by.items()):
  assert len(rows)==3
  means=[r['distribution']['mean_ns'] for r in rows];mean=statistics.mean(means);med=statistics.median(means)
  pct=100*abs(mean-med)/med;spread=100*(max(means)-min(means))/med
  summary[key]={'rounds':rows,'mean_of_round_means_ns':mean,'median_of_round_means_ns':med,'round_mean_median_difference_pct':pct,'round_range_pct':spread,'stable_absolute_cost':pct<3 and spread<=5,'note':'Each distribution uses per-sample time/iteration; samples have unequal iteration counts. Not an interleaved paired population.'}
 ratios=[]
 for key,rows in summary.items():
  if key.endswith('/constructor/current_public'):
   font=key.removesuffix('/constructor/current_public');old=summary[font+'/constructor/historical_extracted']
   ratios.append({'font':font,'historical_over_current_per_round':[{ 'round':a['round'],'ratio_of_sample_mean_ns':b['distribution']['mean_ns']/a['distribution']['mean_ns']} for a,b in zip(rows['rounds'],old['rounds'])],'both_absolute_costs_stable':rows['stable_absolute_cost'] and old['stable_absolute_cost'],'historical_comparator':'Exact extracted historical Coverage algorithm, not full historical crate or vanilla.'})
 (P/'summary.json').write_text(json.dumps({'cases':summary,'constructor_comparisons':ratios,'all_rounds_retained':True},indent=2)+'\n')
 print('Analyzed',len(summary),'cases; stable absolute-cost cases:',sum(r['stable_absolute_cost'] for r in summary.values()))
if __name__=='__main__':main()
