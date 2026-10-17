from pathlib import Path
import json
T=Path('/tmp/tekt-align-table/qa');H=Path('00_nucleo/diagnosticos/align-tabela-2026-10-06');j=json.loads((H/'raster.json').read_text())
for name,c in j['cases'].items():
 sets=[]
 for engine in ['after','vanilla']:
  b=(T/(name+'-'+engine+'-raster.pgm')).read_bytes().split(b'\n',3);w,h=map(int,b[1].split());sets.append({(i%w,i//w) for i,p in enumerate(b[3]) if p<180})
 a,b=sets
 def unmatched(a,b):return sum(not any((x+dx,y+dy) in b for dx in [-1,0,1] for dy in [-1,0,1]) for x,y in a)
 c['ink_mask_unmatched_after_within_1px']=unmatched(a,b);c['ink_mask_unmatched_vanilla_within_1px']=unmatched(b,a);print(name,c['ink_mask_unmatched_after_within_1px'],c['ink_mask_unmatched_vanilla_within_1px'])
(H/'raster.json').write_text(json.dumps(j,indent=2)+'\n');(H/'raster-score.py').write_text(Path(__file__).read_text())
