from pathlib import Path
import gzip,json,statistics
D=Path(__file__).resolve().parent;b=json.loads(gzip.open(D/'bench.json.gz','rt').read());samples=b['samples'];result={}
for case in sorted(set(x['fixture'] for x in samples)):
 s=[x for x in samples if x['fixture']==case];pairs=[{x['engine']:x for x in s if x['round']==rd and x['pair']==p} for rd in range(3) for p in range(4)];r={}
 for metric in ['wall_ms','max_rss_kib']:
  r[metric]={e:statistics.median(x[metric] for x in s if x['engine']==e) for e in ['baseline','current']};r[metric]['paired_percent']=statistics.median((p['current'][metric]/p['baseline'][metric]-1)*100 for p in pairs);r[metric]['blocks_percent']=[statistics.median((p['current'][metric]/p['baseline'][metric]-1)*100 for p in pairs[rd*4:rd*4+4]) for rd in range(3)]
 result[case]=r
(D/'summary.json').write_text(json.dumps(result,indent=2)+'\n');print(json.dumps(result,indent=2))
