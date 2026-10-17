"""Summarize paired ratios and round signs without interpreting instrumented time."""
import gzip,json,statistics,hashlib,datetime
from pathlib import Path
D=Path(__file__).resolve().parent
result={'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'receipts':{}}
for name in ['measurements-equal-argv','measurements-cost-controls','measurements-repeat','measurements-risk-repeat']:
 p=D/(name+'.json.gz')
 if not p.exists():continue
 d=json.load(gzip.open(p,'rt'));summary={}
 for case,c in d['summary'].items():
  ratios=[]
  for row in d['samples']:
   if row['fixture']!=case or row['engine']!='after':continue
   old=next(x for x in d['samples'] if x['fixture']==case and x['engine']=='before' and x['round']==row['round'] and x['pair']==row['pair'])
   ratios.append({'round':row['round'],'time':100*(row['wall_ms']/old['wall_ms']-1),'rss':100*(row['peak_rss_kib']/old['peak_rss_kib']-1)})
  summary[case]={'time_percent':statistics.median(x['time'] for x in ratios),'rss_percent':statistics.median(x['rss'] for x in ratios),'time_round_percent':[statistics.median(x['time'] for x in ratios if x['round']==r) for r in range(3)],'rss_round_percent':[statistics.median(x['rss'] for x in ratios if x['round']==r) for r in range(3)],'before_rss_kib':c['before']['rss_median_kib'],'after_rss_kib':c['after']['rss_median_kib']}
 result['receipts'][name]={'sha256':hashlib.sha256(p.read_bytes()).hexdigest(),'head':d['head'],'utc_start':d['utc_start'],'utc_end':d['utc_end'],'binaries':d['binaries'],'summary':summary}
 print(name)
 for case,c in summary.items():print(case, {k:[round(v,3) for v in x] if isinstance(x,list) else round(x,3) for k,x in c.items()})
(D/'summary.json').write_text(json.dumps(result,indent=2)+'\n')
