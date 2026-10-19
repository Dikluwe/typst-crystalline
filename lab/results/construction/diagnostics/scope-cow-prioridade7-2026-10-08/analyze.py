from pathlib import Path
import gzip,json,statistics
D=Path(__file__).resolve().parent;r=json.load(gzip.open(D/'bench.json.gz','rt'));summary={}
for name in sorted({x['fixture'] for x in r['samples']}):
 rows=[x for x in r['samples'] if x['fixture']==name];out={}
 for measure in ['wall_ms','layout_ms','total_ms','max_rss_kib']:
  val=lambda x:x['timings'][measure] if measure in ['layout_ms','total_ms'] else x[measure]
  pairs=[];rounds=[]
  for round in range(3):
   ratios=[]
   for pair in range(6):
    values={x['engine']:val(x) for x in rows if x['round']==round and x['pair']==pair};ratio=values['current']/values['previous'];pairs.append(ratio);ratios.append(ratio)
   rounds.append(statistics.median(ratios))
  out[measure]={'previous_median':statistics.median(val(x) for x in rows if x['engine']=='previous'),'current_median':statistics.median(val(x) for x in rows if x['engine']=='current'),'median_paired_ratio':statistics.median(pairs),'round_paired_ratios':rounds,'current_lower_pairs':sum(v<1 for v in pairs),'pairs':18}
 summary[name]=out
(D/'summary.json').write_text(json.dumps(summary,indent=2)+'\n');print(json.dumps(summary,indent=2))
