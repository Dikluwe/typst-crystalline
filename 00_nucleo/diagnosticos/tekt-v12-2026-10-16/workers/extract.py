from pathlib import Path
import subprocess,json,hashlib,datetime
out=Path('00_nucleo/diagnosticos/tekt-v12-2026-10-16');cache=Path('/tmp/tekt-linter-ci-target/debug/deps');binary=Path('/tmp/tekt-v12-items');cmd=['rustc','--edition=2021','--crate-name','v12_items','-L','dependency='+str(cache)]
for n in ['tree_sitter','tree_sitter_rust','serde_json']:
 files=list(cache.glob('lib'+n+'-*.rlib'));assert len(files)==1;cmd+=['--extern',n+'='+str(files[0])]
cmd += [str(out/'workers/items.rs'),'-o',str(binary)];r=subprocess.run(cmd,capture_output=True,text=True);assert r.returncode==0,r.stderr
sources=[p for p in json.loads((out/'scope.json').read_text())['sources'] if '/tests/' in p]
cmd2=[str(binary),*sources];r=subprocess.run(cmd2,capture_output=True,text=True);assert r.returncode==0,r.stderr;(out/'items.json').write_text(r.stdout)
(out/'items-provenance.json').write_text(json.dumps(dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),compile=cmd,command=cmd2,helper_sha256=hashlib.sha256((out/'workers/items.rs').read_bytes()).hexdigest(),binary_sha256=hashlib.sha256(binary.read_bytes()).hexdigest()),indent=2)+'\n');print(len(json.loads(r.stdout)),'itens AST')
