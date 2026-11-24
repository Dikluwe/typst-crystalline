from pathlib import Path
import subprocess, sys, json, hashlib, datetime, runpy, shutil
root=Path('/repos/Antigravity/typst-crystalline')
d=root/'lab/experiments/context-substitution-preservation-2026-11-21'
out=root/'lab/results/construction/diagnostics/context-substitution-preservation-2026-11-21'
s=Path(sys.argv[1]); s.mkdir(parents=True,exist_ok=True)
subprocess.run(['rsync','-a',*[str(root/x) for x in ['.cargo','Cargo.toml','Cargo.lock','rust-toolchain.toml','crystalline.toml','00_nucleo','01_core','02_shell','03_infra','04_wiring']],str(s)+'/'],check=True)
(s/'lab').mkdir(exist_ok=True)
subprocess.run(['rsync','-a',str(root/'lab/benches'),str(s/'lab')+'/'],check=True)
runpy.run_path(str(d.parent/'references-500-visible-textual-style-collapse-2026-11-21/prototype.py'))
files={str(p.relative_to(s)):hashlib.sha256(p.read_bytes()).hexdigest() for area in ['01_core','02_shell','03_infra','04_wiring'] for p in (s/area).rglob('*') if p.is_file()}
prior={line.split('  ',1)[1]:line.split('  ',1)[0] for line in (d.parent/'references-500-visible-textual-style-collapse-2026-11-21/candidate-input-manifest.sha256').read_text().splitlines()}
diff=[k for k,v in files.items() if prior.get(k)!=v]
assert not diff,diff
(out/'baseline-verification.json').write_text(json.dumps({'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'files':len(files),'differences':diff,'head':subprocess.check_output(['git','rev-parse','HEAD'],cwd=root,text=True).strip()},indent=2))
subprocess.run(['git','diff','HEAD','--stat'],cwd=root,stdout=(out/'diffstat-before.txt').open('w'),check=True)
shutil.copy(s/'03_infra/src/pipeline.rs',d/'baseline-pipeline.rs')
p=s/'00_nucleo/prompts/infra/pipeline.md'
p.write_text(p.read_text()+'''\n\n## Laboratory bounded inert substitution — 2026-11-21\nEvidence: pipeline.rs:928 rebuilds Source/Located/Realized/ShowRecipe/Styled and Sequence before discovering whether replacement exists; the 2026-11-21 pre-PDF profile attributes about15ms to substitution in references_500. Existing heading-only certificate demonstrates the ownership boundary, not universal safety.\nExperiment: before reconstruction, a private exhaustive structural proof may preserve an immutable subtree that contains only Text/Empty/Space/SpaceFromNewline/Parbreak, Sequence, Strong/Emph and Source/Located/Realized visual/ShowRecipe/ordinary Styled wrappers. Reject prepared-reference transports, all actual ContextBlocks, all other variants and exceeded bounds (depth32,nodes512). Inert means substitution identity, not absence of semantic effects; retain the exact incoming value and every original/style/span field. No memo across calls, no callback or snapshot reuse, no generation or pipeline changes. Fallback remains byte-for-byte original. Tests require same Arc for unchanged subtree, unchanged original after two differing contextual replacements, hidden Realized.original exclusion, rejected prepared references/unsupported containers and bounds. Census is separate from nominal CPU; retain only with correct output and measured benefit. Lab only, no integration.\n''')
h=hashlib.sha256(p.read_bytes()).hexdigest(); (out/'l0-seal.json').write_text(json.dumps({'path':str(p),'sha256':h,'utc':datetime.datetime.now(datetime.timezone.utc).isoformat()},indent=2))
# Tests are appended before implementation; functional RED must fail preservation.
with (s/'03_infra/src/pipeline.rs').open('a') as f: f.write((d/'tests.rs').read_text())
