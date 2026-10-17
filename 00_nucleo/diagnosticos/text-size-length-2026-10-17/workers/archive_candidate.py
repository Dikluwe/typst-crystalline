from pathlib import Path
import datetime,json,gzip,hashlib,tarfile,tempfile,shutil
out=Path(__file__).resolve().parents[1];v=json.load(open(out/'validation-result.json'));assert v.get('complete') and v['sources_unchanged'];sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest();assert all(sha(p)==h for p,h in v['source_hashes'].items());assert not (out/'candidate-1.tar.gz').exists()
record=dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),classification='First materialization; functional Heading refuters open and two V17 require correction. This archive is not TS1 closure.',validation=v)
with gzip.open(out/'candidate-1-state.json.gz','wt') as f:json.dump(record,f)
with tempfile.TemporaryDirectory(prefix='ts1-candidate-',dir='/tmp') as temp:
 archive=Path(temp)/'candidate-1.tar.gz'
 with tarfile.open(archive,'w:gz',compresslevel=3) as t:
  for f in sorted(out.rglob('*')):
   if f.is_file():t.add(f,arcname='diagnostic/'+str(f.relative_to(out)),recursive=False)
  for p in v['source_hashes']:t.add(p,arcname='source-tree/'+p,recursive=False)
  t.add('target/debug/typst',arcname='binaries/typst',recursive=False)
 shutil.move(archive,out/'candidate-1.tar.gz')
(out/'candidate-1-archive.json').write_text(json.dumps(dict(utc=record['utc'],sha256=sha(out/'candidate-1.tar.gz'),source_files=len(v['source_hashes']),binary_sha256=sha('target/debug/typst')),indent=2)+'\n');print('Candidate archived with source tree, measurements and CLI binary')
