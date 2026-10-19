from pathlib import Path
import json,subprocess,gzip,datetime,hashlib
out=Path('00_nucleo/diagnosticos/paint-root-2026-10-14');owners=json.loads((out/'owners.json').read_text());paths=list(owners)+['benches/stream_bench.rs']
# Snapshot API RED, including fixture errors, before migration.
with gzip.open(out/'red-api-state.json.gz','wt') as f:json.dump(dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),exit=101,files={p:dict(text=Path(p).read_text(),sha256=hashlib.sha256(Path(p).read_bytes()).hexdigest()) for p in paths}),f)
rows=[json.loads(s) for s in subprocess.check_output(['/tmp/tekt-group-syntax/target/debug/tekt-group-syntax']+paths,text=True).splitlines()]
preserve_expr={('01_core/src/compiler/layout/'+f+'.rs',line) for f,line in [('helpers',260),('equation',450),('footnote_flush',300),('html_frame',193),('place',415),('slicing',177)]}|{('01_core/src/compiler/math/layout/mod.rs',543)}
preserve_pat={('01_core/src/compiler/layout/'+f+'.rs',line) for f,line in [('helpers',253),('equation',440),('footnote_flush',293),('html_frame',184),('place',408),('slicing',170)]}|{('01_core/src/compiler/math/layout/mod.rs',536)}
for p in paths:
 s=Path(p).read_text();lines=s.splitlines(True);starts=[0]
 for line in lines:starts.append(starts[-1]+len(line))
 edits=[]
 for row in rows:
  if row['path']!=p or 'paint_root' in row['fields']:continue
  key=(p,row['line']);text=None
  if row['kind']=='expr':text='paint_root,' if key in preserve_expr else 'paint_root: false,'
  elif key in preserve_pat:text='paint_root,'
  elif not row['rest']:text='paint_root: _,'
  if text:
   pos=starts[row['open_line']-1]+row['open_column']+1;assert s[pos-1]=='{';edits.append((pos,'\n'+text+'\n'))
 for i,text in sorted(edits,reverse=True):s=s[:i]+text+s[i:]
 if p.endswith('/layout_types.rs'):
  s=s.replace('    Group {\n        pos: Point,','    Group {\n        /// Establishes a local paint root; technical groups inherit when false.\n        paint_root: bool,\n        pos: Point,',1)
 if p.endswith('/layout/tests.rs'):
  s=s.replace('mod paint_root_transport {\n    use super::*;','mod paint_root_transport {\n    use super::*;\n    use crate::entities::layout_types::{Color, TransformMatrix};',1)
 Path(p).write_text(s)
subprocess.run(['rustfmt','--edition','2021','--config','skip_children=true']+paths,check=True)
print('Group field migrated; all new producer roots still false for behavioral RED')
