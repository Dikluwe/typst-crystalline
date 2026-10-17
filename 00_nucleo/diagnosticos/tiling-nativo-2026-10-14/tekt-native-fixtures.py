import json,re,subprocess
from pathlib import Path
paths=set()
for l in Path('00_nucleo/diagnosticos/tiling-nativo-2026-10-14/tests-check-2.jsonl').read_text().splitlines():
 try:d=json.loads(l)
 except:continue
 if d.get('reason')=='compiler-message' and d['message']['level']=='error':
  for sp in d['message']['spans']:
   if sp['is_primary']:paths.add(sp['file_name'])
for path in sorted(paths):
 p=Path(path);s=p.read_text();m=re.search(r'@prompt ([^\n]+)',s)
 if m and 'Transporte preparado em Shape — T1c/T2' not in Path(m[1]).read_text():subprocess.run(['python3','/tmp/tekt-native-spec.py',path,'Migrar fixtures de FrameItem::Shape para FramePaint/FrameStroke, preservando valores e asserts semânticos existentes. Tiling deve construir célula preparada explícita; fixtures de avaliação continuam declarativas. Não adicionar conversão produtiva total nem enfraquecer asserções.'],check=True)
 # Only test portions for productive modules.
 cut=0
 if path not in ['03_infra/src/export/tests/mod.rs','03_infra/src/export/svg/tests.rs','03_infra/tests/p1250_svg_integrated.rs','03_infra/src/export/tests/p1483_pdf_spot_tests.rs']:
  cut=s.find('#[cfg(test)]\nmod ')
  if cut<0:
   print('manual',path);continue
 pre=s[:cut];t=s[cut:]
 t=t.replace('typst_core::entities::paint::Paint::','typst_core::entities::frame_paint::FramePaint::')
 t=t.replace('use typst_core::entities::paint::Paint;', 'use typst_core::entities::frame_paint::FramePaint as Paint;')
 t=t.replace('typst_core::entities::geometry::Stroke', 'typst_core::entities::frame_paint::FrameStroke')
 # Aliased standalone Stroke imports and grouped imports.
 t=t.replace('use typst_core::entities::frame_paint::FrameStroke;', 'use typst_core::entities::frame_paint::FrameStroke as Stroke;')
 def grouped(m):
  names=m[1]
  if not re.search(r'\bStroke\b',names):return m[0]
  names=re.sub(r'\bStroke,?\s*','',names)
  return 'use typst_core::entities::frame_paint::FrameStroke as Stroke;\nuse typst_core::entities::geometry::{'+names+'};'
 t=re.sub(r'use typst_core::entities::geometry::\{([^}]+)\};',grouped,t)
 t=t.replace('Paint::solid(', 'Paint::Solid(')
 p.write_text(pre+t)
