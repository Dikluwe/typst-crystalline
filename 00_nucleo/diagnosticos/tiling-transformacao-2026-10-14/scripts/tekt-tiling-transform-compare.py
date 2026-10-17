from pathlib import Path
import gzip,json,subprocess,datetime,hashlib,re
from PIL import Image
out=Path('00_nucleo/diagnosticos/tiling-transformacao-2026-10-14');r=dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),qpdf=[],controls=[])
for p in sorted(out.rglob('*.pdf')):
 x=subprocess.run(['qpdf','--check',str(p)],capture_output=True,text=True);r['qpdf'].append(dict(path=str(p),sha256=hashlib.sha256(p.read_bytes()).hexdigest(),exit=x.returncode,stdout=x.stdout,stderr=x.stderr));assert x.returncode==0
for name in ['plain','offset','quarter','oblique']:
 for ext in ['tekt.png','tekt-svg.png']:
  a=Image.open(out/'before'/(name+'-'+ext)).convert('RGB');b=Image.open(out/'after'/(name+'-'+ext)).convert('RGB');assert a.size==b.size and a.tobytes()==b.tobytes();r['controls'].append(dict(name=name,format=ext,unchanged_pixels=True))
with gzip.open(out/'before-state.json.gz','rt') as f:before=json.load(f)
norm=lambda s:re.sub(r'^//! @prompt-hash [^\n]+','',s,flags=re.M)
entity='01_core/src/entities/tiling.rs';stdlib='01_core/src/compiler/stdlib/visualize.rs'
assert norm(before['files'][stdlib]['text'])==norm(Path(stdlib).read_text())
assert norm(before['files'][entity]['text']).replace('/// Rotação horária aplicada depois do offset.','/// Rotação horária da grade em torno do ponto offset.')==norm(Path(entity).read_text())
r.update(stdlib_only_lineage=True,entity_only_documentation_and_lineage=True)
with gzip.open(out/'final-comparison.json.gz','wt') as f:json.dump(r,f)
print('qpdf',len(r['qpdf']),'unchanged controls',len(r['controls']))
# Inventory for the unresolved cross-layer tile transport. Textual hits are
# candidates for review, not a count of required changes or approved design.
files=subprocess.check_output(['rg','-l','FrameItem::Shape|Paint::Tiling','01_core/src','03_infra/src','02_shell/src','04_wiring/src'],text=True).splitlines()
inv=dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),head=subprocess.check_output(['git','rev-parse','HEAD'],text=True),candidate_only=True,files={})
for file in files:
 p=Path(file);text=p.read_text();m=re.search(r'^//! @prompt (.+)$',text,re.M);owner=m.group(1) if m else None
 inv['files'][file]=dict(sha256=hashlib.sha256(p.read_bytes()).hexdigest(),owner=owner,owner_sha256=hashlib.sha256(Path(owner).read_bytes()).hexdigest() if owner else None,hits=[dict(line=i,text=line) for i,line in enumerate(text.splitlines(),1) if 'FrameItem::Shape' in line or 'Paint::Tiling' in line])
with gzip.open(out/'carrier-candidate-inventory.json.gz','wt') as f:json.dump(inv,f)
