from pathlib import Path
import datetime,gzip,hashlib,json,re,shutil,subprocess,tomllib
root=Path.cwd();out=Path(__file__).resolve().parents[1];v=json.load(open(out/'validation-result.json'));sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest();assert all(sha(p)==h for p,h in v['source_hashes'].items());assert json.load(open(out/'validation-resources.json'))['exit']==1;assert 'TektWhitespaceProbe' in (out/'validation-build.log').read_text();archive=out/'attempt-target-cache';archive.mkdir(exist_ok=False)
for p in list(out.iterdir()):
 if p.is_file() and p.name.startswith(('validation-','probe-baseline','probe-green')):shutil.move(str(p),archive/p.name)
for name in ['probe.py','measure_after.py','validate.py']:shutil.copyfile(out/'workers'/name,archive/name)
packages={tomllib.loads((root/top/'Cargo.toml').read_text())['package']['name'] for top in ['01_core','02_shell','03_infra','04_wiring','benches']};rust_names={p.replace('-','_') for p in packages};targets=['/tmp/tekt-whitespace-probe-target','/tmp/tekt-whitespace-measure-target'];seeds=[]
for target in targets:
 dst=Path(target);dst.mkdir(exist_ok=False);debug=dst/'debug';(debug/'deps').mkdir(parents=True);copied=0;size=0
 for p in (root/'target/debug/deps').iterdir():
  if not p.is_file() or p.suffix not in ['.rlib','.rmeta','.so','.d']:continue
  crate=p.stem.removeprefix('lib').rsplit('-',1)[0]
  if crate in rust_names:continue
  shutil.copy2(p,debug/'deps'/p.name);copied+=1;size+=p.stat().st_size
 for sub in ['.fingerprint','build']:
  (debug/sub).mkdir()
  for p in (root/'target/debug'/sub).iterdir():
   package=p.name.rsplit('-',1)[0]
   if package in packages:continue
   dest=debug/sub/p.name
   if p.is_dir():shutil.copytree(p,dest)
   elif p.is_file():shutil.copy2(p,dest)
 seeds.append(dict(target=target,external_artifact_files=copied,external_artifact_bytes=size,workspace_packages_excluded=sorted(packages)))
cmd=['cargo','clean','--offline','--locked','-p','typst-core','-p','typst-infra']
with (out/'cache-repair-clean.log').open('w') as f:r=subprocess.run(cmd,stdout=f,stderr=subprocess.STDOUT)
record=dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),reason='Disposable probes reused target/debug and produced a mutated core dependency. Real sources stayed unchanged; normal build detected the synthetic enum in cached metadata.',source_hashes=v['source_hashes'],sources_unchanged=all(sha(p)==h for p,h in v['source_hashes'].items()),seeds=seeds,clean_command=cmd,clean_exit=r.returncode,clean_scope='Generated typst-core/typst-infra artifacts only; external dependencies retained.',main_target=str(root/'target'))
(out/'target-isolation-repair.json').write_text(json.dumps(record,indent=2)+'\n');assert r.returncode==0 and record['sources_unchanged'];print(json.dumps({'cache_repaired':True,'targets':targets,'source_files_changed':0}),flush=True)
