from pathlib import Path
import re,json,hashlib,datetime,subprocess
R=Path.cwd();H=R/'00_nucleo/diagnosticos/bidi-contexto-2026-10-05';rows=[]
for layer in ['01_core','02_shell','03_infra','04_wiring']:
 for p in (R/layer/'src').rglob('*.rs'):
  s=p.read_text();hits=[]
  for m in re.finditer(r'FrameItem::(TextSeparator|TextShaped|Text)\s*\{',s):
   hits.append({'variant':m[1],'line':s.count('\n',0,m.start())+1})
  if hits:
   owner=re.search(r'@prompt\s+([^\s]+)',s);owner=owner[1] if owner else None;rows.append({'path':str(p.relative_to(R)),'sha256':hashlib.sha256(p.read_bytes()).hexdigest(),'owner':owner,'sites':hits})
d={'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'head':subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),'diff_stat':subprocess.check_output(['git','diff','HEAD','--stat'],text=True),'method':'textual inventory of variant references; includes constructors, patterns, tests and comments; compiler/owner audit remains authoritative','files':rows};(H/'inventory.json').write_text(json.dumps(d,ensure_ascii=False,indent=2));(H/'inventory.py').write_text(Path(__file__).read_text());print('candidate files',len(rows),'variant reference sites',sum(len(x['sites']) for x in rows),'owners',len(set(x['owner'] for x in rows)));print('missing owners',[x['path'] for x in rows if not x['owner']])
