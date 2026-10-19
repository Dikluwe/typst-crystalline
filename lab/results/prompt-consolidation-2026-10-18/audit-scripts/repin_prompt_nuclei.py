from pathlib import Path
import hashlib,json,re,datetime,tomllib
root=Path('/repos/Antigravity/typst-crystalline');out=root/'lab/results/prompt-consolidation-2026-10-18';changes=[]
for p in sorted((root/'00_nucleo/prompts').rglob('*.md')):
 s=p.read_text()
 def pin(m):
  rel=m.group(1); nucleus=(root/rel).read_bytes(); assert not tomllib.loads(nucleus.decode()).get('dependencies'), 'Dependency framing must be extended for nonempty DAG edges'; digest=hashlib.sha256(nucleus+b'\x00TEKT-NUCLEUS-DEPS-V1\x00').hexdigest()
  if m.group(2)!=digest:changes.append({'owner':str(p.relative_to(root)),'nucleus':rel,'old':m.group(2),'new':digest})
  return '- '+rel+' sha256:'+digest
 t=re.sub(r'^- (00_nucleo/prompts/_nuclei/[^ ]+) sha256:([0-9a-f]{64})$',pin,s,flags=re.M)
 if s!=t:p.write_text(t)
(out/'repin-receipt.json').write_text(json.dumps({'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'changes':changes,'changed_pins':len(changes),'owners':len({e['owner'] for e in changes})},indent=2)+'\n');print('repinned',len(changes),'edges across',len({e['owner'] for e in changes}),'owners')
