from pathlib import Path
import json,re,gzip,hashlib,datetime
out=Path('00_nucleo/diagnosticos/tekt-frame-items-2026-10-16');scope=json.loads((out/'scope.json').read_text());ctx=json.loads((out/'contexts.json').read_text());lookup={(r['source'],r['line'],r['column']):r for r in ctx};variants=['TextReplacement','TextSeparator','Semantic','Text','TextShaped','Line','Glyph','Image','Shape','Group','Link'];result=[]
def split_top(s,delimiter):
 depth=0;start=0;parts=[]
 for i,c in enumerate(s):
  if c in '{([':depth+=1
  elif c in '})]':depth-=1
  elif c==delimiter and depth==0:parts.append(s[start:i]);start=i+1
 parts.append(s[start:]);return parts

def variants_covered(pattern):
 if re.search(r'\bif\b',pattern):return [],['guard']
 covered=[];restrictions=[]
 for part in split_top(pattern,'|'):
  m=re.fullmatch(r'\s*((?:[A-Za-z_]\w*::)*(?:FrameItem|FI))::([A-Z]\w*)\s*\{(.*)\}\s*',part,re.S)
  assert m,('unexpected pattern',pattern,part)
  v=m[2];assert v in variants
  fields=split_top(m[3],',');full=True
  for field in fields:
   f=field.strip()
   if not f or f=='..':continue
   rhs=f.partition(':')[2].strip() if ':' in f else f
   rhs=re.sub(r'^(?:ref\s+mut\s+|ref\s+|mut\s+)','',rhs)
   if rhs not in ['true','false'] and re.fullmatch(r'(?:r#)?[a-z_]\w*|_',rhs):continue
   full=False;restrictions.append(f'{v}: {f}')
  if full:covered.append(v)
 return covered,restrictions
for basic in scope['rows']:
 r=lookup[(basic['source'],basic['line'],basic['column'])];assert r['pattern_source'] in ['_','_other'];covered=set();restricted=[];found=False
 for arm in r['arms']:
  if arm['line']==r['line']:found=True;break
  vs,rs=variants_covered(arm['pattern']);covered.update(vs);restricted.extend(rs)
 assert found
 missing=[v for v in variants if v not in covered];prefix=re.search(r'\b((?:[A-Za-z_]\w*::)*(?:FrameItem|FI))::',' '.join(a['pattern'] for a in r['arms']))[1]
 new=' | '.join(prefix+'::'+v+' { .. }' for v in missing) if missing else None
 if not missing:assert r['source']=='01_core/src/entities/layout_types.rs' and r['function']=='visit_item'
 result.append(dict(**basic,covered=[v for v in variants if v in covered],complement=missing,restricted_or_guarded=restricted,prefix=prefix,new_pattern=new,action='nominalize' if missing else 'remove-dead-fallback',arm_source=r['arm_source'],arm_start_byte=r['arm_start_byte'],arm_end_byte=r['arm_end_byte'],pattern_start_byte=r['pattern_start_byte'],pattern_end_byte=r['pattern_end_byte'],pattern_source=r['pattern_source']))
assert len(result)==70 and sum(r['action']=='remove-dead-fallback' for r in result)==1
(out/'classification.json').write_text(json.dumps(result,ensure_ascii=False,indent=2)+'\n')
for r in result:print(f"{r['id']} {r['source']}:{r['line']} {r['function']}: "+(','.join(r['complement']) if r['complement'] else 'REMOVE DEAD')+(' [guard/restricted: '+ '; '.join(r['restricted_or_guarded'])+']' if r['restricted_or_guarded'] else ''))
