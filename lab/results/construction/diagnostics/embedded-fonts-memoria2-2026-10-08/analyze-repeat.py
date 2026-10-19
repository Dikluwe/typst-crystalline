from pathlib import Path
import gzip,json,statistics
D=Path(__file__).resolve().parent;b=json.loads(gzip.open(D/'repeat.json.gz','rt').read());samples=b['samples'];result={}
for case in sorted(set(x['fixture'] for x in samples)):
 r={};s=[x for x in samples if x['fixture']==case];pairs=[{x['engine']:x for x in s if x['round']==rd and x['pair']==p} for rd in range(3) for p in range(6)]
 for metric in ['wall_ms','max_rss_kib']:
  r[metric]={e:statistics.median(x[metric] for x in s if x['engine']==e) for e in ['baseline','current','vanilla']}
  for label,a,z in [('current_baseline','current','baseline'),('current_vanilla','current','vanilla')]:
   r[metric][label]={'paired_percent':statistics.median((p[a][metric]/p[z][metric]-1)*100 for p in pairs),'blocks_percent':[statistics.median((p[a][metric]/p[z][metric]-1)*100 for p in pairs[rd*6:rd*6+6]) for rd in range(3)]}
 result[case]=r
print(json.dumps({'started':b['started'],'finished':b['finished'],'cases':result},indent=2));(D/'repeat-summary.json').write_text(json.dumps(result,indent=2)+'\n')
