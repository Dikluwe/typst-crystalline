"""Archive final validation and source state without mutating earlier receipts."""
import collections,datetime,gzip,hashlib,json,subprocess
from pathlib import Path
H=Path(__file__).resolve().parent;R=H.parents[2];O=Path('/tmp/tekt-fixed-cost-2026-10-05');git=lambda *a:subprocess.check_output(['git',*a],cwd=R,text=True);sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
paired=json.load(gzip.open(H/'paired.json.gz','rt'));fs=[R/n for n in git('diff','--name-only').splitlines() if n.endswith(('.rs','.md','.toml'))]+[R/n for n in git('ls-files','--others','--exclude-standard').splitlines() if n.startswith(('01_core/','00_nucleo/prompts/'))]
assert all(sha(R/n)==s for n,s in paired['source_hashes'].items()),'source changed after benchmark'
assert sha(R/'target/release/typst')==paired['binaries']['candidate']['sha256']
logs={'coverage-before':('coverage-before.log','cargo test --offline -p typst-core --lib coverage_'),'coverage-after':('coverage-after.log','cargo test --offline -p typst-core --lib coverage_'),'core':('core.log','cargo test --offline -p typst-core --lib'),'infra':('infra.log','cargo test --offline -p typst-infra --lib'),'build':('build.log','cargo build --offline --release'),'probe-build':('probe-build.log','cargo build --offline --release --manifest-path 00_nucleo/diagnosticos/custo-fixo-2026-10-05/probe/Cargo.toml --target-dir target'),'compression-build':('compression-build.log','cargo build --offline --release --manifest-path 00_nucleo/diagnosticos/custo-fixo-2026-10-05/probe/Cargo.toml --target-dir target'),'lint':('lint.sarif','crystalline-lint --format sarif .'),'seal':('seal.log','python3 /tmp/tekt-fixed-cost-2026-10-05/seal.py'),'verification':('verification.log','python3 00_nucleo/diagnosticos/custo-fixo-2026-10-05/verify.py')}
r={'collected':datetime.datetime.now(datetime.timezone.utc).isoformat(),'head':git('rev-parse','HEAD').strip(),'status':git('status','--short'),'diff_stat':git('diff','HEAD','--stat'),'sources':{str(p.relative_to(R)):p.read_text() for p in fs},'source_hashes':{str(p.relative_to(R)):sha(p) for p in fs},'candidate_sha256':sha(R/'target/release/typst'),'benchmark_sources_and_binary_unchanged':True,'logs':{}}
for n,(f,c) in logs.items():
 p=O/f;r['logs'][n]={'command':c,'log_finished_utc':datetime.datetime.fromtimestamp(p.stat().st_mtime,datetime.timezone.utc).isoformat(),'sha256':sha(p),'output':p.read_text()}
p=Path('/tmp/tekt-realize-nucleus-gate.log');r['logs']['ownership']={'command':'crystalline-lint --checks v15,v26 .','output':p.read_text(),'sha256':sha(p),'log_finished_utc':datetime.datetime.fromtimestamp(p.stat().st_mtime,datetime.timezone.utc).isoformat()}
r['toolchain']={c:subprocess.check_output([c,'--version'],text=True).strip() for c in ['rustc','cargo']}
r['environment_font_config_hashes']={str(p):sha(p) for base in ['/etc/fonts','/home/dikluwe/.config/fontconfig'] for p in Path(base).rglob('*') if p.is_file()}
with gzip.open(H/'execution.json.gz','wt') as f:json.dump(r,f,ensure_ascii=False,indent=2)
print('Final execution sources match benchmark; receipts archived')
