from pathlib import Path
import json,gzip,hashlib,datetime,subprocess,shutil,re
out=Path('00_nucleo/diagnosticos/tekt-v16-catalogos-2026-10-17');out.mkdir(exist_ok=False);(out/'workers').mkdir();prior=Path('00_nucleo/diagnosticos/text-styled-equality-2026-10-17');old=json.load(gzip.open(prior/'final-state.json.gz'));sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest();assert all(sha(p)==h for p,h in old['source_hashes'].items());sources=['01_core/src/compiler/eval/call_dispatch/type_fields.rs','01_core/src/compiler/stdlib/collections.rs'];owners={sources[0]:'00_nucleo/prompts/compiler/eval/call_dispatch/type_fields.md',sources[1]:'00_nucleo/prompts/compiler/stdlib/collections.md'}
b=dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),head=subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),hashes=old['source_hashes'],files={p:Path(p).read_text() for p in sources+list(owners.values())},status=subprocess.check_output(['git','status','--porcelain=v1'],text=True),diffstat=subprocess.check_output(['git','diff','HEAD','--stat'],text=True),linter=dict(path='/tmp/tekt-linter-none-target/debug/crystalline-lint',sha256=sha('/tmp/tekt-linter-none-target/debug/crystalline-lint')))
with gzip.open(out/'before.json.gz','wt') as f:json.dump(b,f)
(out/'scope.json').write_text(json.dumps(dict(sources=sources,owners=owners,mode='Private exhaustive domain dispatch; no runtime/API/default/phase change intended'),indent=2)+'\n');(out/'empty.typ').write_text('')
for name in ['run.py','strict.py','suite.py','rustdoc.py']:
 text=(prior/'workers'/name).read_text().replace('text-styled-equality-2026-10-17','tekt-v16-catalogos-2026-10-17').replace('TEKT_STYLED_EQ_MEMORY_LIMIT','TEKT_CATALOGUE_MEMORY_LIMIT');(out/'workers'/name).write_text(text)
shutil.copy('/tmp/bootstrap-v16-catalogues.py',out/'workers/bootstrap.py')
v=Path('01_core/src/entities/value.rs').read_text();enums={}
for name in ['Type','Value']:
 body=v.split('pub enum '+name+' {',1)[1].split('\n}',1)[0];names=[m.group(1) for line in body.splitlines() if (m:=re.match(r'^    (\w+)(?:\(|,)',line))];enums[name]=names
(out/'domain-enums.json').write_text(json.dumps(dict(source_sha256=sha('01_core/src/entities/value.rs'),enums=enums),indent=2)+'\n');print({n:len(xs) for n,xs in enums.items()})
