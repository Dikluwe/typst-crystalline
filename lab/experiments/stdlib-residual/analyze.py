"""Summarize paired rounds; absolute medians need not equal median pair delta."""
import argparse,csv,random,statistics as s
from collections import defaultdict
from pathlib import Path
p=argparse.ArgumentParser();p.add_argument('csv',type=Path);p.add_argument('output',type=Path);p.add_argument('--reference',default='bas');a=p.parse_args()
g=defaultdict(list)
for r in csv.DictReader(a.csv.open()):
 for metric in ['wall_ns','cpu_ns']:
  g[r['variant'],r['fixture'],r['mode'],metric,int(r['round'])].append(int(r[metric]))
v={k:s.mean(x) for k,x in g.items()}; rows=[]
for variant,fixture,mode,metric in sorted({k[:4] for k in v}):
 if variant==a.reference:continue
 ids=sorted(k[4] for k in v if k[:4]==(variant,fixture,mode,metric))
 if not all((a.reference,fixture,mode,metric,i) in v for i in ids):continue
 b=[v[a.reference,fixture,mode,metric,i] for i in ids];c=[v[variant,fixture,mode,metric,i] for i in ids]
 delta=[100*(y/x-1) for x,y in zip(b,c)]; rng=random.Random(0)
 boots=sorted(s.median(rng.choices(delta,k=len(delta))) for _ in range(10000))
 rows.append(dict(reference=a.reference,variant=variant,fixture=fixture,mode=mode,metric=metric,rounds=len(ids),reference_ms=s.median(b)/1e6,variant_ms=s.median(c)/1e6,paired_percent=s.median(delta),ci_low=boots[249],ci_high=boots[9749]))
with a.output.open('x',newline='') as f:
 w=csv.DictWriter(f,fieldnames=list(rows[0]));w.writeheader();w.writerows(rows)
for r in rows:
 if r['fixture']=='imports_00.typ' and r['mode'] in ['cli','startup'] and r['metric']=='wall_ns':print(r)
