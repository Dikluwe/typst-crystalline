from pathlib import Path
import json
T=Path('/tmp/tekt-bidi-materializacao');H=Path('00_nucleo/diagnosticos/bidi-contexto-implementacao-2026-10-05');j=json.loads((H/'raster.json').read_text())
for name,c in j['cases'].items():
 sets=[]
 for engine in ['after','vanilla']:
  b=(T/(name+'-'+engine+'-raster.pgm')).read_bytes().split(b'\n',3);w,h=map(int,b[1].split());sets.append({(i%w,i//w) for i,p in enumerate(b[3]) if p<180})
 a,b=sets
 def unmatched(a,b):return sum(not any((x+dx,y+dy) in b for dx in [-1,0,1] for dy in [-1,0,1]) for x,y in a)
 c['ink_mask_unmatched_after_within_1px']=unmatched(a,b);c['ink_mask_unmatched_vanilla_within_1px']=unmatched(b,a);print(name,c['ink_mask_unmatched_after_within_1px'],c['ink_mask_unmatched_vanilla_within_1px'])
j['visual_inspection']='Cropped complete ink on the single relevant page inspected for all four bilateral probes. Literal terminal space and authored bidi controls have matching visible text/positions; emph Arabic renders incompatible symbols before and after, while vanilla renders regular Arabic.'
(H/'raster.json').write_text(json.dumps(j,indent=2)+'\n');(H/'raster-score.py').write_text(Path(__file__).read_text())
