"""Final pure reference control rebuild with complete prospective provenance."""
import base64, datetime, hashlib, json, pathlib, shutil, subprocess
root = pathlib.Path(__file__).resolve().parents[2]
area = pathlib.Path('/tmp/p1339-mutants.oPfqdA')
source = area/'reference'
target = area/'target-pure-control'
output = root/'00_nucleo/diagnosticos/p1339-mutant-build-pure-control-r1.json'
assert not output.exists()
sha = lambda p: hashlib.sha256(pathlib.Path(p).read_bytes()).hexdigest()
utc = lambda: datetime.datetime.now(datetime.timezone.utc).isoformat()
argv = ['cargo','build','--manifest-path',str(source/'Cargo.toml'),'-p','typst-cli','--bin','typst','--release','--locked','--offline','--target-dir',str(target),'--config','profile.release.lto=false','--config','profile.release.codegen-units=16','-j','3']
files = sorted(p for p in source.rglob('*') if p.is_file() and (p.suffix == '.rs' or p.name in ('Cargo.toml','Cargo.lock')))
record = {'authority_manifest_sha256':'842d6526739014022c073800148a47b3c886831e2198ab65bbfdbe73ce59411b','start':utc(),'argv':argv,'cwd':str(root),'source_root':str(source),'source_sha256':{str(p.relative_to(source)):sha(p) for p in files},'runner_sha256':sha(__file__),'purpose':'Successor to warmup build whose initial UTC was not recorded; isolated fresh target, no source mutations.'}
p = subprocess.run(argv,cwd=root,capture_output=True,timeout=2700)
record.update(end=utc(),exit=p.returncode)
for channel in ('stdout','stderr'):
    raw = getattr(p,channel)
    record[channel]=raw.decode(errors='replace')
    record[channel+'_base64']=base64.b64encode(raw).decode()
if p.returncode == 0:
    binary=area/'binaries/typst-pure-control-r1'
    assert not binary.exists()
    shutil.copy2(target/'release/typst',binary)
    record['binary']={'path':str(binary),'sha256':sha(binary)}
with output.open('x') as f:
    json.dump(record,f,ensure_ascii=False,indent=2)
    f.write('\n')
print(json.dumps({'output':str(output),'exit':p.returncode,'binary':record.get('binary')}))
