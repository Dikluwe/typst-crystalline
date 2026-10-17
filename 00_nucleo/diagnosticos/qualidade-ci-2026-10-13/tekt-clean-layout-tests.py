from pathlib import Path
import json,re,gzip,hashlib
p=Path('01_core/src/compiler/layout/tests.rs');raw=p.read_bytes();edits={}
with gzip.open('00_nucleo/diagnosticos/qualidade-ci-2026-10-13/before.json.gz','rt') as snapshot_file:
 baseline=json.load(snapshot_file)
assert hashlib.sha256(raw).hexdigest()==baseline['files'][str(p)]['sha256'], 'restore the captured before state before replaying this diagnostic script'
allowed={'unused_imports','unused_variables','unused_mut','clippy::needless_lifetimes','clippy::len_zero','clippy::manual_range_contains','clippy::needless_borrowed_reference','clippy::useless_vec'}
for l in Path('00_nucleo/diagnosticos/correcoes-producao-2026-10-13/clippy-final.jsonl').read_text().splitlines():
 try:r=json.loads(l)
 except:continue
 if r.get('reason')!='compiler-message':continue
 m=r['message'];code=(m.get('code') or {}).get('code')
 if code=='dead_code':
  for span in m['spans']:
   if not span['is_primary'] or span['file_name']!=str(p):continue
   name=raw[span['byte_start']:span['byte_end']].decode()
   if name not in {'ck','heading_with_text','doc_3_equations','lbl'}:continue
   start=raw.rfind(b'\n',0,span['byte_start'])+1
   line=raw[start:raw.find(b'\n',start)]
   indent=line[:len(line)-len(line.lstrip())]
   close=raw.index(b'\n'+indent+b'}\n',span['byte_end'])
   end=close+len(indent)+3
   if raw[end:end+1]==b'\n':end+=1
   edits[(start,end)]=(b'',code)
 if code not in allowed:continue
 for c in m.get('children',[]):
  spans=c.get('spans',[])
  if not spans or not all(s['file_name']==str(p) and s.get('suggestion_applicability')=='MachineApplicable' and s.get('suggested_replacement') is not None for s in spans):continue
  for s in spans:
   start,end=s['byte_start'],s['byte_end'];new=s['suggested_replacement'].encode()
   key=(start,end);assert key not in edits or edits[key][0]==new
   edits[key]=(new,code)
ordered=sorted(edits.items());previous=0
for (start,end),(new,code) in ordered:
 assert start>=previous,(start,end,previous);previous=end
manifest=[{'start':a,'end':b,'old':raw[a:b].decode(),'new':new.decode(),'code':code} for (a,b),(new,code) in ordered]
for (start,end),(new,code) in reversed(ordered):raw=raw[:start]+new+raw[end:]
s=raw.decode()
s=s.replace('use super::*;\n', '#![expect(\n    deprecated,\n    reason = "Exercises permanent L1 pre-shaping FrameItem::Text (layout_types L0 P484)"\n)]\n\nuse super::*;\n',1)
s=s.replace('    #[test]\n    #[test]\n    #[test]\n    fn p1073_ref_table_supplements_en_e_pt()', '    #[test]\n    fn p1073_ref_table_supplements_en_e_pt()')
p.write_text(s)
Path('00_nucleo/diagnosticos/qualidade-ci-2026-10-13/machine-edits.json').write_text(json.dumps(manifest,indent=2))
print('Mechanical suggestions applied:',len(manifest))
p=Path('01_core/src/entities/font_book.rs');s=p.read_text().replace('3..3, 8..2, 12..16','3..3, Range { start: 8, end: 2 }, 12..16').replace('Coverage::from_ranges([0..0x110000])','Coverage::from_ranges(std::iter::once(0..0x110000))').replace('Coverage::from_ranges([u32::MAX - 2..u32::MAX])','Coverage::from_ranges(std::iter::once(u32::MAX - 2..u32::MAX))');p.write_text(s)
