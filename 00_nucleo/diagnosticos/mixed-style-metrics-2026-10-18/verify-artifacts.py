from pathlib import Path
import json,gzip,hashlib,datetime
p=Path(__file__).resolve().parent
read=lambda n:json.loads(gzip.decompress((p/(n+'.json.gz')).read_bytes()))
corpus=read('corpus-after');by={c['label']:c for c in corpus['cases']}
checked=[];recaptures=[]
for label in ['after','bib-after','regions-after','corpus-after','corpus-extra-after','corpus-boundary-after','corpus-repeat-after','controls-after','controls-extra-after','deco-after','deco-empty-after']:
 d=read(label)
 for c in d['cases']:
  for actor,data in c['results'].items():
   variants=data.values() if 'pdf' in data else [data]
   for result in variants:
    if result['exit']:continue
    file=Path(result['command'][-1]);expected=result.get('sha256',result.get('artifact_sha256'))
    if expected is None:continue
    with file.open('rb') as f:actual=hashlib.file_digest(f,'sha256').hexdigest()
    if actual!=expected:
     # These exact six cases are deliberately repeated later by the legacy
     # corpus at the same output path. Oracle PDF timestamps are not frozen
     # by the Tekt-specific environment variable. Retain both observations,
     # and certify the surviving bytes against their actual later capture.
     assert label=='bib-after' and actor=='vanilla' and file.suffix=='.pdf'
     later=by[c['label']];replacement=later['results'][actor]
     assert c['source_sha256']==later['source_sha256']
     assert d['binaries'][actor]['sha256']==corpus['binaries'][actor]['sha256']
     assert result['command']==replacement['command']
     assert result['pages']==replacement['pages']
     assert actual==replacement['artifact_sha256']
     recaptures.append(dict(case=c['label'],original_collection=label,original_sha256=expected,later_collection='corpus-after',later_sha256=actual,source_sha256=c['source_sha256'],same_source_command_binary_and_exact_geometry=True,original_raw_pdf_retained=False))
    checked.append(dict(collection=label,case=c['label'],actor=actor,path=str(file),recorded_sha256=expected,surviving_sha256=actual))
assert len(recaptures)==6
(p/'artifact-verification.json.gz').write_bytes(gzip.compress(json.dumps(dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),observations=len(checked),files=len(set(x['path'] for x in checked)),recaptures=recaptures,checked=checked),indent=2).encode()))
print('artifact observations verified',len(checked),'unique files',len(set(x['path'] for x in checked)),'recaptured identical-geometry oracle PDFs',len(recaptures))
