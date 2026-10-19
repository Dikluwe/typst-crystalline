from pathlib import Path
import gzip,json,hashlib,datetime
out=Path(__file__).resolve().parents[1];sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest();state=json.load(gzip.open(out/'final-state.json.gz'));assert all(sha(p)==h for p,h in state['source_hashes'].items())
for name,expected in [('text-size-length-2026-10-17','771f5fd3d36545eb4b4fb7f0c0f2c91061470d1f010fc9a887b001a636d8feba'),('table-row-groups-2026-10-17','a47fe97272fd6a5bd96022be959ba277a317a74086f5e5f4a569bba008f4fae9')]:
 folder=Path('00_nucleo/materialization')/name;manifest=folder/'SHA256SUMS';assert sha(manifest)==expected
 for line in manifest.read_text().splitlines():
  digest,relative=line.split(maxsplit=1);path=Path(relative.strip().removeprefix('*'));path=path if path.exists() else folder/path;assert sha(path)==digest
exclude={'SHA256SUMS','delivery-check.json'};files=sorted(p for p in out.rglob('*') if p.is_file() and p.name not in exclude and '__pycache__' not in p.parts);(out/'SHA256SUMS').write_text(''.join(sha(p)+'  '+str(p.relative_to(out))+'\n' for p in files));digest=sha(out/'SHA256SUMS')
for line in (out/'SHA256SUMS').read_text().splitlines():h,p=line.split(maxsplit=1);assert sha(out/p)==h
result=dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),source_files_verified=len(state['source_hashes']),artifact_files_verified=len(files),manifest_sha256=digest,approved_ts1_and_pending_rg1_packages_preserved=True);(out/'delivery-check.json').write_text(json.dumps(result,indent=2)+'\n');print(result)
