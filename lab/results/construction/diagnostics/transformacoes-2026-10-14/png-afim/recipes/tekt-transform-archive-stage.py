from pathlib import Path
import json,gzip,hashlib,base64,re,datetime,subprocess,shutil
root=Path.cwd();base=Path('00_nucleo/diagnosticos/transformacoes-2026-10-14');impl=base/'implementation'
result=json.loads((impl/'reflow-workspace/workspace-result.json').read_text())
assert result['exit']==0 and result['sources_unchanged']
for label in ['reflow-build-second','reflow-core','reflow-fmt','reflow-lineage','reflow-reverse']:
 r=json.load(gzip.open(impl/(label+'-result.json.gz'),'rt'));assert r['exit']==0 and r['source_guard'];assert all(hashlib.sha256(Path(p).read_bytes()).hexdigest()==h for p,h in r['files'].items())
paths=set(p for p in base.rglob('*') if p.is_file() and 'evidence' not in p.name and 'png-afim' not in p.parts)
for source in json.loads((impl/'changed-sources.json').read_text()):
 p=Path(source);paths.add(p);owner=Path(re.search(r'^//! @prompt (.+)$',p.read_text(),re.M)[1]);paths.add(owner)
paths.add(Path('00_nucleo/prompts/_nuclei/transform-semantics.toml'))
shutil.copyfile(base.with_suffix('.md'),impl/'report-before-png.md');paths.add(impl/'report-before-png.md')
recipes=impl/'recipes';recipes.mkdir(exist_ok=True)
for pattern in ['tekt-transform*.py','tekt-seal-owned.py']:
 for p in Path('/tmp').glob(pattern):
  d=recipes/p.name;shutil.copyfile(p,d);paths.add(d)
for p in Path('/tmp').glob('tekt-transform*.log'):
 d=impl/p.name
 if not d.exists():shutil.copyfile(p,d)
 paths.add(d)
sha=lambda b:hashlib.sha256(b).hexdigest()
entries=[]
for p in sorted(paths):
 data=p.read_bytes();entries.append(dict(path=str(p),sha256=sha(data),base64=base64.b64encode(data).decode()))
archive=base/'implementation-evidence-1.json.gz';assert not archive.exists()
with gzip.open(archive,'wt') as f:json.dump(entries,f)
with gzip.open(archive,'rt') as f:check=json.load(f)
for e in check:
 data=base64.b64decode(e['base64']);assert sha(data)==e['sha256'];assert data==Path(e['path']).read_bytes()
receipt=dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),archive=str(archive),sha256=sha(archive.read_bytes()),entries=len(check),bytes=archive.stat().st_size,reopened_and_compared=True)
(base/'implementation-evidence-1-receipt.json').write_text(json.dumps(receipt,indent=2));print(json.dumps(receipt),flush=True)
