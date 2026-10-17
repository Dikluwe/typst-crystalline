from pathlib import Path
import json,gzip,datetime
out=Path('00_nucleo/diagnosticos/place-origem-local-2026-10-14')
def load(name):
 with gzip.open(out/name/'results.json.gz','rt') as f:return json.load(f)
def pairs(name):
 rows=load(name)['rows'];result={}
 for row in rows:result.setdefault(row['case'],{})[row['binary']]=row
 return result
r={'utc':datetime.datetime.now(datetime.timezone.utc).isoformat()}
for name in ['before','after','baseline-expanded','candidate-expanded','final-expanded','pdf-final']:
 ps=pairs(name);successful={k:v for k,v in ps.items() if (all(c['exit']==0 for row in v.values() for c in row['commands']) if name=='pdf-final' else all(row['exit']==0 for row in v.values()))}
 r[name]={'cases':len(ps),'successful_pairs':len(successful),'bbox_matches':sum(v['tekt']['blue_bbox']==v['vanilla']['blue_bbox'] for v in successful.values()),'bbox_differences':{k:{who:v[who]['blue_bbox'] for who in v} for k,v in successful.items() if v['tekt']['blue_bbox']!=v['vanilla']['blue_bbox']},'same_pixels':sum(v['tekt']['pixels_sha256']==v['vanilla']['pixels_sha256'] for v in successful.values())}
# A bbox match is geometry evidence for the colored marker, not whole-document parity.
r['note']='Primary before/after markers use exactly 4px/pt. Initial align-controls used librsvg 96dpi and zoom4: its printed bbox values must be multiplied by 3/4 to obtain pt; expanded controls and PDFs explicitly use 4px/pt. Cases with no marker are not evidence of marker geometry.'
(out/'comparison.json').write_text(json.dumps(r,indent=2));print(json.dumps(r,indent=2))
