from pathlib import Path
import datetime, difflib, gzip, hashlib, json, os, shutil, subprocess
root=Path.cwd();out=root/'lab/results/v16-corrections-2026-10-19/batch-a/metadata-control';out.mkdir(parents=True,exist_ok=True)
stage=root/'lab/scratch/v16-batch-a-metadata-control';stage.mkdir(parents=True,exist_ok=True)
now=lambda:datetime.datetime.now(datetime.timezone.utc).isoformat()
sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
for top in ['01_core','02_shell','03_infra','04_wiring','lab/benches','lab/corpus','lab/parity/corpus']:shutil.copytree(root/top,stage/top,dirs_exist_ok=True)
for name in ['Cargo.toml','Cargo.lock','clippy.toml','rustfmt.toml','rust-toolchain.toml','.cargo/config.toml']:
 p=stage/name;p.parent.mkdir(parents=True,exist_ok=True);shutil.copyfile(root/name,p)
path='03_infra/src/export/builder/metadata.rs';p=stage/path;original=p.read_text();start=original.index('                            typst_core::entities::layout_types::SemanticKind::Formula',original.index('fn collect_top('));end=original.index('=> {}',start)+len('=> {}')
assert original[start:end].count('SemanticKind::')==9
mutated=original[:start]+'                            _ => {}'+original[end:]
assert 'fn every_semantic_continuation_reaches_top_level_processing' in original
p.write_text(mutated)
record={'utc_start':now(),'head':subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),'diff_head_stat':subprocess.check_output(['git','diff','HEAD','--stat'],text=True),'kind':'counterfactual original wildcard at the selected metadata match; all other current sources and new tests unchanged','changed_path':path,'original_sha256':sha(root/path),'mutated_sha256':sha(p),'patch':''.join(difflib.unified_diff(original.splitlines(True),mutated.splitlines(True))), 'source_hashes':{str(p.relative_to(stage)):sha(p) for top in ['01_core','02_shell','03_infra','04_wiring'] for p in (stage/top).rglob('*.rs')},'lock_sha256':sha(stage/'Cargo.lock'),'fixture_hashes':{str(f.relative_to(stage)):sha(f) for top in ['lab/corpus','lab/parity/corpus'] for f in (stage/top).rglob('*') if f.is_file()}}
cmd=['cargo','test','--offline','--locked','-p','typst-infra','--lib','v16_semantic_continuation_tests','--','--test-threads=1'];record['command']=cmd
with gzip.open(out/'input.json.gz','wt') as f:json.dump({'record':record,'original':original,'mutated':mutated},f)
env=dict(os.environ,CARGO_BUILD_JOBS='1',CARGO_PROFILE_DEV_DEBUG='0',CARGO_PROFILE_TEST_DEBUG='0',CARGO_INCREMENTAL='0',CARGO_TARGET_DIR=str(root/'lab/target/product'),CRYSTALLINE_PDF_FIXED_EPOCH='1792281600')
with (out/'original-wildcard.log').open('w') as f:r=subprocess.run(cmd,cwd=stage,env=env,stdout=f,stderr=subprocess.STDOUT)
record.update(utc_end=now(),exit=r.returncode,productive_unchanged=sha(root/path)==record['original_sha256']);(out/'result.json').write_text(json.dumps(record,indent=2)+'\n')
print((out/'original-wildcard.log').read_text()[-3500:]);assert record['productive_unchanged'];assert r.returncode==0
