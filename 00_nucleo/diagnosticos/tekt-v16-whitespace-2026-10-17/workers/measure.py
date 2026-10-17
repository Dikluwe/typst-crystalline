from pathlib import Path
import datetime,gzip,hashlib,json,os,shutil,subprocess,tempfile
root=Path.cwd();out=Path(__file__).resolve().parents[1];b=json.load(gzip.open(out/'before.json.gz'));sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest();assert all(sha(p)==h for p,h in b['hashes'].items());source=b['sources'][0];addition=(out/'workers/measure_test.rs.txt').read_text();record=dict(start=datetime.datetime.now(datetime.timezone.utc).isoformat(),head=b['head'],source_hashes=b['hashes'],original_owner_sha256=sha(source),added_measurement=addition,scope='Original module compiled within the actual crate in a disposable workspace. No copied or substituted Source helper.')
with tempfile.TemporaryDirectory(prefix='tekt-whitespace-measure-') as directory:
 stage=Path(directory)
 for top in ['01_core','02_shell','03_infra','04_wiring','benches']:shutil.copytree(root/top,stage/top,ignore=shutil.ignore_patterns('target','.git','__pycache__'))
 for name in ['Cargo.toml','Cargo.lock','clippy.toml','rustfmt.toml','rust-toolchain.toml','.cargo/config.toml']:
  dst=stage/name;dst.parent.mkdir(parents=True,exist_ok=True);shutil.copyfile(root/name,dst)
 (stage/source).write_text(Path(source).read_text()+addition);cmd=['cargo','test','--offline','--locked','-p','typst-core','--lib','v16_whitespace_measure','--','--test-threads=1','--nocapture'];record.update(command=cmd,workspace=str(stage),measured_source=(stage/source).read_text());env=dict(os.environ,CARGO_TARGET_DIR=str(root/'target'),CARGO_BUILD_JOBS='1')
 with (out/'measurement-before.log').open('w') as f:r=subprocess.run(cmd,cwd=stage,env=env,stdout=f,stderr=subprocess.STDOUT)
 record.update(exit=r.returncode,end=datetime.datetime.now(datetime.timezone.utc).isoformat(),sources_unchanged=all(sha(p)==h for p,h in b['hashes'].items()))
 with gzip.open(out/'source-before.json.gz','wt') as f:json.dump(record,f)
 assert r.returncode==0 and record['sources_unchanged']
lines=[]
for line in (out/'measurement-before.log').read_text().splitlines():
 if 'source=false' in line or 'source=true' in line:lines.append(line.removeprefix('test compiler::eval::rules::textual::space::v16_measure::v16_whitespace_measure ... '))
assert len(lines)==14,lines
pairs=[dict(case=a.split()[0],plain=a,source=z,same=a.replace('source=false','source=*')==z.replace('source=true','source=*')) for a,z in zip(lines[::2],lines[1::2])];(out/'source-before-analysis.json').write_text(json.dumps(pairs,indent=2)+'\n');print(json.dumps(pairs,indent=2),flush=True)
