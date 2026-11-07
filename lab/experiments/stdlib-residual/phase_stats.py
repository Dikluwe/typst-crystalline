"""Inclusive L3 probe totals by publication; never sum overlapping scopes."""
import argparse,csv,statistics as s
from pathlib import Path
from collections import defaultdict
p=argparse.ArgumentParser();p.add_argument('directory',type=Path);p.add_argument('output',type=Path);a=p.parse_args()
g=defaultdict(list);rows=list(csv.DictReader((a.directory/'measurements.csv').open()))
for r in rows:
 if not r['variant'].startswith('on'):continue
 cycle={'cli':0,'startup':0,'same_bytes':1,'main_edit':2}[r['mode']]
 path=a.directory/r['case']/'phases.tsv';assert path.exists(),r
 for line in path.read_text().splitlines():
  pid,c,label,count,ns=line.split('\t')
  if pid==r['process_id'] and int(c)==cycle:g[r['variant'],r['fixture'],r['mode'],label].append((int(count),int(ns)))
out=[]
for (v,f,m,label),values in sorted(g.items()):
 out.append(dict(variant=v,fixture=f,mode=m,label=label,samples=len(values),median_calls=s.median(c for c,n in values),median_ms=s.median(n for c,n in values)/1e6))
with a.output.open('x',newline='') as handle:
 w=csv.DictWriter(handle,fieldnames=list(out[0]));w.writeheader();w.writerows(out)
print('Inclusive phase rows',len(out))
