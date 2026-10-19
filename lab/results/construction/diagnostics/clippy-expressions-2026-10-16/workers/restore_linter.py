from pathlib import Path
import subprocess,tarfile,io,os,json,hashlib,datetime
repo='/repos/Antigravity/tekt-linter';pin='49c6bb0e584104a0c89270431d8402f5c6c61150';src=Path('/tmp/tekt-linter-ci-src');target=Path('/tmp/tekt-linter-ci-target');out=Path('00_nucleo/diagnosticos/clippy-expressions-2026-10-16')
assert not src.exists();src.mkdir()
archive=subprocess.check_output(['git','-C',repo,'archive',pin])
with tarfile.open(fileobj=io.BytesIO(archive)) as f:f.extractall(src,filter='data')
env=dict(os.environ,CARGO_TARGET_DIR=str(target),CARGO_BUILD_JOBS='1')
subprocess.run(['cargo','build','--offline','--locked','--manifest-path',str(src/'Cargo.toml'),'--bin','crystalline-lint'],env=env,check=True)
binary=target/'debug/crystalline-lint';record=dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),pin=pin,tree=subprocess.check_output(['git','-C',repo,'rev-parse',pin+'^{tree}'],text=True).strip(),archive_sha256=hashlib.sha256(archive).hexdigest(),binary=str(binary),binary_sha256=hashlib.sha256(binary.read_bytes()).hexdigest())
(out/'linter-provenance.json').write_text(json.dumps(record,indent=2)+'\n');print(json.dumps(record))
