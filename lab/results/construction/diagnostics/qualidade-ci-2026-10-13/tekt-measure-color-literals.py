from pathlib import Path
import json,subprocess,gzip,datetime,hashlib
p=Path('01_core/src/entities/color/conversion.rs');raw=p.read_bytes();edits={}
for l in Path('00_nucleo/diagnosticos/correcoes-producao-2026-10-13/clippy-final.jsonl').read_text().splitlines():
 try:r=json.loads(l)
 except:continue
 if r.get('reason')!='compiler-message':continue
 m=r['message']
 if (m.get('code') or {}).get('code')!='clippy::excessive_precision':continue
 for c in m.get('children',[]):
  for s in c.get('spans',[]):
   if s['file_name']==str(p) and s.get('suggestion_applicability')=='MachineApplicable':edits[(s['byte_start'],s['byte_end'])]=s['suggested_replacement']
rows=[{'start':a,'end':b,'old':raw[a:b].decode(),'new':new} for (a,b),new in sorted(edits.items())]
proof=['fn main() {']
for i,e in enumerate(rows):proof.append(f'let before: f32 = {e["old"]}; let after: f32 = {e["new"]}; assert_eq!(before.to_bits(),after.to_bits(),"literal {i}");')
proof.append(f'println!("Verified {{}} f32 literal identities", {len(rows)}); }}')
Path('/tmp/tekt-color-literals.rs').write_text('\n'.join(proof))
subprocess.run(['rustc','--crate-name','tekt_color_literals','/tmp/tekt-color-literals.rs','-o','/tmp/tekt-color-literals'],check=True)
r=subprocess.run(['/tmp/tekt-color-literals'],check=True,capture_output=True,text=True)
out=Path('00_nucleo/diagnosticos/qualidade-ci-2026-10-13')
files=[str(p),'00_nucleo/prompts/entities/color/conversion.md'];git=lambda *a:subprocess.check_output(['git',*a],text=True)
data={'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'head':git('rev-parse','HEAD').strip(),'diffstat':git('diff','HEAD','--stat'),'status':git('status','--porcelain=v1','--untracked-files=all'),'files':{f:{'text':Path(f).read_text(),'sha256':hashlib.sha256(Path(f).read_bytes()).hexdigest()} for f in files},'candidates':rows,'proof': '\n'.join(proof),'stdout':r.stdout,'exit':r.returncode}
with gzip.open(out/'color-literals-before.json.gz','wt') as f:json.dump(data,f,ensure_ascii=False)
print(r.stdout)
