from pathlib import Path
import json,gzip,hashlib,base64,datetime,re,shutil
base=Path('00_nucleo/diagnosticos/transformacoes-2026-10-14');out=base/'logical-width'
sha=lambda b:hashlib.sha256(b).hexdigest()
v=out/'validation-final';pre=json.load(gzip.open(v/'pre-validation.json.gz'))
assert all(sha(Path(p).read_bytes())==x['sha256'] for p,x in pre['files'].items())
assert all(x['exit']==0 for x in json.loads((v/'followup-validation-results.json').read_text()))
suite=json.loads((out/'final-workspace/workspace-result.json').read_text());assert suite['exit']==0 and suite['sources_unchanged']
shutil.copyfile(base.with_suffix('.md'),out/'report-snapshot.md')
paths={p for p in out.rglob('*') if p.is_file()}
for name in ['logical-before-complete','logical-observation','logical-after','logical-spacing-after','after-logical-width','refuters-logical-width','margin-logical-width','pdf-logical-width','logical-final','logical-spacing-final','after-logical-final','refuters-logical-final','pdf-logical-final','logical-pdf-final']:
 paths.update(p for p in (base/name).rglob('*') if p.is_file())
paths.update(p for p in (base/'implementation').glob('logical-*') if p.is_file())
paths.add(base/'implementation/changed-sources.json')
for stem in ['transform','tests','line_commit','h_space']:
 source=Path('01_core/src/compiler/layout')/(stem+'.rs');owner=Path('00_nucleo/prompts/compiler/layout')/(stem+'.md');paths.update([source,owner])
 for n in re.findall(r'^- (00_nucleo/prompts/_nuclei/\S+) sha256:',owner.read_text(),re.M):paths.add(Path(n))
recipes=out/'recipes';recipes.mkdir(exist_ok=True)
for name in ['tekt-transform-logical-archive.py','tekt-transform-logical-pdf.py','tekt-transform-logical-pdf-text.py','tekt-transform-run.py','tekt-seal-owned.py','tekt-transform-logical-refuters.py','tekt-transform-logical-observe.py','tekt-transform-spacing-refuters.py','tekt-transform-spacing-final.py','tekt-transform-reprobe.py','tekt-transform-refuters.py','tekt-transform-margin-refuters.py','tekt-transform-pdf-refuters.py','tekt-transform-logical-suite.py','tekt-transform-logical-suite-final.py','tekt-transform-logical-validate.py','tekt-transform-logical-validate-final.py']:
 dest=recipes/name;shutil.copyfile(Path('/tmp')/name,dest);paths.add(dest)
for p in Path('/tmp').glob('tekt-transform-*logical*.log'):
 dest=out/p.name;shutil.copyfile(p,dest);paths.add(dest)
entries=[]
for p in sorted(paths):
 b=p.read_bytes();entries.append(dict(path=str(p),sha256=sha(b),base64=base64.b64encode(b).decode()))
archive=out/'evidence.json.gz';assert not archive.exists()
with gzip.open(archive,'wt') as f:json.dump(entries,f)
check=json.load(gzip.open(archive))
for e in check:
 b=base64.b64decode(e['base64']);assert sha(b)==e['sha256'] and b==Path(e['path']).read_bytes()
receipt=dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),archive=str(archive),sha256=sha(archive.read_bytes()),entries=len(check),bytes=archive.stat().st_size,reopened_and_compared=True)
(out/'receipt.json').write_text(json.dumps(receipt,indent=2));print(json.dumps(receipt),flush=True)
p=base.with_suffix('.md');p.write_text(p.read_text()+f"\nArquivo recuperável: `logical-width/evidence.json.gz`, {receipt['entries']} entradas\ncom path/SHA-256/base64, reabertas e comparadas. SHA-256 `{receipt['sha256']}`.\n")
